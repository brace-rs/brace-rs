use actix_web::dev::Payload;
use actix_web::error::Error;
use actix_web::middleware::identity::Identity;
use actix_web::web::Data;
use actix_web::{FromRequest, HttpRequest};
use chrono::{DateTime, Duration, Local, Utc};
use futures::future::{ok, Either, Future, FutureResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::util::decode_token;

type BoxedFuture<I, E> = Box<Future<Item = I, Error = E>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    #[serde(with = "serde_datetime_utc")]
    pub created: DateTime<Utc>,
    #[serde(with = "serde_datetime_utc")]
    pub updated: DateTime<Utc>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            email: "".to_string(),
            password: "".to_string(),
            created: Utc::now(),
            updated: Utc::now(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserAuth {
    pub email: String,
    pub password: String,
}

impl Default for UserAuth {
    fn default() -> Self {
        Self {
            email: "".to_string(),
            password: "".to_string(),
        }
    }
}

pub enum CurrentUser {
    Anonymous,
    Authenticated(User),
}

impl<P> FromRequest<P> for CurrentUser {
    type Error = Error;
    type Future = Either<FutureResult<Self, Self::Error>, BoxedFuture<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload<P>) -> Self::Future {
        if let Ok(id) = Identity::from_request(req, payload) {
            if let Some(user) = id.identity() {
                if let Ok(uuid) = user.parse::<Uuid>() {
                    if let Ok(database) = Data::from_request(req, payload) {
                        return Either::B(Box::new(
                            crate::action::retrieve::retrieve(&database, uuid).then(move |user| {
                                match user {
                                    Ok(user) => ok(CurrentUser::Authenticated(user)),
                                    Err(_) => {
                                        id.forget();
                                        ok(CurrentUser::Anonymous)
                                    }
                                }
                            }),
                        ));
                    }
                }
            }

            id.forget();
        }

        Either::A(ok(CurrentUser::Anonymous))
    }
}

pub enum CurrentAuth {
    Unauthenticated,
    Authenticated(User),
}

impl<P> FromRequest<P> for CurrentAuth {
    type Error = Error;
    type Future = Either<FutureResult<Self, Self::Error>, BoxedFuture<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload<P>) -> Self::Future {
        if let Some(header) = req.headers().get("Authorization") {
            if let Ok(header) = header.to_str() {
                let parts = header.split(' ').collect::<Vec<&str>>();

                if parts.len() == 2 && parts[0] == "Bearer" {
                    if let Ok(auth) = decode_token(parts[1]) {
                        if let Ok(database) = Data::from_request(req, payload) {
                            return Either::B(Box::new(
                                crate::action::locate::locate(&database, auth.email).then(
                                    move |res| match res {
                                        Ok(user) => ok(CurrentAuth::Authenticated(user)),
                                        Err(_) => ok(CurrentAuth::Unauthenticated),
                                    },
                                ),
                            ));
                        }
                    }
                }
            }
        }

        Either::A(ok(CurrentAuth::Unauthenticated))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SlimUser {
    pub email: String,
}

impl From<Claims> for SlimUser {
    fn from(claims: Claims) -> Self {
        SlimUser {
            email: claims.email,
        }
    }
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser { email: user.email }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    pub email: String,
    pub iss: String,
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
}

impl Claims {
    pub fn with_email(email: &str) -> Self {
        Claims {
            iss: "localhost".into(),
            sub: "auth".into(),
            email: email.to_owned(),
            iat: Local::now().timestamp(),
            exp: (Local::now() + Duration::hours(24)).timestamp(),
        }
    }
}

mod serde_datetime_utc {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::de::Error;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(
        datetime: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        datetime
            .format("%Y-%m-%dT%H:%M")
            .to_string()
            .serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error> {
        let datetime: String = Deserialize::deserialize(deserializer)?;

        NaiveDateTime::parse_from_str(&datetime, "%Y-%m-%dT%H:%M")
            .map_err(Error::custom)
            .map(|datetime| DateTime::from_utc(datetime, Utc))
    }
}

use actix_web::dev::ServiceFromRequest;
use actix_web::error::{Error, ErrorInternalServerError};
use actix_web::middleware::identity::Identity;
use actix_web::web::Data;
use actix_web::FromRequest;
use chrono::{DateTime, Utc};
use futures::future::{ok, Either, Future, FutureResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

    fn from_request(req: &mut ServiceFromRequest<P>) -> Self::Future {
        match Identity::from_request(req) {
            Ok(id) => match id.identity() {
                Some(user) => match user.parse::<Uuid>() {
                    Ok(uuid) => match Data::from_request(req) {
                        Ok(database) => Either::B(Box::new(
                            crate::action::retrieve::retrieve(&database, uuid)
                                .map_err(ErrorInternalServerError)
                                .and_then(move |user| ok(CurrentUser::Authenticated(user))),
                        )),
                        Err(_) => Either::A(ok(CurrentUser::Anonymous)),
                    },
                    Err(_) => Either::A(ok(CurrentUser::Anonymous)),
                },
                None => Either::A(ok(CurrentUser::Anonymous)),
            },
            Err(_) => Either::A(ok(CurrentUser::Anonymous)),
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

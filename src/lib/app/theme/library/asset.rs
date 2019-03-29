use std::ops::Deref;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum AssetInfo {
    #[serde(rename = "css")]
    StyleSheet(StyleSheetInfo),
    #[serde(rename = "js")]
    JavaScript(JavaScriptInfo),
}

impl AssetInfo {
    pub fn name(&self) -> &str {
        match self {
            AssetInfo::StyleSheet(ref info) => &info.name,
            AssetInfo::JavaScript(ref info) => &info.name,
        }
    }

    pub fn location(&self) -> &Location {
        match self {
            AssetInfo::StyleSheet(ref info) => &info.location,
            AssetInfo::JavaScript(ref info) => &info.location,
        }
    }

    pub fn as_stylesheet(&self) -> Option<&StyleSheetInfo> {
        match self {
            AssetInfo::StyleSheet(ref info) => Some(info),
            _ => None,
        }
    }

    pub fn is_stylesheet(&self) -> bool {
        match self {
            AssetInfo::StyleSheet(_) => true,
            AssetInfo::JavaScript(_) => false,
        }
    }

    pub fn as_javascript(&self) -> Option<&JavaScriptInfo> {
        match self {
            AssetInfo::JavaScript(ref info) => Some(info),
            _ => None,
        }
    }

    pub fn is_javascript(&self) -> bool {
        match self {
            AssetInfo::StyleSheet(_) => false,
            AssetInfo::JavaScript(_) => true,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StyleSheetInfo {
    pub name: String,
    pub location: Location,
}

impl StyleSheetInfo {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &Location {
        &self.location
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct JavaScriptInfo {
    pub name: String,
    pub location: Location,
}

impl JavaScriptInfo {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(transparent)]
pub struct Location(pub String);

impl Location {
    pub fn new<S: Into<String>>(location: S) -> Self {
        Self(location.into())
    }

    pub fn as_url(&self) -> Option<Url> {
        match Url::parse(&self.0) {
            Ok(url) => Some(url),
            Err(_) => None,
        }
    }

    pub fn is_url(&self) -> bool {
        self.as_url().is_some()
    }

    pub fn as_path(&self) -> Option<&Path> {
        match self.as_url() {
            Some(_) => None,
            None => Some(Path::new(&self.0)),
        }
    }

    pub fn is_path(&self) -> bool {
        self.as_path().is_some()
    }

    pub fn is_external(&self) -> bool {
        match self.as_url() {
            Some(url) => url.scheme() != "file",
            None => false,
        }
    }

    pub fn is_internal(&self) -> bool {
        !self.is_external()
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Deref for Location {
    type Target = String;

    fn deref(&self) -> &String {
        &self.0
    }
}

impl From<PathBuf> for Location {
    fn from(path: PathBuf) -> Self {
        Location(path.to_string_lossy().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::Location;

    #[test]
    fn test_location_info() {
        let a = Location::new("foo/bar.txt");
        let b = Location::new("/foo/bar.txt");
        let c = Location::new("file://foo/bar.txt");
        let d = Location::new("http://website.com/foo/bar.txt");

        assert!(a.is_path());
        assert!(a.is_internal());
        assert!(!a.is_url());
        assert!(!a.is_external());

        assert!(b.is_path());
        assert!(b.is_internal());
        assert!(!b.is_url());
        assert!(!b.is_external());

        assert!(!c.is_path());
        assert!(c.is_internal());
        assert!(c.is_url());
        assert!(!c.is_external());

        assert!(!d.is_path());
        assert!(!d.is_internal());
        assert!(d.is_url());
        assert!(d.is_external());
    }
}

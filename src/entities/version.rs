use super::*;

impl Entity<'_> for Version {}

#[derive(Clone, Eq, PartialEq)]
pub struct Version {
    full: String,
    minor: u64,
}

impl Version {
    pub fn new(minor: u64) -> Self {
        Self {
            full: format!("1.{}", minor),
            minor,
        }
    }
}

impl Debug for Version {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.debug_tuple("Version").field(&self.full).finish()
    }
}

impl Display for Version {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.full, formatter)
    }
}

impl AsRef<str> for Version {
    fn as_ref(&self) -> &str {
        self.full.as_ref()
    }
}

impl Default for Version {
    fn default() -> Self {
        Self {
            full: "1.0".into(),
            minor: 0,
        }
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.minor.cmp(&other.minor)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Version {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: make static
        let regex = regex::Regex::new(r"^1\.(\d+)$").unwrap();
        let captures = regex.captures(s).ok_or(())?;
        let minor: u64 = captures[1].parse().or(Err(()))?;
        Ok(Self::new(minor))
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.full)
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MyVisitor;

        impl<'de> Visitor<'de> for MyVisitor {
            type Value = Version;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("JSON API version")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                value.parse().map_err(|_| {
                    serde::de::Error::custom("invalid JSON API version")
                })
            }
        }

        deserializer.deserialize_str(MyVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    #[test]
    fn clone() {
        assert_eq!(Version::default().clone(), Version::default());
        assert_eq!(Version::new(123).clone(), Version::new(123));
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", Version::default()), "Version(\"1.0\")");
        assert_eq!(format!("{:?}", Version::new(123)), "Version(\"1.123\")");
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", Version::default()), "1.0");
        assert_eq!(format!("{}", Version::new(123)), "1.123");
        assert_eq!(format!("{:<5}", Version::default()), "1.0  ");
        assert_eq!(format!("{:_>6}", Version::default()), "___1.0");
    }

    #[test]
    fn to_string() {
        assert_eq!(Version::default().to_string(), "1.0");
        assert_eq!(Version::new(123).to_string(), "1.123");
    }

    #[test]
    fn as_ref_str() {
        let version = Version::default();
        let version_str: &str = version.as_ref();
        assert_eq!(version_str, "1.0");

        let version = Version::new(123);
        let version_str: &str = version.as_ref();
        assert_eq!(version_str, "1.123");
    }

    #[test]
    fn default() {
        let version = Version::default();

        assert_eq!(version, Version::new(0));
        assert_eq!(version.full, "1.0");
        assert_eq!(version.minor, 0);
    }

    #[test]
    fn equality() {
        assert_eq!(Version::new(0), Version::new(0));
        assert_eq!(Version::new(1), Version::new(1));
        assert_eq!(Version::new(2), Version::new(2));
        assert_eq!(Version::new(1234), Version::new(1234));

        assert_ne!(Version::new(0), Version::new(1));
        assert_ne!(Version::new(0), Version::new(2));
        assert_ne!(Version::new(0), Version::new(1234));

        assert_ne!(Version::new(1), Version::new(0));
        assert_ne!(Version::new(1), Version::new(2));
        assert_ne!(Version::new(1), Version::new(1234));

        assert_ne!(Version::new(2), Version::new(0));
        assert_ne!(Version::new(2), Version::new(1));
        assert_ne!(Version::new(2), Version::new(1234));

        assert_ne!(Version::new(1234), Version::new(0));
        assert_ne!(Version::new(1234), Version::new(1));
        assert_ne!(Version::new(1234), Version::new(2));
    }

    #[test]
    fn order() {
        assert!(!(Version::new(0) < Version::new(0)));
        assert!(Version::new(0) <= Version::new(0));

        assert!(Version::new(0) < Version::new(1));
        assert!(Version::new(0) <= Version::new(1));

        assert!(Version::new(0) < Version::new(2));
        assert!(Version::new(0) <= Version::new(2));

        assert!(!(Version::new(1) < Version::new(0)));
        assert!(!(Version::new(1) <= Version::new(0)));

        assert!(!(Version::new(1) < Version::new(1)));
        assert!(Version::new(1) <= Version::new(1));

        assert!(Version::new(1) < Version::new(2));
        assert!(Version::new(1) <= Version::new(2));
    }

    #[test]
    fn parse() {
        let version: Version = "1.0".parse().unwrap();

        assert_eq!(version.full, "1.0");
        assert_eq!(version.minor, 0);

        let version: Version = "1.1".parse().unwrap();

        assert_eq!(version.full, "1.1");
        assert_eq!(version.minor, 1);

        let version: Version = "1.2".parse().unwrap();

        assert_eq!(version.full, "1.2");
        assert_eq!(version.minor, 2);
    }

    #[test]
    fn serialize_and_deserialize() {
        let version = Version::default();
        let json = serde_json::to_string(&version).unwrap();
        let deserialized: Version = serde_json::from_str(&json).unwrap();
        assert_eq!(version, deserialized);
    }

    #[test]
    fn serialize() {
        let version = Version::default();
        let json = serde_json::to_string(&version).unwrap();
        let value: Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value, json!("1.0"));

        let version = Version::new(123);
        let json = serde_json::to_string(&version).unwrap();
        let value: Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value, json!("1.123"));
    }

    #[test]
    fn deserialize() {
        let value = json!("1.0");
        let json = serde_json::to_string(&value).unwrap();
        let version: Version = serde_json::from_str(&json).unwrap();
        assert_eq!(version, Version::default());

        let value = json!("1.123");
        let json = serde_json::to_string(&value).unwrap();
        let version: Version = serde_json::from_str(&json).unwrap();
        assert_eq!(version, Version::new(123));
    }
}

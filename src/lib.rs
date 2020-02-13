#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;

use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum Trust {
    Public,
    Authenticated,
    Vouched,
    Ndaed,
    Staff,
}

#[derive(Fail, Debug)]
pub enum TrustError {
    #[fail(display = "Invalid trust level")]
    InvalidTrustLevel,
    #[fail(display = "More trust required")]
    TrustLevelToLow,
}

impl TryFrom<&str> for Trust {
    type Error = TrustError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "staff" => Ok(Trust::Staff),
            "ndaed" => Ok(Trust::Ndaed),
            "vouched" => Ok(Trust::Vouched),
            "authenticated" => Ok(Trust::Authenticated),
            "public" => Ok(Trust::Public),
            _ => Err(TrustError::InvalidTrustLevel),
        }
    }
}
impl TryFrom<String> for Trust {
    type Error = TrustError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Trust::try_from(s.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use failure::Error;

    #[test]
    fn test_from_str() -> Result<(), Error> {
        assert_eq!(Trust::try_from("staff")?, Trust::Staff);
        assert_eq!(Trust::try_from("ndaed".to_owned())?, Trust::Ndaed);
        Ok(())
    }

    #[test]
    fn test_ord() {
        assert!(Trust::Ndaed >= Trust::Public);
        assert!(Trust::Staff <= Trust::Staff);
    }
}

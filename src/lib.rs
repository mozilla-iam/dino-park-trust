#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;

use std::convert::TryFrom;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum Trust {
    Public,
    Authenticated,
    Vouched,
    Ndaed,
    Staff,
}

#[derive(Clone, Fail, Debug)]
pub enum TrustError {
    #[fail(display = "Invalid trust level")]
    InvalidTrustLevel,
    #[fail(display = "More trust required")]
    TrustLevelToLow,
}

#[derive(Clone, Fail, Debug)]
pub enum GroupsTrustError {
    #[fail(display = "Invalid groups trust level")]
    InvalidGroupsTrustLevel,
    #[fail(display = "More groups trust required")]
    GroupsTrustLevelToLow,
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

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum GroupsTrust {
    Creator,
    Admin,
    None,
}

impl TryFrom<&str> for GroupsTrust {
    type Error = GroupsTrustError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "creator" => Ok(GroupsTrust::Creator),
            "admin" => Ok(GroupsTrust::Admin),
            "none" | "" => Ok(GroupsTrust::None),
            _ => Err(GroupsTrustError::InvalidGroupsTrustLevel),
        }
    }
}

impl TryFrom<String> for GroupsTrust {
    type Error = GroupsTrustError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        GroupsTrust::try_from(s.as_ref())
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
    fn trust_test_ord() {
        assert!(Trust::Ndaed >= Trust::Public);
        assert!(Trust::Staff <= Trust::Staff);
    }

    #[test]
    fn groups_from_str() -> Result<(), Error> {
        assert_eq!(GroupsTrust::try_from("")?, GroupsTrust::None);
        assert_eq!(GroupsTrust::try_from("creator")?, GroupsTrust::Creator);
        assert_eq!(GroupsTrust::try_from("admin")?, GroupsTrust::Admin);
        Ok(())
    }

    #[test]
    fn groups_test_ord() {
        assert!(GroupsTrust::Admin >= GroupsTrust::Creator);
    }
}

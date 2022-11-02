#[macro_use]
extern crate serde_derive;
use thiserror::Error;

use std::convert::TryFrom;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum Trust {
    Public,
    Authenticated,
    Vouched,
    Ndaed,
    Staff,
}

#[derive(Clone, Error, Debug, PartialEq, Eq)]
pub enum TrustError {
    #[error("invalid_trust_level")]
    InvalidTrustLevel,
    #[error("more_trust_required")]
    TrustLevelToLow,
}

#[derive(Clone, Error, Debug, PartialEq, Eq)]
pub enum GroupsTrustError {
    #[error("invalid_groups_trust_level")]
    InvalidGroupsTrustLevel,
    #[error("more_groups_trust_required")]
    GroupsTrustLevelToLow,
}

#[derive(Clone, Error, Debug, PartialEq, Eq)]
pub enum AALevelError {
    #[error("higher_aal_required")]
    AALevelToLow,
}

impl Trust {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Staff => "staff",
            Self::Ndaed => "ndaed",
            Self::Vouched => "vouched",
            Self::Authenticated => "authenticated",
            Self::Public => "public",
        }
    }
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

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum GroupsTrust {
    None,
    Creator,
    Admin,
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

/// Authenticator Assurance Level
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd)]
#[serde(rename_all = "UPPERCASE")]
pub enum AALevel {
    Unknown,
    Low,
    Medium,
    High,
    Maximum,
}

impl From<&str> for AALevel {
    fn from(s: &str) -> Self {
        match s {
            "LOW" => AALevel::Low,
            "MEDIUM" => AALevel::Medium,
            "HIGH" => AALevel::High,
            "MAXIMUM" => AALevel::Maximum,
            _ => AALevel::Unknown,
        }
    }
}

impl From<String> for AALevel {
    fn from(s: String) -> Self {
        AALevel::from(s.as_ref())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Error;

    #[test]
    fn test_from_str() -> Result<(), Error> {
        assert_eq!(Trust::try_from("staff")?, Trust::Staff);
        assert_eq!(Trust::try_from("ndaed".to_owned())?, Trust::Ndaed);
        Ok(())
    }

    #[test]
    fn test_to_str() -> Result<(), Error> {
        assert_eq!("staff", Trust::Staff.as_str());
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

    #[test]
    fn aal_from_str() {
        assert_eq!(AALevel::from(""), AALevel::Unknown);
        assert_eq!(AALevel::from("MEDIUM"), AALevel::Medium);
    }

    #[test]
    fn aal_test_ord() {
        assert!(AALevel::Unknown < AALevel::Low);
        assert!(AALevel::High > AALevel::Low);
    }
}

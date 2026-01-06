use std::fmt::{self, Display};

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::service::playlist::enums::MediaType;

use super::enums::Platform;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct Id {
    pub platform: Platform,
    pub media_type: MediaType,
    pub id: String,
}
impl Id {
    pub fn new(platform: Platform, media_type: MediaType, id: String) -> Self {
        Self {
            platform,
            media_type,
            id,
        }
    }
    pub fn from_string(s: String) -> anyhow::Result<Self> {
        let mut parts = s.split(",");

        let platform: Platform = parts
            .next()
            .context(
                "malformed id string; expected 3 parts with the first part being a valid platform",
            )?
            .parse()?;
        let media_type: MediaType = parts
            .next()
            .context("malformed id string; expected 3 parts with the second part being a valid media type")?
            .parse()?;
        let id: String = parts
            .next()
            .context("malformed id string; expected 3 parts with the third being a valid id")?
            .to_string();
        Ok(Id {
            platform,
            media_type,
            id,
        })
    }
    pub fn valid_string(s: String) -> bool {
        matches!(Self::from_string(s), Ok(_))
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{},{},{}",
            &self.platform.to_string(),
            &self.media_type.to_string(),
            &self.id
        )
    }
}

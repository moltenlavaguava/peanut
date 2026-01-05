use anyhow::Context;

use crate::service::playlist::enums::MediaType;

use super::enums::Platform;

#[derive(Debug)]
pub struct Id {
    platform: Platform,
    media_type: MediaType,
    id: String,
}
impl Id {
    pub fn new(platform: Platform, media_type: MediaType, id: String) -> Self {
        Self {
            platform,
            media_type,
            id,
        }
    }
    pub fn to_string(&self) -> String {
        format!(
            "{}.{}.{}",
            &self.platform.to_string(),
            &self.media_type.to_string(),
            self.id.clone(),
        )
    }
    pub fn from_string(s: String) -> anyhow::Result<Self> {
        let mut parts = s.split(".");

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
}

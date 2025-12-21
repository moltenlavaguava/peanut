use crate::service::playlist::enums::MediaType;

use super::enums::Platform;

pub struct Id {
    platform: Platform,
    media_type: MediaType,
    id: String,
}
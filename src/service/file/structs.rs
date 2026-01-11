use std::path::PathBuf;

use crate::service::file::enums::SizeUnit;

#[derive(Clone)]
pub struct BinApps {
    pub yt_dlp: PathBuf,
    pub ffmpeg: PathBuf,
    pub deno: PathBuf,
}

#[derive(Debug, Clone)]
pub struct DataSize {
    size: u64,
}
impl DataSize {
    pub fn new(size: f64, unit: SizeUnit) -> Self {
        // do the conversions
        match unit {
            SizeUnit::Kibibyte => Self {
                size: (size * 1024.0) as u64,
            },
            SizeUnit::Kilobyte => Self {
                size: (size * 1000.0) as u64,
            },
            SizeUnit::Mebibyte => Self {
                size: (size * 1048576.0) as u64,
            },
            SizeUnit::Megabyte => Self {
                size: (size * 1000000.0) as u64,
            },
        }
    }
    pub fn as_kibibytes(&self) -> f64 {
        self.size as f64 / 1024.0
    }
    pub fn as_kilobytes(&self) -> f64 {
        self.size as f64 / 1000.0
    }
    pub fn as_mebibytes(&self) -> f64 {
        self.size as f64 / 1048576.0
    }
}

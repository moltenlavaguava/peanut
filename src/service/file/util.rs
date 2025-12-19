use std::env;
use std::path::PathBuf;

use super::structs::BinApps;

/// Returns the current project root. If in debug mode, returns the root directory for the project. Otherwise, returns the directory the executable is in.
pub fn get_project_root() -> std::io::Result<PathBuf> {
    #[cfg(not(debug_assertions))]
    {
        let mut exe_path = env::current_exe()?;
        return Ok(exe_path.pop());
    }
    #[cfg(debug_assertions)]
    {
        Ok(PathBuf::from(env!("CARGO_MANIFEST_DIR")))
    }
}

pub fn get_bin_app_paths() -> BinApps {
    // get root path where binaries should be located
    let mut root_path = get_project_root().expect("Critical: could not find root directory path");
    // manipulate paths
    root_path.push("bin");
    let yt_dlp_path = root_path.join("yt-dlp_x86.exe");
    let ffmpeg_path = root_path.join("ffmpeg").join("ffmpeg.exe");
    if !yt_dlp_path.is_file() || !ffmpeg_path.is_file() {
        panic!(
            "Critical: yt_dlp or ffmpeg exes are not found or invalid.\nEnsure that there exists a bin folder in the main directory and that it contains yt-dlp_x86.exe and a folder named ffmpeg containing ffmpeg.exe"
        )
    }
    // construct result
    BinApps {
        yt_dlp: yt_dlp_path,
        ffmpeg: ffmpeg_path,
    }
}

use peanut::core::CoreService;
use peanut::service::file::util;
use std::fs;

fn main() {
    // create basic folder structure for program if it doesn't already exist
    fs::create_dir_all(util::track_dir_path().unwrap()).unwrap();
    fs::create_dir_all(util::data_dir_path().unwrap()).unwrap();

    let _ = CoreService::spawn();
}

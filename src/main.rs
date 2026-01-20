use peanut::core::CoreService;
use peanut::service::file::util;
use std::fs;

fn main() {
    // create basic folder structure for program if it doesn't already exist
    fs::create_dir_all(util::track_dir_path().unwrap()).unwrap();
    fs::create_dir_all(util::data_dir_path().unwrap()).unwrap();

    let _ = CoreService::spawn();
}

// #[tokio::main]
// async fn main() {
//     let s = fs::read_to_string(
//         "C:\\Users\\lavam\\Projects\\peanut\\output\\data\\yt,pl,PLQgzBNb2huG9k0ubtbHIBrQzCLmMOhB6_.json",
//     ).unwrap();
//     let playlist: Playlist = serde_json::from_str(&s).unwrap();
//     let tracks = playlist.tracks;
//     let mut metadatas = Vec::new();

//     let mut client = MusicBrainzClient::default();
//     client
//         .set_user_agent("peanut/0.1.0 ( https://github.com/moltenlavaguava/peanut )")
//         .unwrap();

//     // all of them
//     for track in tracks {
//         let metadata = audio::identification::extract_metadata(&track);

//         // println!("searching..?");
//         // let safe_title = metadata.track_title.replace("\"", "");
//         // let safe_artist = metadata.main_artist_string.replace("\"", "");
//         // let query = format!(
//         //     r#"query=recording:"{}" AND artist:"{}""#,
//         //     safe_title, safe_artist
//         // );
//         // let result = Recording::search(query).execute_with_client(&client).await;
//         // println!("Search result: {:?}", result);

//         metadatas.push(metadata);
//     }

//     // just one

//     // let test_str = "Lace (Silksong OST Sample)".to_string();
//     // let artist = Artist::Community("Christopher Larkin".to_string());
//     // let track = Track {
//     //     album: None,
//     //     title: test_str,
//     //     length: Duration::from_secs(0),
//     //     artist,
//     //     source_id: Id::new(Platform::Youtube, MediaType::Track, "fff".to_string()),
//     //     dyn_id: Id::new(Platform::Youtube, MediaType::Track, "fff".to_string()),
//     //     index: 0,
//     //     download_url: Url::from_str("https://www.youtube.com/watch?v=B_myvYZZBms&list=PLQgzBNb2huG9k0ubtbHIBrQzCLmMOhB6_&index=68").unwrap(),
//     // };
//     // let metadata = audio::identification::extract_metadata(&track);
//     // println!("~~ metadata ~~\n{metadata:?}");

//     // println!("searching..?");
//     // let safe_title = metadata.track_title.replace("\"", "");
//     // let safe_artist = metadata.main_artist_string.replace("\"", "");
//     // let query = format!(
//     //     r#"query=recording:"{}"~ AND artist:"{}""#,
//     //     safe_title, safe_artist
//     // );
//     // let result = Recording::search(query).execute_with_client(&client).await;
//     // println!("Search result: {:?}", result);

//     // metadatas.push(metadata);

//     let json = serde_json::to_string(&metadatas).unwrap();
//     fs::write("output.txt", json).unwrap();
// }

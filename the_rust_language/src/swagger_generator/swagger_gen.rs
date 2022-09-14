use crate::swagger_generator::models::swagger_format::SwaggerFormat;
use reqwest::blocking::get;
use serde::{Deserialize, Serialize};

const Swagger_URL: &str =
    "https://natcom-api-development.azurewebsites.net/swagger/v1/swagger.json";

pub fn song_example() {
    pub type Response = Vec<Song>;

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Song {
        pub id: i64,
        #[serde(rename = "type")]
        pub type_field: String,
        pub title: String,
        pub artist: Artist,
        pub chords_present: bool,
        pub tab_types: Vec<String>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Artist {
        pub id: i64,
        #[serde(rename = "type")]
        pub type_field: String,
        pub name_without_the_prefix: String,
        pub use_the_prefix: bool,
        pub name: String,
    }

    let res = get("https://www.songsterr.com/a/ra/songs.json?pattern=Beatles").unwrap();
    let songs = res.json::<Response>().unwrap();
    for song in songs {
        println!(
            "Song: {} Artist: {} Tab types: {:?}",
            song.title, song.artist.name, song.tab_types
        );
    }
}

pub fn get_data() {
    let res = match get(Swagger_URL) {
        Result::Ok(val) => val,
        Result::Err(err) => panic!("API call failed : {}", err),
    };

    let json_response = match res.json::<SwaggerFormat>() {
        Result::Ok(val) => val,
        Result::Err(err) => panic!("Failed to parse JSON : {}", err),
    };

    let paths_array = match json_response.paths.as_array() {
        Option::Some(pathsArray) => Some(pathsArray),
        None => None,
    };
    let paths_obj = json_response.paths.as_object();

    println!("paths_array = {:?}", json_response.paths);

    println!("paths_obj = {:?}", paths_obj);

    // print!(
    //     "Song: {:?} ",
    //     serde_json::to_string_pretty(&json_response)
    //         .unwrap()
    //         .replace("\n", "\n\r")
    // );
}

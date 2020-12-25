use core::panic;
use std::{convert::TryInto, io::Cursor, path::{Path, PathBuf}};

use hyper::http::{
	Uri,
};
use beatsaver_rs::{BeatSaverApi, client::BeatSaver};
use zip::ZipArchive;

const CUSTOM_SONGS_FOLDER: &str = "Beat Saber_Data/CustomLevels";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Config {
	path: PathBuf
}
impl Default for Config {
	fn default() -> Self {
		 Config {
			 path: get_default_installation_path()
		 }
	}
}

#[tokio::main]
async fn main() {
	match &*std::env::args().nth(1).expect("Expected Argument") {
		"--set-path" | "-p" => {
			let path = match std::env::args().nth(2) {
				Some(path) => PathBuf::from(path),
				None => get_default_installation_path(),
			};
			println!("setting path to {:?}", path);
			let mut config: Config = confy::load("lbsaver").unwrap();
			config.path = path;
			confy::store("lbsaver", config).unwrap();
		}
		_ => {
			let link = std::env::args().last().expect("Expected URI to Asset");
			let beat_saver = BeatSaver::new();
			install_asset(&beat_saver, &link).await;
		}
	}
}

async fn install_asset(beat_s: &BeatSaver, link: &str) {

	let config = get_config();

	let uri: Uri = link.parse().expect("Invalid URI");
	match uri.scheme_str().expect("Invalid URI") {
		"beatsaver" => {
			println!("Installing beatsaver asset");
			let asset = uri.host().unwrap();
			let map_id = &asset.try_into().unwrap();
			let map = beat_s.map(map_id).await.expect("Error getting map data");
			let map_name = format!("{} ({} - {})", map.key, map.name, map.metadata.level_author);
			println!("Map: {}", map_name);

			let map_download = beat_s.download(map_id.clone()).await.unwrap();
			
			let song_path = config.path.join(&Path::new(CUSTOM_SONGS_FOLDER));
			let dir = song_path.join(map_name);
			println!("Downloading");

			println!("Saving map to: {:?}", dir);
			match tokio::fs::create_dir(dir.clone()).await {
				Ok(_) => (),
				Err(err) => match err.kind() {
					std::io::ErrorKind::AlreadyExists => (),
					_ => panic!("Error creating map directory: {}", err)
				}
			}

			println!("Extracting");
			let mut zip = ZipArchive::new(Cursor::new(&map_download[..])).unwrap();
			zip.extract(dir).unwrap();
			println!("Done")
		}
		_ => panic!("Invalid URI scheme. Currently only beatsaver is supported")
	}
}

fn get_default_installation_path() -> PathBuf {
	let home_str = std::env::var("HOME").unwrap();
	let home = Path::new(&home_str);
	let relative = Path::new(".steam/debian-installation/steamapps/common/Beat Saber/");
	home.join(relative)
}
fn get_config() -> Config {
	confy::load("lbsaver").unwrap()
}
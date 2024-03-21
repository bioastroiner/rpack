// Module responsible for handling mcmodinstance files

use serde::{Deserialize, Serialize};

/// metadata.packconfig
#[derive(Serialize, Deserialize)]
pub struct MeteaData {
    /*
    [file]
    formatVersion = 1 # Don't change this for now, only do if there's a significant format update in the future.

    [modloader]
    type = forge
    version = [Your forge version, ex: 1614]
    minecraftVersion = [Your Minecraft version, ex: 1.7.10]

    [pack]
    name = [Your pack name]
    author = [Your name]
    description = [A short description, only on one line]
    version = [Your pack version]
    */
    #[serde(rename = "modLoader")]
    mod_loader: ModLoader,
    pack: Pack,
}
#[derive(Serialize, Deserialize)]
struct ModLoader {
    #[serde(rename = "type")]
    type_cfg: String,
    version: u32,
    #[serde(rename = "minecraftVersion")]
    minecraft_version: String,
}
#[derive(Serialize, Deserialize)]
struct Pack {
    description: String,
    name: String,
    author: String,
    version: String,
}
/// optionals.packconfig
/// defines how the menus are structered
struct Optionals {}
/// resources.packconfig
/// houses download links to jars
struct Resources {}

pub fn read_metadata(metadata_toml:&String) -> MeteaData {
    let metadata : MeteaData = toml::from_str(metadata_toml).unwrap();
    metadata
}
pub fn read_options() -> Optionals {
    todo!()
}
pub fn read_resources() -> Resources {
    todo!()
}

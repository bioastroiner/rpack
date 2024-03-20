use serde::{Deserialize, Serialize};
use serde_json::{Result, Serializer, Value};
#[derive(Serialize, Deserialize)]
pub struct McInfo {
    modid: String,
    name: String,
    description: String,
    version: String,
    mcversion: String,
    url: String,
    #[serde(rename = "authorList")]
    author_list: Vec<String>,
}
/// parse McModInfo files for fancy printings
/// could be used to effortlessly find a curse/modrinth/github page as well
/// but that isnt its main intended porpouse
/// Note: this is for 1.7.10 mcmodinfo pack format
pub fn parse_mcinfo(content: &String) -> McInfo {
    let arr: Value = serde_json::from_str(content).unwrap();
    let mcinfo: McInfo = serde_json::from_str(arr[0].to_string().as_str()).unwrap();
    mcinfo
}

#[test]
fn serialize_test() {
let left = McInfo{
        modid: String::from("unimixins"),
        name: String::from("UniMixins"),
        description: String::from("UniMixins composite jar consisting of the following modules:\nMixin (UniMix); Compatibility; Mixingasm; SpongeMixins; MixinBooterLegacy; GasStation; GTNHMixins; MixinExtras"),
        version: String::from("0.1.11"),
        mcversion: String::from("1.7.10"),
        url: String::from("https://github.com/LegacyModdingMC/UniMixins"),
        author_list: [
            String::from("makamys")
        ].to_vec()
    };
let right = parse_mcinfo(&String::from(
        r#"
[
    {
        "modid": "unimixins",
        "name": "UniMixins",
        "description": "UniMixins composite jar consisting of the following modules:\nMixin (UniMix); Compatibility; Mixingasm; SpongeMixins; MixinBooterLegacy; GasStation; GTNHMixins; MixinExtras",
        "version": "0.1.11",
        "mcversion": "1.7.10",
        "url": "https://github.com/LegacyModdingMC/UniMixins",
        "authorList": [
            "makamys"
        ]
    }
] 
    "#,
    ));
    assert_eq!(left.modid,right.modid);
    assert_eq!(left.description,right.description);
    assert_eq!(left.author_list,right.author_list);
    assert_eq!(left.version,right.version);
}

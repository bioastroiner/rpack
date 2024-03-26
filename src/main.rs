use std::{
    io::Read,
    path::{Path, PathBuf},
};

use clap::{Arg, ArgAction, Command};
use reqwest::Method;

mod curse;
mod mcinstance;
mod modinfo;
mod modjars;
enum EntryType {
    Curse(String, String),
    Modrinth(String, String),
    Url(String),
}
struct Entry {
    source: EntryType,
    name: String,
    parent_file: String,
    hash: String,
}
fn main() {
    // let input = Args::parse();
    let cli = clap::Command::new("rpack")
        .about("pack utility")
        .subcommand(
            Command::new("add")
            .short_flag('A')
            .about("Adds an entry mod to your resources.packinfo, provoide your own custom file by -f flag").args([
                Arg::new("curse")
                    .long("curse-forge")
                    .short('c')
                    .conflicts_with("modrinth")
                    .conflicts_with("url")
                    .value_names(["PID","FID"]),
                Arg::new("modrinth")
                    .long("modrinth")
                    .short('m')
                    .conflicts_with("curse")
                    .conflicts_with("url")
                    .value_names(["PID","FID"]),
                Arg::new("url")
                    .long("url")
                    .short('u')
                    .conflicts_with("curse")
                    .conflicts_with("modrinth")
                    .value_names(["URL"]),
                Arg::new("hash")
                    .long("hash")
                    .short('H')
                    .help("to supply a SHA hash, must be a continious string"),
                Arg::new("name")
                    .long("name")
                    .alias("title")
                    .short('n')
                    .help("Not Required when used on curse or modrinth options"),
                Arg::new("res_file")
                    .long("res_file")
                    .short('f')
                    .help("Provide an Optional target for resource.packinfo, default is always resource.packinfo in your current directory if not otherwise configured")
            ]),
        )
        .subcommand(Command::new("search")
            .short_flag('S')
            .about("search curseforge for a mod")
        .arg(Arg::new("jar")
            .long("jar")
            .short('j')
            .help("path to a jar or directory containing jars (dose not search recursively!)")
            .value_name("JAR")
            .action(ArgAction::Append)
            )
        // .arg(arg!(-j --jar <JAR> "search a jar/multiple jars on curse forge, also accepts directories"))
            )
        .get_matches();
    match cli.subcommand() {
        Some(("add", add_args)) => {
            // todo!("retrive a curse/or/modrinth project id and file id, /or/ retrive a url");
            // todo!("retrive optional name");
            // todo!("retrive optional hash");
            // todo!("retrive optional alternative output file");
        }
        Some(("search", search_args)) => {
            let vals: Vec<&String> = search_args.get_many::<String>("jar").unwrap().collect();
            let all_jars = jars_from_paths(&vals);
            let all_jars_names = all_jars
                .iter()
                .map(|f| f.file_name().unwrap())
                .collect::<Vec<_>>();
            let all_jars_parsed: Vec<_> = all_jars_names
                .iter()
                .map(|f| {
                    f.to_str()
                        .expect(&format!("Failed to Parse {:?}", f))
                        .replace(".jar", "")
                })
                // .map(|f| f.split('-').map(|f| String::from(f)).collect::<Vec<_>>())
                // fuck u borrowchecker (:
                .filter_map(|f| match f.split_once('-') {
                    Some((s1, s2)) => Some((String::from(s1), String::from(s2))),
                    _ => None,
                })
                // .map(|f| f.)
                .collect();

            for ele in all_jars_parsed {
                //todo!("make searching less taxing on the server")
                search_curse_forge(&ele.0);
            }
            // println!(
            //     "found jars: {:#?} ===> {:?}\n",
            //     all_jars_names, all_jars_parsed
            // );
        }
        _ => unreachable!(),
    }
    // println!("{:#?}", cli);
}
/// returns a vector countaining all jars
/// in multiple directories, it also checks wheter
/// the input is valid directory and discards it
/// if the input is also a valid jar and not a dir
/// it will be included in the return
// todo move to one own's module
fn jars_from_paths(paths: &Vec<&String>) -> Vec<PathBuf> {
    let dirs: Vec<&Path> = paths
        .clone()
        .iter()
        .map(|s| Path::new(s.as_str()))
        .filter(|p| Path::is_dir(p))
        .collect();
    let jars_dirs: Vec<PathBuf> = dirs
        .clone()
        .iter()
        .map(|p| p.read_dir())
        .flat_map(|p| p.unwrap())
        .flat_map(|p| p.ok())
        .filter(|p| {
            p.path()
                .extension()
                .is_some_and(|s| s.to_str().eq(&Some("jar")))
        })
        .map(|p| p.path().to_owned())
        .collect();
    let jars: Vec<PathBuf> = paths
        .clone()
        .iter()
        .map(|s| PathBuf::from(s))
        .filter(|p| p.extension().is_some_and(|s| s.to_str().eq(&Some("jar"))))
        .collect();
    [jars_dirs, jars].concat()
}

//todo version search
//todo parse its json result
// todo move to one own's module
fn search_curse_forge(mod_name: &str) {
    let api = "$2a$10$bL4bIL5pUWqfcO7KQtnMReakwtfHbNKh6v1uTpKlzhwoueEJQnPnm"; // stolen from polymc
    let curse_link = r#"https://api.curseforge.com/v1/mods/search"#;
    let client = reqwest::blocking::Client::new();
    let resp = client
        .request(Method::GET, curse_link)
        .header("x-api-key", api)
        .header("Accept", "application/json")
        .query(&[("gameID", 432)])
        .query(&[("searchFilter", mod_name), ("gameVersion", "1.7.10")])
        .send();
    if let Some(mut res) = resp.ok() {
        let mut body = String::new();
        res.read_to_string(&mut body).ok();
        println!("Status: {}", res.status());
        println!("Headers:\n{:#?}", res.headers());
        println!(
            "Body:\n{:#?}",
            serde_json::from_str::<serde_json::Value>(body.as_str())
        );
    } else {
        println!("Failed to send request for {}", mod_name);
    }
}

use std::{
    borrow::Borrow,
    ffi::OsStr,
    path::{self, Path, PathBuf}, str::FromStr,
};

use clap::{arg, Arg, ArgAction, Command};

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
            todo!("retrive a curse/or/modrinth project id and file id, /or/ retrive a url");
            todo!("retrive optional name");
            todo!("retrive optional hash");
            todo!("retrive optional alternative output file");
        }
        Some(("search", search_args)) => {
            // todo!("search curseforge");
            // let vals : Vec<Vec<&String>> = search_args.get_occurrences("jar").unwrap().map(Iterator::collect).collect();
            let vals: Vec<&String> = search_args.get_many::<String>("jar").unwrap().collect();
            let all_jars = jars_from_paths(&vals);
            println!("found jars: {:#?}", all_jars.iter().map(|f|f.file_name().unwrap()).collect::<Vec<_>>());
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
        .map(|s|PathBuf::from(s))
        .filter(|p| p.extension().is_some_and(|s| s.to_str().eq(&Some("jar"))))
        .collect();
    [jars_dirs, jars].concat()
}

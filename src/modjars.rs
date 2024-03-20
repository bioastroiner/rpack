use jars::{Jar, JarOptionBuilder};

enum Source {
    CurseForge,
    Modrinth,
    Url,
    Local,
}
/// A Processed Jar representation
/// countains an optional modinfo
/// and a source for it to be located
/// with an associated path where it comes from
struct ModJar {
    path: String,
    modinfo: Option<String>,
    source: Source,
}
/// prints the contents of a jar
fn print_jar(jar: &Jar) {
    for (file_path, content) in &jar.files {
        println!("{}", file_path);
    }
}
/// adds a jar and check its mcmod.info's content
/// if it exists then check for it on curse or modrinth
/// if it didnt exist or last check failed (optionaly) ask the user
/// for a url if not provided add the jar directly to the override folder
fn add_jar(path_to_jar: &str) -> ModJar {
    let jar = jars::jar(path_to_jar, JarOptionBuilder::default());
    let mut minfo: Option<String> = None;
    if let Some(data) = jar.unwrap().files.get("mcmod.info") {
        minfo = Some(String::from_utf8(data.to_vec()).unwrap());
    } else {
        println!("couldnt read {} file", path_to_jar);
    }
    ModJar {
        path: String::from(path_to_jar),
        modinfo: minfo,
        source: Source::Local,
    }
}
/// resolves a source for a jar to be retrived
/// checks for modrinth and curse
/// if none existed then if not prompted for a URL
/// then adds the jar to a local pack
fn add_jar_source(path_to_jar: &str) -> Source {
    todo!(
    "check_curse_forge
    check_modrinth
    ask_for_url");
    
    Source::Local
}
#[test]
fn jar_test() {
    // !Example of a Non curse or Non Modrinth jar, also has no mcmod.info
    // add_jar("MicdoodleCore-1.7-3.0.12.504.jar");
    // !Example of a Curse jar // that also has mcmod.info
    // add_jar("Et_Futurum_Requiem-2.5.1+nomixin.jar");
}

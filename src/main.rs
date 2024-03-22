use clap::{Arg, Command};

mod curse;
mod mcinstance;
mod modinfo;
mod modjars;

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
        ).get_matches();
        match cli.subcommand() {
            Some(("Add",add_args)) => {
                todo!("retrive a curse/or/modrinth project id and file id, /or/ retrive a url");
                todo!("retrive optional name");
                todo!("retrive optional hash");
                todo!("retrive optional alternative output file");
            },
            _ => unreachable!()
        }
        println!("{:#?}",cli);
}

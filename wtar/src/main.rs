extern crate clap;
use clap::{App, Arg, SubCommand};

mod extractor;
mod creator;

fn main() {

    let  matches = App::new("wtar")
    .version("1.0")
    .author("Wyatt E. <wyatt.k.emery@gmail.com>")
    .about("A small version of tar that is compatible with tar -H ustar. Only supports files, directories and links")
    
    .subcommand(SubCommand::with_name("create")
        .about("Creates a tar file from a set of input files")
        .arg(Arg::with_name("file")
            .help("The file name of the output tar directory")
            .takes_value(true)
            .short("f")
            .long("file")
            .required(true))
        .arg(Arg::with_name("input")
            .help("The input files to be put into the tar directory")
            .takes_value(true)
            .index(1)
            .multiple(true)
            ))
                    
    .subcommand(SubCommand::with_name("extract")
        .about("Extracts a given tar file")
        .arg(Arg::with_name("file")
            .help("the file name of the unput tar directory")
            .takes_value(true)
            .short("f")
            .long("file")
            .required(true)))
    .get_matches();
    
    
    
    match matches.subcommand_name() {
        Some("extract") => extract(&matches.subcommand_matches("extract").unwrap()),
        Some("create") => create(&matches.subcommand_matches("create").unwrap()),
        None => println!("No subcommands were used"),
        _ => println!("An unrecognized subcmomand was used")
    }
    
}


fn extract( matches : &clap::ArgMatches)
{
    let input_file = matches.value_of("file").unwrap();
    /*Begin to extract the tar*/
}

fn create(matches: &clap::ArgMatches)
{
    let output_file =  matches.value_of("file").unwrap();
    let mut input_files = Vec::new();
    let values = matches.values_of("input");
    
    if values.is_some() {
        let files = values.unwrap();
        for file in files{
            input_files.push(file);
        }
    }
    
    creator::create(output_file, input_files);
    
}

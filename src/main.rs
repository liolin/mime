use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use std::str::FromStr;

mod generation;
mod structure_information;

use crate::generation::GenerationConfig;
use crate::structure_information::model::*;

static PROG_NAME: &str = "Mime";

fn main() {
    let matches = App::new(PROG_NAME)
        .version("0.1.0")
        .author("Olivier. L <olivier.lischer@liolin.ch>")
        .about("Generate source code from arguments")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            SubCommand::with_name("generate")
                .setting(AppSettings::SubcommandRequired)
                .help_short("Generate source code files")
                .subcommand(
                    SubCommand::with_name("model")
                        .arg(
                            Arg::with_name("name")
                                .index(1)
                                .help("The name of the new model")
                                .required(true),
                        )
                        .arg(Arg::with_name("field").multiple(true).help(
                            "The fields in the format name:type:visability (visability pub/pri)",
                        ))
                        .arg(
                            Arg::with_name("directory")
                                .long("dir")
                                .short("d")
                                .takes_value(true)
                                .value_name("DIR")
                                .help("Specify the models directory (default is src/models)"),
                        )
                        .arg(
                            Arg::with_name("force")
                                .long("force")
                                .short("f")
                                .help("Overwrites an existing file without warining"),
                        ),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("generate", Some(matches)) => run_generate(matches),
        _ => unreachable!("The cli parser should prevent reaching here"),
    }
}

fn run_generate(matches: &ArgMatches) {
    match matches.subcommand() {
        ("model", Some(matches)) => run_generate_model(matches),
        _ => unreachable!("The cli parser should prevent reaching here"),
    }
}

fn run_generate_model(matches: &ArgMatches) {
    let model_name = matches.value_of("name").unwrap();
    let fields: Vec<_> = matches.values_of("field").unwrap().collect();
    let config = GenerationConfig::from(matches);
    let fields: Vec<_> = fields
        .into_iter()
        .map(|s| FieldDefinition::from_str(s).unwrap())
        .collect();

    let model_data = ModelData::new(model_name, fields);
    let model = ModelDefinition::new(model_data);

    println!("Generate the following model:\n");
    print!("{}", model);
    model.generate(config).unwrap();
}

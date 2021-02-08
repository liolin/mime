use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use std::io::prelude::*;
use std::{fmt, str::FromStr};
use std::{fs::File, path::Path};

static PROG_NAME: &str = "Mime";

fn main() {
    let matches = App::new(PROG_NAME)
        .version("0.1.0")
        .author("Olivier. L <olivier.lischer@liolin.ch>")
        .about("Generate models based from given arguments")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            SubCommand::with_name("generate")
                .setting(AppSettings::SubcommandRequired)
                .subcommand(
                    SubCommand::with_name("model")
                        .arg(Arg::with_name("name").index(1).help("TODO").required(true))
                        .arg(Arg::with_name("field").multiple(true).help("TODO")),
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

    let fields: Vec<_> = fields
        .into_iter()
        .map(|s| FieldDefinition::from_str(s).unwrap())
        .collect();

    let model_data = ModelData::new(model_name, fields);
    let model = ModelDefinition::new(model_data);

    let model_dir = Path::new("./models");
    let file_path = model_dir
        .join(&model.model_data.file_name)
        .with_extension("rs");
    std::fs::create_dir_all(model_dir).unwrap();
    let mut file = File::create(file_path).unwrap();

    println!("Generate the following model:\n");
    print!("{}", model);
    write!(file, "{}", model).unwrap();
}

#[derive(Debug)]
struct ModelDefinition {
    model_data: ModelData,
}

impl ModelDefinition {
    fn new(data: ModelData) -> Self {
        Self { model_data: data }
    }
}

impl fmt::Display for ModelDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.model_data)
    }
}

#[derive(Debug)]
struct ModelData {
    class_name: String,
    file_name: String,
    field_data: Vec<FieldDefinition>,
}

impl ModelData {
    fn new(name: &str, fields: Vec<FieldDefinition>) -> Self {
        // transform first character to an its uppercase counter part
        let mut chars = name.chars();
        let uppercase_char: String = chars.next().unwrap().to_uppercase().collect();
        let uppercase_name = format!("{}{}", uppercase_char, chars.collect::<String>());

        let mut chars = name.chars();
        let lowercase_char: String = chars.next().unwrap().to_lowercase().collect();
        let lowercase_name = format!("{}{}", lowercase_char, chars.collect::<String>());

        Self {
            class_name: uppercase_name.to_owned(),
            file_name: lowercase_name.to_owned(),
            field_data: fields,
        }
    }
}

impl fmt::Display for ModelData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "struct {} {{", self.class_name)?;

        for field in &self.field_data {
            writeln!(f, "    {}", field)?
        }
        writeln!(f, "}}")?;
        Ok(())
    }
}

#[derive(Debug)]
enum FieldVisability {
    Public,
    Private,
}

impl fmt::Display for FieldVisability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            FieldVisability::Public => write!(f, "pub")?,
            FieldVisability::Private => {}
        }
        Ok(())
    }
}

impl FromStr for FieldVisability {
    type Err = FieldVisabilityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pub" => Ok(FieldVisability::Public),
            "pri" => Ok(FieldVisability::Private),
            _ => Err(FieldVisabilityError {}),
        }
    }
}

#[derive(Debug)]
struct FieldDefinition {
    name: String,
    ty: String,
    visability: FieldVisability,
}

impl fmt::Display for FieldDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.visability {
            FieldVisability::Public => write!(f, "{} {}: {},", self.visability, self.name, self.ty),
            FieldVisability::Private => write!(f, "{}: {},", self.name, self.ty),
        }
    }
}

impl FromStr for FieldDefinition {
    type Err = FieldDefinitionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tmp: Vec<&str> = s.split(':').collect();

        match tmp.len() {
            2 => Ok(FieldDefinition {
                name: tmp[0].to_owned(),
                ty: tmp[1].to_owned(),
                visability: FieldVisability::Public,
            }),
            3 => Ok(FieldDefinition {
                name: tmp[0].to_owned(),
                ty: tmp[1].to_owned(),
                visability: FieldVisability::from_str(tmp[2]).unwrap(),
            }),
            _ => Err(FieldDefinitionError {}),
        }
    }
}

#[derive(Debug)]
struct FieldDefinitionError {}

impl fmt::Display for FieldDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperError is here!")
    }
}

impl std::error::Error for FieldDefinitionError {}

#[derive(Debug)]
struct FieldVisabilityError {}

impl fmt::Display for FieldVisabilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperError is here!")
    }
}

impl std::error::Error for FieldVisabilityError {}

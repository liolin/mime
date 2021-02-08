use clap::ArgMatches;
use std::convert::From;

#[derive(Debug)]
pub struct GenerationConfig {
    force: bool,
    model_dir: String,
}

impl GenerationConfig {
    pub fn is_force(&self) -> bool {
        self.force
    }

    pub fn model_dir(&self) -> String {
        self.model_dir.clone()
    }
}

impl<'a> From<&ArgMatches<'a>> for GenerationConfig {
    fn from(args: &ArgMatches<'a>) -> Self {
        let force = args.is_present("force");
        let model_dir = match args.value_of("directory") {
            Some(dir) => dir.to_string(),
            None => "src/models".to_owned(),
        };

        GenerationConfig { force, model_dir }
    }
}

use clap::ArgMatches;
use std::convert::From;

#[derive(Debug)]
pub struct GenerationConfig {
    force: bool,
    dry_run: bool,
    model_dir: String,
}

impl GenerationConfig {
    pub fn is_force(&self) -> bool {
        self.force
    }

    pub fn model_dir(&self) -> String {
        self.model_dir.clone()
    }

    pub fn dry_run(&self) -> bool {
        self.dry_run
    }
}

impl<'a> From<&ArgMatches<'a>> for GenerationConfig {
    fn from(args: &ArgMatches<'a>) -> Self {
        let force = args.is_present("force");
        let dry_run = args.is_present("dry");
        let model_dir = match args.value_of("directory") {
            Some(dir) => dir.to_string(),
            None => "src/models".to_owned(),
        };

        GenerationConfig {
            force,
            dry_run,
            model_dir,
        }
    }
}

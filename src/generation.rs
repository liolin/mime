use clap::ArgMatches;
use std::convert::From;

#[derive(Debug)]
pub struct GenerationConfig {
    force: bool,
}

impl GenerationConfig {
    pub fn is_force(&self) -> bool {
        self.force
    }
}

impl<'a> From<&ArgMatches<'a>> for GenerationConfig {
    fn from(args: &ArgMatches<'a>) -> Self {
        GenerationConfig {
            force: args.is_present("force"),
        }
    }
}

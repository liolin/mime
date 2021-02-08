use std::io::ErrorKind;
use std::{fmt, fs::File, str::FromStr};
use std::{io::prelude::*, path::Path};

use crate::GenerationConfig;

#[derive(Debug)]
pub struct ModelDefinition {
    model_data: ModelData,
    model_dir: String,
}

impl ModelDefinition {
    pub fn new(data: ModelData) -> Self {
        Self {
            model_data: data,
            model_dir: "models".to_owned(),
        }
    }

    pub fn generate(&self, config: GenerationConfig) -> std::io::Result<()> {
        let model_dir = Path::new(&self.model_dir);
        let file_path = model_dir
            .join(&self.model_data.file_name)
            .with_extension("rs");
        std::fs::create_dir_all(model_dir).unwrap();

        if !config.is_force() {
            if file_path.exists() {
                return Err(std::io::Error::new(
                    ErrorKind::AlreadyExists,
                    format!("The file {} already exists", file_path.to_str().unwrap()),
                ));
            }
        };

        let mut file = File::create(file_path).unwrap();
        write!(file, "{}", self)
    }
}

impl fmt::Display for ModelDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.model_data)
    }
}

#[derive(Debug)]
pub struct ModelData {
    class_name: String,
    file_name: String,
    field_data: Vec<FieldDefinition>,
}

impl ModelData {
    pub fn new(name: &str, fields: Vec<FieldDefinition>) -> Self {
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
pub enum FieldVisability {
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
pub struct FieldDefinition {
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
pub struct FieldDefinitionError {}

impl fmt::Display for FieldDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperError is here!")
    }
}

impl std::error::Error for FieldDefinitionError {}

#[derive(Debug)]
pub struct FieldVisabilityError {}

impl fmt::Display for FieldVisabilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperError is here!")
    }
}

impl std::error::Error for FieldVisabilityError {}

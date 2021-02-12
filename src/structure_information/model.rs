use std::io::ErrorKind;
use std::{fmt, fs::File, str::FromStr};
use std::{io::prelude::*, path::Path};

use crate::GenerationConfig;

#[derive(Debug)]
pub struct ModelDefinition {
    model_data: ModelData,
}

impl ModelDefinition {
    pub fn new(data: ModelData) -> Self {
        Self { model_data: data }
    }

    pub fn generate(&self, config: GenerationConfig) -> std::io::Result<()> {
        if config.dry_run() {
            return Ok(());
        }

        let model_dir = config.model_dir();
        let model_dir = Path::new(&model_dir);
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
    table_name: String,
    field_data: Vec<FieldDefinition>,
}

impl ModelData {
    pub fn new(name: &str, table: &str, fields: Vec<FieldDefinition>) -> Self {
        let uppercase_name = Self::first_character_to_upper(name);
        let lowercase_name = Self::first_character_to_lower(&name);

        Self {
            class_name: uppercase_name.to_owned(),
            file_name: lowercase_name.to_owned(),
            table_name: table.to_owned(),
            field_data: fields,
        }
    }

    // TODO: Add a function trait as parameter. Then only one version of the following two functions
    // are required
    fn first_character_to_upper(characters: &str) -> String {
        let mut chars = characters.chars();
        let uppercase_char: String = chars.next().unwrap().to_uppercase().collect();
        format!("{}{}", uppercase_char, chars.collect::<String>())
    }

    fn first_character_to_lower(characters: &str) -> String {
        let mut chars = characters.chars();
        let uppercase_char: String = chars.next().unwrap().to_lowercase().collect();
        format!("{}{}", uppercase_char, chars.collect::<String>())
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

#[derive(Debug, PartialEq)]
pub enum FieldVisibility {
    Public,
    Private,
}

impl fmt::Display for FieldVisibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            FieldVisibility::Public => write!(f, "pub")?,
            FieldVisibility::Private => {}
        }
        Ok(())
    }
}

impl FromStr for FieldVisibility {
    type Err = FieldVisibilityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pub" => Ok(FieldVisibility::Public),
            "pri" => Ok(FieldVisibility::Private),
            _ => Err(FieldVisibilityError::new("Invalid visibility specified")),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct FieldDefinition {
    name: String,
    ty: String,
    visibility: FieldVisibility,
}

impl fmt::Display for FieldDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.visibility {
            FieldVisibility::Public => write!(f, "{} {}: {},", self.visibility, self.name, self.ty),
            FieldVisibility::Private => write!(f, "{}: {},", self.name, self.ty),
        }
    }
}

impl FromStr for FieldDefinition {
    type Err = FieldDefinitionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(' ') {
            return Err(FieldDefinitionError::new(
                FieldDefinitionErrorKind::Space,
                "Space not allowed in field definition",
            ));
        }
        let tmp: Vec<&str> = s.split(':').collect();

        match tmp.len() {
            2 => Ok(FieldDefinition {
                name: tmp[0].to_owned(),
                ty: tmp[1].to_owned(),
                visibility: FieldVisibility::Public,
            }),
            3 => Ok(FieldDefinition {
                name: tmp[0].to_owned(),
                ty: tmp[1].to_owned(),
                visibility: FieldVisibility::from_str(tmp[2])?,
            }),
            _ => Err(FieldDefinitionError::new(
                FieldDefinitionErrorKind::InvalidFormat,
                "Malformed field definition",
            )),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct FieldDefinitionError {
    kind: FieldDefinitionErrorKind,
    msg: String,
}

#[derive(Debug, PartialEq)]
pub enum FieldDefinitionErrorKind {
    Space,
    InvalidType,
    InvalidFormat,
    InvalidVisibility(FieldVisibilityError),
}

impl FieldDefinitionError {
    fn new<S: Into<String>>(kind: FieldDefinitionErrorKind, msg: S) -> Self {
        Self {
            kind,
            msg: msg.into(),
        }
    }
}

impl fmt::Display for FieldDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::convert::From<FieldVisibilityError> for FieldDefinitionError {
    fn from(error: FieldVisibilityError) -> Self {
        FieldDefinitionError::new(
            FieldDefinitionErrorKind::InvalidVisibility(error),
            "Invalid visibility",
        )
    }
}

impl std::error::Error for FieldDefinitionError {}

#[derive(Debug, PartialEq)]
pub struct FieldVisibilityError {
    msg: String,
}

impl FieldVisibilityError {
    fn new<S: Into<String>>(msg: S) -> FieldVisibilityError {
        FieldVisibilityError { msg: msg.into() }
    }
}

impl fmt::Display for FieldVisibilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for FieldVisibilityError {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn field_visibility_valid_from_str() {
        assert_eq!(
            FieldVisibility::Public,
            FieldVisibility::from_str("pub").unwrap()
        );
        assert_eq!(
            FieldVisibility::Private,
            FieldVisibility::from_str("pri").unwrap()
        );
    }

    #[test]
    fn field_viasbility_invalid_from_str() {
        assert_eq!(
            Err(FieldVisibilityError::new("Invalid visibility specified")),
            FieldVisibility::from_str("test")
        );
    }

    #[test]
    fn field_visibility_format() {
        let public = FieldVisibility::from_str("pub").unwrap();
        assert_eq!("pub", format!("{}", public));

        let private = FieldVisibility::from_str("pri").unwrap();
        assert_eq!("", format!("{}", private));
    }

    #[test]
    fn field_definition_valid_from_str() {
        let field = FieldDefinition {
            name: "id".to_owned(),
            ty: "i32".to_owned(),
            visibility: FieldVisibility::Public,
        };
        assert_eq!(field, FieldDefinition::from_str("id:i32").unwrap());

        let field = FieldDefinition {
            name: "id".to_owned(),
            ty: "i32".to_owned(),
            visibility: FieldVisibility::Private,
        };
        assert_eq!(field, FieldDefinition::from_str("id:i32:pri").unwrap());
    }

    #[test]
    fn field_definition_invalid_from_str() {
        assert_eq!(
            FieldDefinitionErrorKind::InvalidFormat,
            FieldDefinition::from_str("id:i32:pub:test")
                .err()
                .unwrap()
                .kind
        );
        assert_eq!(
            FieldDefinitionErrorKind::InvalidVisibility(FieldVisibilityError::new(
                "Invalid visibility specified"
            )),
            FieldDefinition::from_str("id:i32:hans").err().unwrap().kind
        );
        assert_eq!(
            FieldDefinitionErrorKind::Space,
            FieldDefinition::from_str("id:i32 test").err().unwrap().kind
        );
    }
}

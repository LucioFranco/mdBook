extern crate toml;

use std::path::{Path, PathBuf};
use std::error::Error;

pub struct Config {
    title: String,
    description: String,

    authors: Vec<Author>,

    source: PathBuf,

    outputs: Vec<Output>,

    language: Language,
    translations: Vec<Language>,

    plugins: Vec<Plugin>,
}

pub struct Author {
    name: String,
    email: Option<String>,
}

pub struct Output {
    identifier: String,
    destination: PathBuf,
    config: Option<toml::Table>,
}

pub struct Language {
    name: String,
    code: String,
}

pub struct Plugin {
    identifier: String,
    enabled: bool,
    config: Option<toml::Table>,
}


impl Config {
    pub fn new() -> Self {
        Config {
            title: String::new(),
            description: String::new(),

            authors: vec![],

            source: PathBuf::new(),

            outputs: vec![],

            language: Language::default(),
            translations: vec![],

            plugins: vec![],
        }
    }

    pub fn read_config(&mut self) -> Result<(), Box<Error>> {
        unimplemented!()
    }

    fn fill_config(&mut self, toml: &str) -> Result<(), Box<Error>> {
        unimplemented!()
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn authors(&self) -> &[Author] {
        &self.authors
    }

    pub fn source(&self) -> &Path {
        &self.source
    }

    pub fn outputs(&self) -> &[Output] {
        &self.outputs
    }

    pub fn language(&self) -> &Language {
        &self.language
    }

    pub fn translations(&self) -> &[Language] {
        &self.translations
    }

    pub fn plugins(&self) -> &[Plugin] {
        &self.plugins
    }
}


impl Author {
    /// Creates a new `Author` struct with the given name. The email field will be set to `None`
    pub fn new(name: &str) -> Self {
        Author {
            name: String::from(name),
            email: None,
        }
    }

    /// Builder pattern function, chained to `new()` it sets the email adress.
    /// Used like this:
    /// ```
    /// #extern crate mdbook;
    /// #
    /// #fn main() {
    ///     let author = mdbook::config::Author::new("John Doe").set_email("john@doe.org");
    /// #}
    ///
    pub fn set_email(mut self, email: &str) -> Self {
        self.email = Some(String::from(email));
        self
    }

    /// Returns the name of the author as `&str`
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns an `Option` with the email adress of the author
    pub fn email(&self) -> Option<&str> {
        self.email.as_ref().map(String::as_ref)
    }
}


impl Output {
    pub fn new(identifier: &str, destination: &Path) -> Self {
        Output {
            identifier: String::from(identifier),
            destination: PathBuf::from(destination),
            config: None,
        }
    }

    pub fn set_config(mut self, config: toml::Table) -> Self {
        self.config = Some(config);
        self
    }

    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    pub fn destination(&self) -> &Path {
        &self.destination
    }

    pub fn config(&self) -> Option<&toml::Table> {
        self.config.as_ref().map(|x| &*x)
    }
}


impl Language {
    pub fn new(name: &str, code: &str) -> Self {
        Language {
            name: String::from(name),
            code: String::from(code),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn code(&self) -> &str {
        &self.code
    }
}

impl Default for Language {
    fn default() -> Self {
        Language {
            name: String::from("English"),
            code: String::from("en"),
        }
    }
}


impl Plugin {
    pub fn new(identifier: &str, enabled: bool) -> Self {
        Plugin {
            identifier: String::from(identifier),
            enabled: enabled,
            config: None,
        }
    }

    pub fn set_config(mut self, config: toml::Table) -> Self {
        self.config = Some(config);
        self
    }

    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn config(&self) -> Option<&toml::Table> {
        self.config.as_ref().map(|x| &*x)
    }
}

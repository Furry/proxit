use colored::*;
use lazy_static::lazy_static;
pub struct Logger {
    logging_types: Vec<LoggingType>
}

lazy_static! {
    pub static ref LOGGER: Logger = Logger::default();
}

#[derive(PartialEq, Eq)]
pub enum LoggingType {
    Request,
    Warn,
    Error,
    Status,
    Log
}

impl Logger {
    pub fn default() -> Self {
        return Self {
            logging_types: vec![

            ]
        }
    }

    pub fn set_logging(&mut self, types: Vec<LoggingType>) {
        self.logging_types = types;
    }

    pub fn warn<T: Into<String>>(&self, contents: T) {
        if self.logging_types.contains(&LoggingType::Warn) {
            println!("{}", format!(
                "{}{}{}",
                "[WARN]".yellow(),
                ": ".white(),
                contents.into()
            ))
        }
    }

    pub fn error<T: Into<String>>(&self, contents: T) {
        if self.logging_types.contains(&LoggingType::Error) {
            println!("{}", format!(
                "{}{}{}",
                "[ERROR]".red(),
                ": ".white(),
                contents.into()
            ))
        }
    }

    pub fn log<T: Into<String>>(&self, contents: T) {
        if self.logging_types.contains(&LoggingType::Log) {
            println!("{}", format!(
                "{}{}{}",
                "[LOG]".green(),
                ": ".white(),
                contents.into()
            ))
        }
    }
}
// for person errors

use std::fmt;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PersonError {
    #[error("member not found")]
    PersonNotFound,

    #[error("failed to update member")]
    PersonUpdateFailure,

    #[error("failed to create member")]
    PersonCreationFailure,
}

// pub type ErrorMessage = String;
pub struct ErrorMessage(String);

impl ErrorMessage {
    pub fn new(message: &str) -> ErrorMessage {
        ErrorMessage(message.to_string())
    }

    fn create(person_error: PersonError) -> ErrorMessage {
        match person_error {
            PersonError::PersonNotFound => ErrorMessage::new("member not found"),
            PersonError::PersonUpdateFailure => ErrorMessage::new("failed to update member"),
            PersonError::PersonCreationFailure => ErrorMessage::new("failed to create member"),
        }
    }
}

impl fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
pub trait ResponseErrorTrait {
    fn create(person_error: PersonError) -> ErrorMessage;
}

impl ResponseErrorTrait for ErrorMessage {
    fn create(person_error: PersonError) -> ErrorMessage {
        match person_error {
            PersonError::PersonNotFound => ErrorMessage::create(person_error),
            PersonError::PersonUpdateFailure => ErrorMessage::create(person_error),
            PersonError::PersonCreationFailure => ErrorMessage::create(person_error),
        }
    }
}
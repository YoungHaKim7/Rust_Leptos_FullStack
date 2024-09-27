// for person errors

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

// struct ErrorMessage;

// impl ErrorMessage {
//     fn from(error: &str) -> String {
//         error.to_string()
//     }
// }

pub type ErrorMessage = String;

pub trait ResponseErrorTrait {
    fn create(person_error: PersonError) -> ErrorMessage;
}

impl ResponseErrorTrait for ErrorMessage {
    fn create(person_error: PersonError) -> ErrorMessage {
        match person_error {
            PersonError::PersonNotFound => ErrorMessage::from("member not found"),
            PersonError::PersonUpdateFailure => ErrorMessage::from("failed to update member"),
            PersonError::PersonCreationFailure => ErrorMessage::from("failed to create member"),
        }
    }
}

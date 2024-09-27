// for person errors

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PersonError {
    #[error("member not found")]
    PersonNotFound,

    #[error("failed to update member")]
    PersonUpdatefailure,

    #[error("failed to create member")]
    PersonCreationFailure,
}

pub type ErrorMessage = String;

pub trait ResponseErrorTrait {
    fn create(person_error: PersonError) -> ErrorMessage;
}

impl ResponseErrorTrait for ErrorMessage {
    fn create(person_error: PersonError) -> ErrorMessage {
        match person_error {
            PersonError::PersonNotFound => ErrorMessage::from("member not found"),
            PersonError::PersonUpdatefailure => ErrorMessage::from("failed to update member"),
            PersonError::PersonCreationFailure => ErrorMessage::from("failed to create member"),
        }
    }
}

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

// pub type ErrorMessage = String;

pub trait ResponseErrorTrait {
    fn create(person_error: PersonError) -> String;
}

impl<T: ResponseErrorTrait> ResponseErrorTrait for T {
    fn create(person_error: PersonError) -> String {
        match person_error {
            PersonError::PersonNotFound => "member not found".to_string(),
            PersonError::PersonUpdateFailure => "failed to update member".to_string(),
            PersonError::PersonCreationFailure => "failed to create member".to_string(),
        }
    }
}

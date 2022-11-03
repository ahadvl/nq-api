use actix_web::{error, Error};
use validator::Validate;

/// Validate the value and return actix error
pub fn validate<T>(data: &T) -> Result<(), Error>
where
    T: Validate,
{
    let validation = data.validate();

    if validation.is_err() {
        return Err(error::ErrorBadRequest(validation.err().unwrap()));
    }

    Ok(())
}

use validator::ValidationError;

pub fn validate_pagination(number: i8) -> Result<(), ValidationError> {
    match number >= 1 {
        true => Ok(()),
        false => Err(ValidationError::new(
            "Pagination value must be greather than or equal to 1",
        )),
    }
}

pub fn validate_string(string_to_validate: &str) -> Result<(), ValidationError> {
    let string_to_validate = string_to_validate.trim().replace(" ", "");

    match string_to_validate.len() >= 1 || !string_to_validate.is_empty() {
        true => Ok(()),
        false => Err(ValidationError::new("Value is not a valid String")),
    }
}

use std::borrow::Borrow;

use validator::ValidationErrors;

pub fn validation_error_message(e: ValidationErrors) -> String {
    let mut error_string: String = "".to_string();
    let validations = e.field_errors();
    let values = validations.values();
    for validation_errors in values {
        for validation_error in validation_errors.iter() {
            let error_message = validation_error.message.borrow();
            match error_message {
                Some(message) => {
                    error_string += message.borrow();
                    error_string += ".";
                }
                None => {}
            }
        }
    }
    error_string
}

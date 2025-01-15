use super::errors::TaskError;

const DEV : &str= "dev";
const NON_DEV : &str= "non-dev";
const WISH : &str= "wish";

pub fn validate_task_type(type_: &str) -> Result<(), TaskError> {
    if type_ == DEV || type_ == NON_DEV || type_ == WISH {
        Ok(())
    } else {
        Err(TaskError::InvalidPayload("invalid typeL use dev, non-dev or wish".to_owned()))
    }
    
}

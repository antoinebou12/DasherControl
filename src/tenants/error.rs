use std::fmt;

use bcrypt::BcryptError;
use diesel::result;

pub enum MyError {
    HashError(BcryptError),
    DBError(result::Error),
    PasswordNotMatch(String),
    WrongPassword(String),
    EmailExist(String)
}

// We need this to performs a conversion from BcryptError to MyError
impl From<BcryptError> for MyError {
    fn from(error: BcryptError) -> Self {
        MyError::HashError(error)
    }
}

// We need this to performs a conversion from diesel::result::Error to MyError
impl From<result::Error> for MyError {
    fn from(error: result::Error) -> Self {
        MyError::DBError(error)
    }
}

// We need this so we can use the method to_string over MyError 
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::HashError(error) => write!(f, "{}", error),
            MyError::DBError(error) => write!(f, "{}", error),
            MyError::PasswordNotMatch(error) => write!(f, "{}", error),
            MyError::WrongPassword(error) => write!(f, "{}", error),
            MyError::EmailExist(error) => write!(f, "{}", error)
        }
    }
}
use std::fmt;
use bcrypt::BcryptError;
use diesel::result;

pub enum MyStoreError {
    HashError(BcryptError),
    DBError(result::Error),
    PasswordNotMatch(String),
    WrongPassword(String)
}

// We need this to performs a conversion from BcryptError to MyStoreError
impl From<BcryptError> for MyStoreError {
    fn from(error: BcryptError) -> Self {
        MyStoreError::HashError(error)
    }
}

// We need this to performs a conversion from diesel::result::Error to MyStoreError
impl From<result::Error> for MyStoreError {
    fn from(error: result::Error) -> Self {
        MyStoreError::DBError(error)
    }
}

// We need this so we can use the method to_string over MyStoreError 
impl fmt::Display for MyStoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyStoreError::HashError(error) => write!(f, "{}", error),
            MyStoreError::DBError(error) => write!(f, "{}", error),
            MyStoreError::PasswordNotMatch(error) => write!(f, "{}", error),
            MyStoreError::WrongPassword(error) => write!(f, "{}", error)
        }
    }
}
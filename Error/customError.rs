 #[derive(Debug)]
enum LockError {
    MechanicalError(i32),
    NetworkError,
    NotAuthorized
}

use std::error::Error;
impl Error for LockError {}

use::std::fmt;
impl fmt::Display for LockError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
           self::MechanicalError(code) => write!(f,"Mechanical Error : {}" ,   code ),
           self::Network => write!(f,"Network Error"),
           self::NotAuthorized => write!(f,"Not Authorized"),
        }
    }
}

//this error
use thiserror::Error;

#[derive(Debug, Error)]
enum LockError {
    #[error("Mechanical Error : {0}")]
    MechanicalError(i32),
    #[error("Network Error")]
    NetworkError,
    #[error("Not Authorized")]
    NotAuthorized,
}

//usage 
fn lock_door() -> Result<() , LockError> {

    Err(LockError::NetworkError)
}

// Error Conversion
use thiserror::Error;

#[derive(Debug, Error)]
enum NetworkError {
  #[error("Connection Timed out")]
  TimeOut,
  #[error("Unreachable")]
  Unreachable
}


enum LockError {
    #[error("Mechanical Error : {0}")]
    MechanicalError(i32),
    #[error("Network Error")]
    NetworkError(#[from] NetworkError
    ),
    #[error("Not Authorized")]
    NotAuthorized,
  }
  

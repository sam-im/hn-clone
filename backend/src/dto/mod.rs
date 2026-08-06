pub mod user;

use crate::error::AppError;

pub trait Validate {
    fn validate(&self) -> Result<(), AppError>;
}

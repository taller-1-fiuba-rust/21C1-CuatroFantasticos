use std::fmt::{Debug, Formatter};

pub enum ConfServiceError {
    GetConfError,
}

impl Debug for ConfServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfServiceError::GetConfError => {
                write!(f, "GlobalResourses: Couldn't get a configuration")
            }
        }
    }
}

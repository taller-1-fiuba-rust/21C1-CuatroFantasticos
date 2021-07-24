use std::fmt::{Debug, Formatter};

pub enum GlobalResourcesError {
    GetConfError,
}

impl Debug for GlobalResourcesError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GlobalResourcesError::GetConfError => {
                write!(f, "GlobalResourses: Couldn't get a configuration")
            }
        }
    }
}

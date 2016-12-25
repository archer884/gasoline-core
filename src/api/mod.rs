use rocket::request::FromParam;
use service;
use std::error;
use std::fmt;
use std::ops::Deref;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

#[derive(Debug)]
pub enum ErrorKind {
    Unauthorized,
    NotFound,
    Invalid,
}

impl Error {
    pub fn not_found() -> Error {
        Error {
            kind: ErrorKind::NotFound,
            message: "Entity not found".into(),
        }
    }

    pub fn unauthorized() -> Error {
        Error {
            kind: ErrorKind::Unauthorized,
            message: "Unauthorized".into(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.message.as_ref()
    }
}

pub struct Identifier(i64);

impl Deref for Identifier {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> FromParam<'a> for Identifier {
    type Error = Error;

    fn from_param(param: &'a str) -> Result<Self> {
        match service::harsh().decode(param) {
            Some(ref mut x) if !x.is_empty() => {
                let id = x.pop().unwrap();
                Ok(Identifier(id as i64))
            },

            _ => Err(Error {
                kind: ErrorKind::Invalid,
                message: format!("`{}` does not map to a valid identifier", param),
            })
        }
    }
}
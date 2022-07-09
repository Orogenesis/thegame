use serde::{Deserialize, Serialize};
use crate::messages;

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd)]
#[serde(untagged)]
pub enum Id {
    /// Numeric ID.
    Number(usize),
    /// String ID.
    String(String),
}

#[derive(Deserialize)]
pub struct Request {
    /// An identifier established by the client.
    pub id: Id,
    /// A structured value that holds the parameter values
    /// to be used during the invocation of the method.
    pub payload: messages::MessageFromClient,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Response<T> where T: Serialize {
    /// Success response.
    Success(Result<T>),
    /// Failure response.
    Failure(Result<T>),
}

#[derive(Serialize)]
pub struct Result<T> where T: Serialize {
    /// Correlation id.
    /// It must be the same as the value of the id member in the request object.
    pub id: Id,
    /// Execution payload.
    pub payload: T,
}

impl<T> Response<T> where T: Serialize {
    /// Creates a success response.
    pub fn success(id: Id, payload: T) -> Self {
        Self::Success(Result::new(id, payload))
    }

    /// Creates a failure response.
    pub fn failure(id: Id, payload: T) -> Self {
        Self::Failure(Result::new(id, payload))
    }
}

impl<T: Serialize> Result<T> where T: Serialize {
    /// Creates an execution result.
    pub fn new(id: Id, payload: T) -> Self {
        Self { id, payload }
    }
}

impl From<usize> for Id {
    fn from(n: usize) -> Self {
        Id::Number(n)
    }
}

impl From<&'_ str> for Id {
    fn from(s: &'_ str) -> Self {
        Id::String(s.to_string())
    }
}

impl From<String> for Id {
    fn from(s: String) -> Self {
        Id::String(s)
    }
}

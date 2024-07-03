use tonic::Status;
use mongodb::error::Error as MongoError;
use reqwest::Error as ReqwestError;
use mongodb::bson::document::ValueAccessError;
use std::env::VarError;

#[derive(Debug)]
pub enum ArtieError {
    MongoDBError(MongoError),
    TonicError(Status),
    ReqwestError(ReqwestError),
    ValueAccessError(ValueAccessError),
    VarError(VarError),
}

impl From<MongoError> for ArtieError {
    fn from(err: MongoError) -> Self {
        ArtieError::MongoDBError(err)
    }
}

impl From<Status> for ArtieError {
    fn from(err: Status) -> Self {
        ArtieError::TonicError(err)
    }
}

impl From<ReqwestError> for ArtieError {
    fn from(err: ReqwestError) -> Self {
        ArtieError::ReqwestError(err)
    }
}

impl From<ValueAccessError> for ArtieError {
    fn from(err: ValueAccessError) -> Self {
        ArtieError::ValueAccessError(err)
    }
}

impl From<VarError> for ArtieError {
    fn from(err: VarError) -> Self {
        ArtieError::VarError(err)
    }
}

impl std::fmt::Display for ArtieError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArtieError::MongoDBError(err) => write!(f, "MongoDB error: {}", err),
            ArtieError::TonicError(err) => write!(f, "Tonic error: {}", err),
            ArtieError::ReqwestError(err) => write!(f, "Reqwest error: {}", err),
            ArtieError::ValueAccessError(err) => write!(f, "Value access error: {}", err),
            ArtieError::VarError(err) => write!(f, "Environment variable error: {}", err),
        }
    }
}

impl std::error::Error for ArtieError {}

impl From<ArtieError> for Status {
    fn from(err: ArtieError) -> Self {
        match err {
            ArtieError::MongoDBError(err) => Status::internal(format!("MongoDB error: {}", err)),
            ArtieError::TonicError(err) => err,
            ArtieError::ReqwestError(err) => Status::internal(format!("Reqwest error: {}", err)),
            ArtieError::ValueAccessError(err) => Status::internal(format!("Value access error: {}", err)),
            ArtieError::VarError(err) => Status::internal(format!("Environment variable error: {}", err)),
        }
    }
}

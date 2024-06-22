#[derive(Debug)]
pub enum ArtieChatError {
    MongoDBError(mongodb::error::Error),
    TonicError(tonic::Status),
    ReqwestError(reqwest::Error),
}

impl From<mongodb::error::Error> for ArtieChatError {
    fn from(err: mongodb::error::Error) -> Self {
        ArtieChatError::MongoDBError(err)
    }
}

impl From<tonic::Status> for ArtieChatError {
    fn from(err: tonic::Status) -> Self {
        ArtieChatError::TonicError(err)
    }
}

impl From<reqwest::Error> for ArtieChatError {
    fn from(err: reqwest::Error) -> Self {
        ArtieChatError::ReqwestError(err)
    }
}

impl std::fmt::Display for ArtieChatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArtieChatError::MongoDBError(err) => write!(f, "MongoDB error: {}", err),
            ArtieChatError::TonicError(err) => write!(f, "Tonic error: {}", err),
            ArtieChatError::ReqwestError(err) => write!(f, "Reqwest error: {}", err),
            // Agrega otras variantes según sea necesario
        }
    }
}

impl std::error::Error for ArtieChatError {}
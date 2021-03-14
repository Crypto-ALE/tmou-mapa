// TODO:
// - implement error::Error
// - add types of encapsulated models::errors for better handling
// - make an automatic response somehow?
#[derive(Debug)]
pub struct TmouError {
    pub message: String,
    pub response: u16,
}

impl TmouError {
    fn new(msg: &str, resp: u16) -> TmouError {
        TmouError {
            message: msg.to_string(),
            response: resp,
        }
    }
}

impl From<std::io::Error> for TmouError {
    fn from(err: std::io::Error) -> Self {
        TmouError::new(&format!("io operation failed: {}", err), 404)
    }
}

impl From<serde_json::error::Error> for TmouError {
    fn from(err: serde_json::error::Error) -> Self {
        TmouError::new(&format!("deserialization failed: {}", err), 404)
    }
}

impl From<roxmltree::Error> for TmouError {
    fn from(err: roxmltree::Error) -> Self {
        TmouError::new(&format!("Invalid XML data {}", err), 404)
    }
}

impl From<TmouError> for rocket::http::Status {
    fn from(err: TmouError) -> Self {
        match err.response {
            400 => rocket::http::Status::BadRequest,
            404 => rocket::http::Status::NotFound,
            _ => rocket::http::Status::InternalServerError,
        }
    }
}

impl From<diesel::result::Error> for TmouError {
    fn from(err: diesel::result::Error) -> Self {
        TmouError {
            message: err.to_string(),
            response: 404,
        }
    }
}

impl From<std::env::VarError> for TmouError {
    fn from(err: std::env::VarError) -> Self {
        TmouError {
            message: err.to_string(),
            response: 404,
        }
    }
}

impl From<diesel::ConnectionError> for TmouError {
    fn from(err: diesel::ConnectionError) -> Self {
        TmouError {
            message: err.to_string(),
            response: 500,
        }
    }
}

impl From<chrono::format::ParseError> for TmouError {
    fn from(err: chrono::format::ParseError) -> Self {
        TmouError {
            message: err.to_string(),
            response: 500,
        }
    }
}

impl From<evalexpr::EvalexprError> for TmouError {
    fn from(err: evalexpr::EvalexprError) -> Self {
        TmouError {
            message: err.to_string(),
            response: 500,
        }
    }
}

pub type TmouResult<T> = Result<T, TmouError>;

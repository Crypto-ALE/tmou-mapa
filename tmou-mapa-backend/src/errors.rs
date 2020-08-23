// TODO: 
// - implement error::Error
// - add types of encapsulated errors for better handling
// - make an automatic response somehow?
#[derive(Debug)]
pub struct TmouError
{
    pub message: String,
    pub response: u16
}

impl TmouError 
{
    fn new(msg: &str, resp: u16) -> TmouError 
    {
        TmouError{message: msg.to_string(), response: resp}
    }
}


impl From<std::io::Error> for TmouError
{
    fn from(err:std::io::Error) -> Self
    {
        TmouError::new(&format!("io operation failed: {}", err), 404)
    }
}

impl From<serde_json::error::Error> for TmouError
{
    fn from(err:serde_json::error::Error) -> Self
    {
        TmouError::new(&format!("deserialization failed: {}", err), 404)
    }
}

impl From<roxmltree::Error> for TmouError
{
    fn from(err:roxmltree::Error) -> Self
    {
        TmouError::new(&format!("Invalid OSM data {}", err), 404)
    }
}

impl From<TmouError> for rocket::http::Status
{
    fn from(err:TmouError) -> Self
    {
        // wtf, how to get String into the rocket's Status::reason?
        rocket::http::Status::new(err.response, "Unknown error")
    }
}

impl From<diesel::result::Error> for TmouError
{
    fn from(err:diesel::result::Error) -> Self
    {
        TmouError{message: err.to_string(), response: 404}
    }
}

impl From<std::env::VarError> for TmouError
{
    fn from(err:std::env::VarError) -> Self
    {
        TmouError{message: err.to_string(), response: 404}
    }
}


pub type TmouResult<T> = Result<T, TmouError>;
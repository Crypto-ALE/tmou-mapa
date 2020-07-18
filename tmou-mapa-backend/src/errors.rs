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
        TmouError::new("io operation failed", 404)
    }
}

impl From<serde_json::error::Error> for TmouError
{
    fn from(err:serde_json::error::Error) -> Self
    {
        TmouError::new("deserialization failed", 404)
    }
}

impl From<roxmltree::Error> for TmouError
{
    fn from(err:roxmltree::Error) -> Self
    {
        TmouError::new("Invalid OSM data", 404)
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


pub type TmouResult<T> = Result<T, TmouError>;
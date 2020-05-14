
use std::fmt;

/**
 * method.rs
 * RequestMethod
 */
#[allow(unused)]
pub enum RequestMethod {
    GET(String),
    
    POST(String),
    Delete(String),
    QUERY(String),
    UNSUPPORT,
}


impl fmt::Display for RequestMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r:String = match *self {
            RequestMethod::GET(ref s) => s.to_string(),
            _ => "UNSUPPORT".to_string(),
        };
        write!(f, ":{}", r)
    }
 }

impl RequestMethod {
    #[allow(unused)]
    pub fn value(&self) -> String {
        let val = match *self {
            RequestMethod::GET(ref s) => s.to_string(),
            _ => "UNSUPPORT".to_string(),
        };

        val
    }
}
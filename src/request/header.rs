/**
 * RequestHeader
 */

 use std::fmt;
 use super::method::RequestMethod;

 // path
 pub type URI = String;

 // version
 pub type HTTPVER = String;

 pub struct RequestHeader {
    method: RequestMethod,
    path:   URI,
    ver:    HTTPVER,
 }


 impl RequestHeader {
    pub fn translate(req: &str) -> RequestHeader {
        let mut req_args = req.split_whitespace();

        let get: String = String::from("get").to_uppercase();
        let method_val = req_args.next().unwrap().to_uppercase();
        let method: RequestMethod = match method_val {
            _ if get == method_val => RequestMethod::GET(get.to_string()),
            _  => RequestMethod::UNSUPPORT,
        };

        let uri: URI = req_args.next().unwrap().to_string();
        let ver: HTTPVER = req_args.next().unwrap().to_string();

        RequestHeader {
            method: method,
            path: uri,
            ver,
        }
    }

    pub fn eq_path(&self, val: String) -> bool {
        // println!("eq_path {0}, {1}", self.path.to_lowercase(), val.to_lowercase());
        self.path.to_string().to_lowercase() == val.to_lowercase()
    }

 }

 impl fmt::Display for RequestHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "print RequestHeader Method({}) Path({}) Ver({})", self.method, self.path, self.ver)
    }
 }
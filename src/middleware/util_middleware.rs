use actix_web::http::header::HeaderMap;
use actix_identity::Identity;
use crate::auth;


pub fn parse_jwt_token_header(token: &str) -> Result<String, String> {
    let token = token.to_string();
    let token: Vec<&str> = token.split(" ").collect();

    if token.len() < 2 {
        return Err(format!("Invalid authorization field format: token length invalid. must include atleast 2 tokens"));
    }

    if token[0].to_lowercase() != "bearer" {
        return Err(format!(
            "Authorization field does not include the keyword 'Bearer'"
        ));
    }

    let token_jwt = token[1].to_string();

    return Ok(token_jwt);
}

pub fn parse_cookie(cookie: &Option<Identity>) -> Result<String, String> {

    let cookie = match cookie {
        None => return Err("Cookie not found".to_string()),
        Some(v) => v,
    };

    let token = match cookie.id() {
        Err(e) => return Err(e.to_string()),
        Ok(v) => v,
    };

    return Ok(token);
}


pub fn parse_header_token(headers: &HeaderMap) -> Result<String, String> {
    let authorization = headers.get("Authorization");
    let authorization = match authorization {
        None => return Err("Auth header not found".to_string()),
        Some(v) => v,
    };

    let bearer = authorization.to_owned();

    let mut token = String::new();

    match bearer.to_str() {
        Err(e) => return Err(e.to_string()),
        Ok(v) => {
            token = v.to_string();
        }
    };

    let token_jwt = match parse_jwt_token_header(&token) {
        Err(e) => return Err(e),
        Ok(v) =>  v
    };
    
    return Ok(token_jwt);
}


pub fn extract_token_cookie_or_header(identity: &Option<Identity>, headers: &HeaderMap) -> Result<auth::JWTClaims, String>{

    let mut token_jwt: String = String::new();

    let mut cookie_available = false;

    let mut auth_header_available = false;

    let mut token_jwt_cookie: String = String::new();
    let mut token_jwt_header: String = String::new();

    let mut cookie_errors = String::new();


    match self::parse_cookie(identity) {
        Err(e) => {
            cookie_errors = e;
        }
        Ok(v) => {
            token_jwt_cookie = v;
            cookie_available = true;
        }
    };


    let mut auth_headers_errors = String::new();

    match self::parse_header_token(&headers) {
        Err(e) => {
            auth_headers_errors = e;
        }
        Ok(v) => {
            token_jwt_header = v;
            auth_header_available = true;
        }
    };

    if !cookie_available && !auth_header_available {

        let errors = format!("Unauthorized. auth not found in cookie or auth header: {}, {}", cookie_errors, auth_headers_errors);
        return Err(errors);
    }

    if cookie_available {
        token_jwt = token_jwt_cookie;
    } else if auth_header_available {
        token_jwt = token_jwt_header;
    }

    let token_decode = match auth::decode_token(token_jwt) {
        Err(e) => return Err(e),
        Ok(v) => v,
    };

    return Ok(token_decode);

}

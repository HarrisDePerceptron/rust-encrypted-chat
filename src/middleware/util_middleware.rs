use actix_web::http::header::HeaderMap;
use actix_identity::Identity;



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
    let mut token = String::new();

    let cookie = match cookie {
        None => return Err("Cookie not found".to_string()),
        Some(v) => v,
    };

    token = match cookie.id() {
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
    println!("auth header: {}", bearer.to_str().unwrap());

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

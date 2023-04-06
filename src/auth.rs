
use serde::{Serialize, Deserialize};
use std::fmt::{Debug};

use std::time::{SystemTime, UNIX_EPOCH, Instant, Duration, self};
use crate::utils;
use crate::secrets;


use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use actix_web::http::header::HeaderMap;


#[derive(Debug, Serialize, Deserialize)]
pub struct JWTClaims {
    pub exp: usize,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub iat: usize,          // Optional. Issued at (as UTC timestamp)
    pub iss: String,         // Optional. Issuer
    pub sub: String,         // Optional. Subject (whom token refers to)
}


pub fn generate_token(subject: &str, issuer: &str, expiry_days: u64) -> Result<String, String>{

    let utils::SECONDS(elasped) = match utils::get_current_timestamp() {
        Err(e)=> return Err(e),
        Ok(v) => v
    };


    let elasped = match usize::try_from(elasped){
        Err(e)=> return Err(e.to_string()),
        Ok(v) => v
    };

    let seconds_in_day: u64 = 24*60*60;
    let seconds_total: u64 =  seconds_in_day* expiry_days;


    let seconds_total  = match usize::try_from(seconds_total){
        Err(e) => return Err(e.to_string()),
        Ok(v) => v
    };

    let exp_elasped=  elasped + seconds_total;


    let claim = JWTClaims {
        iat: elasped,
        sub: subject.to_string(),
        exp: exp_elasped,
        iss: issuer.to_string(),
    };

    let token = match encode(&Header::default(), &claim, &EncodingKey::from_secret(secrets::SESSION_KEY.as_ref())){
        Err(e)=> return Err(e.to_string()),
        Ok(v) =>v
    };
    
    return Ok(token);
   
}


pub fn decode_token(token: String) -> Result<JWTClaims, String> {
    let token = match decode::<JWTClaims>(&token, &DecodingKey::from_secret(secrets::SESSION_KEY.as_ref()), &Validation::default()){
        Err(e) => return Err(e.to_string()),
        Ok(v) => v
    };

    let result = token.claims;
    return Ok(result);

}


pub fn verify_token(token: &str) -> Result<bool, String> {
    let mut validation= Validation::default();
    validation.validate_exp = true;
    validation.insecure_disable_signature_validation();

    let dummy_key = DecodingKey::from_secret(&[]);

    let data = match decode::<JWTClaims>(token, &dummy_key, &validation){
        Ok(v) => v,
        Err(e)=> return Err(e.to_string())
    };

    
    return Ok(true);

}





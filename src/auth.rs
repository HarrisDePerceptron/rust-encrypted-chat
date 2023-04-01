
use serde::{Serialize, Deserialize};
use std::fmt::{Debug};

use std::time::{SystemTime, UNIX_EPOCH, Instant, Duration, self};
use crate::utils;
use crate::secrets;


use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};



#[derive(Debug, Serialize, Deserialize)]
pub struct JWTClaims {
    aud: Option<String>,         // Optional. Audience
    exp: usize,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize,          // Optional. Issued at (as UTC timestamp)
    iss: String,         // Optional. Issuer
    nbf: Option<usize>,  // Optional. Not Before (as UTC timestamp)
    sub: String,         // Optional. Subject (whom token refers to)
}


pub fn generate_token(expiry_days: u64) -> Result<String, String>{

    let iat  = match SystemTime::now()
        .duration_since(UNIX_EPOCH) {
            Err(e)=> return Err(e.to_string()),
            Ok(v) => v
        };
    let elasped=  iat.as_millis();


    let elasped = match usize::try_from(elasped){
        Err(e)=> return Err(e.to_string()),
        Ok(v) => v
    };

    let uid = match utils::generate_unique_id(){
        Err(e) => return Err(e.to_string()),
        Ok(v)=> v
    };

    let seconds_in_day: u64 = 24*60*60;
    let seconds_total: u64 =  seconds_in_day* expiry_days;

    let exp = iat + Duration::from_secs(seconds_total);
    let exp_milli = exp.as_millis();
    let exp_milli = match usize::try_from(exp_milli){
        Err(e)=> return Err(e.to_string()),
        Ok(v)=> v
    };


    let claim = JWTClaims {
        iat: elasped,
        sub: uid,
        aud: None,
        exp: exp_milli,
        iss: "rust".to_string(),
        nbf: None
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



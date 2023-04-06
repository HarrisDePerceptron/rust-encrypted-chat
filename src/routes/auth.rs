
use actix_identity::{Identity, IdentityMiddleware};

use actix_web::{
    get, post, HttpMessage, HttpRequest, HttpResponse, Responder, web
};


use crate::auth;

use crate::utils;

use crate::secrets;
use crate::middleware::auth_extractor;

use serde::{Deserialize, Serialize};

#[get("/user")]
async fn index(user: Option<Identity>, auth: Option<auth_extractor::AuthExtractor>) -> impl Responder {

    if let Some(auth) = auth{
        println!("auth user id: {}", auth.user_id);
    }

    
    if let Some(user) = user {
        format!("Welcome! {}", user.id().unwrap())
    } else {
        "Welcome Anonymous!".to_owned()
    }
}



#[derive(Clone, Debug, Deserialize)]
pub struct VerifyRequest{
    pub token: String,
    
}

#[post("/verify")]
async fn verify(param: web::Json<VerifyRequest>,) -> impl Responder {
    let token = &param.token;
    
    let result = match auth::verify_token(token){
        Err(e)=> return HttpResponse::BadRequest().body(e.to_string()),
        Ok(v)=> v
    };

    HttpResponse::Ok().body("verified")
}



#[post("/login")]
async fn login(request: HttpRequest) -> impl Responder {

    let uid = match utils::generate_unique_id(){
        Err(e)=> return HttpResponse::BadRequest().body(e.to_string()),
        Ok(v) => v
    };


    let expiry: u64 = match secrets::TOKEN_EXPIRY_DAYS.to_string().parse(){
        Err(e)=> return HttpResponse::BadRequest().body(format!("{:?}",e)),
        Ok(v) => v
    };

    let token  = match auth::generate_token(&uid, 
        &secrets::TOKEN_ISSUER.to_string(), 
        expiry){
            Err(e)=> return HttpResponse::BadRequest().body(e.to_string()),
            Ok(v) => v
        };

    Identity::login(&request.extensions(), token.to_owned().into()).unwrap();

    HttpResponse::Ok().body(token)

}

#[post("/logout")]
async fn logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::NoContent()
}
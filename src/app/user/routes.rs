use actix_identity::{Identity, IdentityMiddleware};

use actix_web::{error, get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder, Result};

use redis::AsyncCommands;
use std::sync::Mutex;

use crate::auth;

use crate::app::application_factory::FactoryTrait;
use crate::utils;

use crate::middleware::auth_extractor;
use crate::secrets;

use serde::{Deserialize, Serialize};

use crate::persistence::redis::RedisProvider;
use crate::app::application_factory::ServiceFactory;
use super::routes_model;
use super::service_model;



#[get("/user")]
async fn index(
    user: Option<Identity>,
    auth: Option<auth_extractor::AuthExtractor>,
) -> impl Responder {
    if let Some(auth) = auth {
        println!("auth user id: {}", auth.user_id);
    }

    if let Some(user) = user {
        format!("Welcome! {}", user.id().unwrap())
    } else {
        "Welcome Anonymous!".to_owned()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct VerifyRequest {
    pub token: String,
}

#[post("/verify")]
async fn verify(param: web::Json<VerifyRequest>) -> impl Responder {
    let token = &param.token;

    let result = match auth::verify_token(token) {
        Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
        Ok(v) => v,
    };

    HttpResponse::Ok().body("verified")
}

#[derive(Clone, Debug, Deserialize)]
pub struct SignupRequest {
    pub username: String,
    pub password: String,
}
#[post("/signup")]
async fn signup(
    param: web::Json<SignupRequest>,
    redis: web::Data<Mutex<RedisProvider>>,
    service_factory: web::Data<ServiceFactory>
) -> Result<HttpResponse> {

    let uf = &service_factory.user;

    let mut us = uf.get();

    let signup_request = service_model::SignupRequest{
        username: param.username.to_string(),
        password: param.password.to_string()
    };

    let result = us.signup(signup_request)
        .await
        .map_err(|e| error::ErrorBadRequest(format!("{:?}", e)))?;



    Ok(HttpResponse::Ok().body(format!("signup complete: {:?}", result)))

}

#[post("/login")]
async fn login(request: HttpRequest) -> impl Responder {
    let uid = match utils::generate_unique_id() {
        Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
        Ok(v) => v,
    };

    let expiry: u64 = match secrets::TOKEN_EXPIRY_DAYS.to_string().parse() {
        Err(e) => return HttpResponse::BadRequest().body(format!("{:?}", e)),
        Ok(v) => v,
    };

    let token = match auth::generate_token(&uid, &secrets::TOKEN_ISSUER.to_string(), expiry) {
        Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
        Ok(v) => v,
    };

    Identity::login(&request.extensions(), token.to_owned().into()).unwrap();

    HttpResponse::Ok().body(token)
}

#[post("/logout")]
async fn logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::NoContent()
}

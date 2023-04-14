use actix_identity::{Identity, IdentityMiddleware};

use actix_web::ResponseError;
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

use super::super::application_model::{RouteResponse, RouteResponseOk, RouteResponseError, RouteResponseErrorDefault, RouteResponseErrorCode};




#[get("/user")]
async fn index(
    auth: Option<auth_extractor::AuthExtractor>,
) -> Result<impl Responder> {
    let auth = auth
        .ok_or(RouteResponseErrorDefault("auith infomation not found".to_string()))?;
    
    Ok(RouteResponse::Ok(auth.user_id))
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
    req: HttpRequest,
    param: web::Json<SignupRequest>,
    redis: web::Data<Mutex<RedisProvider>>,
    service_factory: web::Data<ServiceFactory>
) -> Result<impl Responder> {

    let uf = &service_factory.user;

    let mut us = uf.get();

    let signup_request = service_model::SignupRequest{
        username: param.username.to_string(),
        password: param.password.to_string()
    };


    let result = us.signup(signup_request)
        .await
        .map_err(|e| RouteResponseErrorDefault(e.to_string()))?;


    Ok(RouteResponse::Ok(result))

}

#[post("/login")]
async fn login(request: HttpRequest, param: web::Json<routes_model::LoginRequest>,  service_factory: web::Data<ServiceFactory>) -> Result<impl Responder> {
    
    let mut user_service = service_factory.user.get();
    let login_request = service_model::LoginRequest{
        username: param.username.to_string(),
        password: param.password.to_string(),
        user_id: param.user_id.to_string()
    };


    let result = user_service.login(login_request)
        .await
        .map_err(|e| RouteResponseErrorDefault(e.to_string()) )?;


    Identity::login(&request.extensions(), result.token.to_owned().into()).unwrap();
    Ok(RouteResponse::Ok(result))
}

#[post("/logout")]
async fn logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::NoContent()
}

use actix_identity::{Identity, IdentityMiddleware};

use actix_web::ResponseError;
use actix_web::{error, get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder, Result};

use crate::auth;

use crate::app::application_factory::FactoryTrait;
use crate::middleware::auth_extractor;
use crate::app::application_factory::ServiceFactory;
use super::routes_model;
use super::service_model;

use super::super::application_model::{RouteResponse, RouteResponseOk, RouteResponseError, RouteResponseErrorDefault, RouteResponseErrorCode};
use super::service_trait::UserServiceTrait;




#[get("/user")]
async fn index(
    auth: Option<auth_extractor::UserAuthSession>,
) -> Result<impl Responder> {
    let auth = auth
        .ok_or(RouteResponseErrorDefault("auith infomation not found".to_string()))?;
    
    Ok(RouteResponse::Ok(auth))
}


#[post("/verify")]
async fn verify(param: web::Json<routes_model::VerifyRequest>) -> impl Responder {
    let token = &param.token;

    let result = match auth::verify_token(token) {
        Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
        Ok(v) => v,
    };

    HttpResponse::Ok().body("verified")
}


#[post("/signup")]
async fn signup(
    req: HttpRequest,
    param: web::Json<routes_model::SignupRequest>,
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

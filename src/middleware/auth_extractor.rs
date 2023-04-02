use std::pin::Pin;
use actix_web::http::header::HeaderMap;
use futures_util::{Future};
use actix_web::{dev, web, Error, HttpRequest, FromRequest, HttpMessage};
use actix_web::error::{ErrorUnauthorized, InternalError};
use serde::Deserialize;


use actix_identity::{Identity};
use actix::fut::{ready, Ready};
use crate::middleware::util_middleware;
use crate::auth;


#[derive(Debug, Deserialize)]
pub struct AuthExtractor {
    pub user_id: String,
}

impl FromRequest for AuthExtractor {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
       
       
        let identity = match Identity::extract(req).into_inner(){
            Err(e) => None,
            Ok(v) => Some(v)

        };

        let headers = req.headers();        

        let token_claims =  util_middleware::extract_token_cookie_or_header(&identity, headers);

        let token_claims= match token_claims {
            Err(e) => return ready(Err(ErrorUnauthorized(e))),
            Ok(v) => v
        };


        let data: Self = Self {
            user_id: token_claims.sub
        };

        ready(Ok(data))

    }

}
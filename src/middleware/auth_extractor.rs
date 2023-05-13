use actix_web::error::{ErrorUnauthorized};

use actix_web::{dev, Error, FromRequest, HttpMessage, HttpRequest};
use futures_util::Future;
use serde::{Serialize,Deserialize};

use crate::app::application_factory::FactoryTrait;

use crate::middleware::util_middleware;

use actix_identity::Identity;

use crate::app::user::factory::UserFactory;
use crate::app::user::service_trait::UserServiceTrait;



use std::pin::Pin;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct UserAuthSession {
    pub user_id: String,
    pub username: String,
}

impl FromRequest for UserAuthSession {
    type Error = Error;
    // type Future = Ready<Result<Self, Self::Error>>;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        let identity = match Identity::extract(req).into_inner() {
            Err(_e) => None,
            Ok(v) => Some(v),
        };
        let headers = req.headers();

        let token_claims = util_middleware::extract_token_cookie_or_header(&identity, headers);

        Box::pin(async  move{

            let token_claims = token_claims
            .map_err(|e| ErrorUnauthorized(e.to_string()))?;

            let user_factory = UserFactory::new();
            let mut user_service = user_factory.get();

            let user = user_service
                .get_by_id(&token_claims.sub)
                .await
                .map_err(|e| ErrorUnauthorized(e.to_string()))?;

            let data: Self = Self {
                user_id: token_claims.sub,
                username: user.username.to_owned(),
            };

            Ok(data)
        })
    }
}

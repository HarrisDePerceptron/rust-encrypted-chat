
use actix_identity::{Identity, IdentityMiddleware};

use actix_web::{
    get, post, HttpMessage, HttpRequest, HttpResponse, Responder,
};



#[get("/user")]
async fn index(user: Option<Identity>) -> impl Responder {
    if let Some(user) = user {
        format!("Welcome! {}", user.id().unwrap())
    } else {
        "Welcome Anonymous!".to_owned()
    }
}

#[post("/login")]
async fn login(request: HttpRequest) -> impl Responder {
    // Some kind of authentication should happen here -
    // e.g. password-based, biometric, etc.
    // [...]

    // Attached a verified user identity to the active
    // session.
    Identity::login(&request.extensions(), "User1".into()).unwrap();

    HttpResponse::Ok()
}

#[post("/logout")]
async fn logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::NoContent()
}
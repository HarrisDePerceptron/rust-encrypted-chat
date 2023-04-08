
use actix_identity::{Identity, IdentityMiddleware};

use actix_web::{
    get, post, HttpMessage, HttpRequest, HttpResponse, Responder, web, Result, error
};

use std::sync::Mutex;
use redis::AsyncCommands;

use crate::auth;

use crate::utils;

use crate::secrets;
use crate::middleware::auth_extractor;

use serde::{Deserialize, Serialize};

use crate::persistence::redis::RedisProvider;

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


#[derive(Clone, Debug, Deserialize)]
pub struct SignupRequest{
    pub username: String,
    pub password: String,
    
}
#[post("/signup")]
async fn signup(param: web::Json<SignupRequest>, redis: web::Data<Mutex<RedisProvider>>) -> Result<HttpResponse> {
    let username = &param.username;
    let password = &param.password;


    let mut redis = redis.lock()
                    .map_err(|e| error::ErrorBadRequest(e.to_string()))?;
                    

    let conn= redis.get_connection().await.map_err(|e| error::ErrorBadRequest("Redis connection error".to_string()))?;
    

    let user_id = utils::generate_unique_id().map_err(|e|error::ErrorBadRequest(e.to_string()))?;

    let user_key =  format!("user:{}", user_id);
    let user_data = format!("{}:{}", username, password);



    conn.set(user_key.to_string(), user_data).await.map_err(|e|error::ErrorBadRequest(e.to_string()))?;



        // redis::cmd("SET")
        //     .arg(&["key2", "bar"])
        //     .query_async(&mut con)
        //     .await?;

        // let result: String = redis::cmd("MGET")
        //     .arg(&[&user_key])
        //     .query_async(conn)
        //     .await.map_err(|e|error::ErrorBadRequest(e.to_string()))?;

        let result: String = conn.get(&user_key)
                                .await.map_err(|e|error::ErrorBadRequest(e.to_string()))?;


    
    Ok(HttpResponse::Ok().body(format!("signup complete: {}", result)))
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
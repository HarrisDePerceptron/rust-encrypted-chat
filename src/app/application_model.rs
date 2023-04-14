use std::ops::{Deref, DerefMut};
use std::fmt::{Debug, Display, Write};
use actix_web::body::BoxBody;
use serde::{Serialize, Deserialize,de::DeserializeOwned};
use actix_web::{HttpResponse,  Responder};


#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct ApplicationModel<T>
{
    pub id: Option<String>,
    pub data: T

}

pub trait ApplicationModelTrait<T> 
where
    T: Clone + Debug + DeserializeOwned + Serialize + 'static
{
    fn id(&self) -> Option<String>;
    fn data(&self) ->  T;
}

impl<T> ApplicationModelTrait<T> for  ApplicationModel<T>
where 
    T: Clone + Debug + DeserializeOwned + Serialize + 'static
{
    fn id(&self) -> Option<String> {
        self.id.to_owned()
    }

    fn data(&self) ->  T {
        self.data.to_owned()
    }
}

impl<'a, T> Deref for ApplicationModel<T> 
where
    T: Debug + Serialize + Deserialize<'a> + Clone
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}


impl<'a, T> DerefMut for ApplicationModel<T> 
where
    T: Debug + Serialize + Deserialize<'a> + Clone
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}




#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteResponseOk<T>
{
    pub message: String,
    pub code: usize,
    pub data: T    
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteResponseError
{
    pub message: String,
    pub code: usize,
    pub sub_code: Option<String>,  
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RouteResponse<T> {
    Ok(T),
    Error(String)
}




impl Responder for RouteResponseError
{
    type Body = BoxBody;

    fn respond_to(self, req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let response = serde_json::to_string(&self)
            .unwrap_or_else(|e| e.to_string());

        HttpResponse::BadRequest().content_type("application/json").body(response)
        
    }
}


impl<T> Responder for RouteResponseOk<T>
where
    T: Debug + Clone + Serialize
{
    type Body = BoxBody;

    fn respond_to(self, req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let response = serde_json::to_string(&self)
            .map_or_else(|e|{
                let error = RouteResponseError{
                    code: 400,
                    message: e.to_string(),
                    sub_code: Some("DEFAULT".to_string())
                };

                error.respond_to(req)

            }, |v|{
                 HttpResponse::Ok().content_type("application/json").body(v)
            });


        return response;
        
    }
}



impl<T> Responder for RouteResponse<T>
where
    T: Debug + Clone + Serialize
{
    type Body = BoxBody;

    fn respond_to(self, req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        
        match self {
            Self::Ok(v) => {
                let response = RouteResponseOk {
                    code: 200,
                    data: v,
                    message: "success".to_string()
                };
                
                response.respond_to(req)
            },
            Self::Error(v) => {
                let response = RouteResponseError{
                    code: 400,
                    message: "error".to_string(),
                    sub_code: Some("DEFAULT".to_string())
                };

                response.respond_to(req)
            }
        }
        
    }
}



impl<T> Display for RouteResponse<T>
where
    Self: Debug + Serialize
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std:: fmt::Result {
        let response = serde_json::to_string(self)
            .map_err(|e| std::fmt::Error::from(std::fmt::Error))?;

        f.write_str(&response)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteResponseErrorDefault(pub String);


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteResponseErrorCode(pub String, pub String);


impl Display for RouteResponseErrorDefault
where
    Self: Debug + Serialize
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std:: fmt::Result {

        let error_response = RouteResponseError {
            code: 400,
            sub_code: Some("DEFAULT".to_string()),
            message: self.0.to_string()
        };

        let response = serde_json::to_string(&error_response)
            .map_err(|e| std::fmt::Error::from(std::fmt::Error))?;

        f.write_str(&response)
    }
}


impl Display for RouteResponseErrorCode
where
    Self: Debug + Serialize
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std:: fmt::Result {

        let error_response = RouteResponseError {
            code: 400,
            sub_code: Some(self.0.to_string()),
            message: self.1.to_string()
        };

        let response = serde_json::to_string(&error_response)
            .map_err(|e| std::fmt::Error::from(std::fmt::Error))?;

        f.write_str(&response)
    }
}


impl Display for RouteResponseError
where
    Self: Debug + Serialize
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std:: fmt::Result {
        let response = serde_json::to_string(&self)
            .map_err(|e| std::fmt::Error::from(std::fmt::Error))?;

        f.write_str(&response)
    }
}


impl<T> actix_web::ResponseError for RouteResponse<T>
where 
    Self: Debug + Serialize
 {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::BAD_REQUEST
    }

}


impl actix_web::ResponseError for RouteResponseErrorDefault
where 
    Self: Debug + Serialize
 {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::BAD_REQUEST
    }

}


impl actix_web::ResponseError for RouteResponseErrorCode
where 
    Self: Debug + Serialize
 {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::BAD_REQUEST
    }

}


impl actix_web::ResponseError for RouteResponseError
where 
    Self: Debug + Serialize
 {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::BAD_REQUEST
    }

}
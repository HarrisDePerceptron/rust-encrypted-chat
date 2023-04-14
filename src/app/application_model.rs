use std::ops::{Deref, DerefMut};
use std::fmt::{Debug};
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
pub struct ApplicationRouteResponseOk<T>
{
    pub message: String,
    pub code: usize,
    pub data: T    
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationRouteResponseError
{
    pub message: String,
    pub code: usize,
    pub sub_code: Option<usize>,  
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApplicationRouteResponse<T> {
    Ok(ApplicationRouteResponseOk<T>),
    Error(ApplicationRouteResponseError)
}







impl Responder for ApplicationRouteResponseError
{
    type Body = String;

    fn respond_to(self, req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let response = serde_json::to_string(&self)
            .unwrap_or_else(|e| e.to_string());

        HttpResponse::BadRequest().body("").set_body(response)
        
    }
}


impl<T> Responder for ApplicationRouteResponseOk<T>
where
    T: Debug + Clone + Serialize
{
    type Body = String;

    fn respond_to(self, req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let response = serde_json::to_string(&self)
            .map_or_else(|e|{
                let error = ApplicationRouteResponseError{
                    code: 400,
                    message: e.to_string(),
                    sub_code: Some(0)
                };

                error.respond_to(req)

            }, |v|{
                 HttpResponse::Ok().body("").set_body(v)
            });


        return response;
        
    }
}



impl<T> Responder for ApplicationRouteResponse<T>
where
    T: Debug + Clone + Serialize
{
    type Body = String;

    fn respond_to(self, req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        
        match self {
            Self::Ok(v) => v.respond_to(req),
            Self::Error(v) => v.respond_to(req)
        }
        
    }
}
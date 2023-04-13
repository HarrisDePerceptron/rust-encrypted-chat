use std::ops::{Deref, DerefMut};
use std::fmt::{Debug};
use serde::{Serialize, Deserialize,de::DeserializeOwned};


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


use std::ops::{Deref, DerefMut};
use std::fmt::{Debug};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct ApplicationModel<T>
{
    pub id: Option<String>,
    pub data: T

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


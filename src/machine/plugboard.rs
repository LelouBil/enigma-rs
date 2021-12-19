use std::collections::HashMap;
use std::hash::Hash;
use bimap::{BiHashMap, BiMap};
use crate::machine::Transform;

pub struct PlugBoard<'a,T>
 where T: Eq + Hash{
    pub values: &'a Vec<T>,
    pub associations: BiHashMap<&'a T,&'a T>
}

trait ByAny<T>{
    fn remove(&mut self,val: T) -> Option<(T,T)>;
    fn get(&self,val: T) -> Option<&T>;
}

impl<T> ByAny<T> for BiMap<T,T> where T: Eq + Hash{
    fn remove(&mut self, val: T) -> Option<(T,T)> {
        self.remove_by_right(&val)
            .or_else(|| self.remove_by_left(&val))
    }

    fn get(&self, val: T) -> Option<&T> {
        self.get_by_left(&val)
            .or_else(|| self.get_by_right(&val))
    }
}

impl<'a,T> PlugBoard<'a, T> where T: Eq + Hash {
    pub fn add_association(&mut self,left: &'a T, right: &'a T){
        self.associations.insert(left,right);
    }
    pub fn remove_association(&mut self,origin: &'a T){
        self.associations.remove(origin);
    }
}


impl<'a,T> Transform<'a,T> for PlugBoard<'a,T> where T: Eq + Hash {
    fn get_transform(&self, entering: &'a T) -> &'a T {
        self.associations.get(entering).unwrap_or(&entering)
    }
}
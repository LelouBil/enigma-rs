mod plugboard;
mod rotors;

use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
use bimap::BiHashMap;
use plugboard::PlugBoard;
use rotors::Rotor;
use crate::machine::rotors::Reflector;

pub struct EnigmaMachine<'a,T> where T: Eq + Hash{
    pub values: &'a Vec<T>,
    pub reflector: Reflector<'a,T>,
    pub rotors: Vec<Rotor<'a,T>>,
    pub plugboard: PlugBoard<'a,T>
}

impl<'a,T> EnigmaMachine<'a,T> where T: Eq + Hash{
    pub fn new(values: &'a Vec<T>) -> EnigmaMachine<'a,T>{
        EnigmaMachine{
            values,
            reflector: Reflector{values},
            rotors: Vec::new(),
            plugboard: PlugBoard {values: &values, associations: BiHashMap::new()}
        }
    }
    
    pub fn encode(&self,data: Vec<&'a T>) -> Vec<&'a T>{
        let mut output_buffer: Vec<&'a T> = Vec::new();

        for val in data {
            output_buffer.push(self.encode_single(val));
        }
        
        output_buffer
    }
    
    pub fn encode_single(&self,val: &'a T) -> &'a T{
        //todo
        self.plugboard.get_transform(val)
    }
}


trait Transform<'a,T> where T: Eq + Hash {
    fn get_transform(&self,entering: &'a T) -> &'a T;
}
use std::marker::PhantomData;

pub struct Rotor<'a,T>{
    phantom: PhantomData<&'a T>
    //todo
}
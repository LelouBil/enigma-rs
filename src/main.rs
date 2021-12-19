use std::fmt::{Debug, Display};
use std::hash::Hash;
use crate::machine::EnigmaMachine;

mod machine;

#[macro_use]
extern crate lazy_static;

lazy_static! {
static ref ALPHABET : Vec<i32> = vec![1,2,3];
}

fn main() {
    let mut machine = EnigmaMachine::new(&ALPHABET);
    configure(&mut machine);
    encode(vec![1, 2, 3], &machine);
}

fn configure(machine: &mut EnigmaMachine<i32>) {
    machine.plugboard.add_association(&ALPHABET[0], &ALPHABET[1]);
    machine.plugboard.add_association(&ALPHABET[2], &ALPHABET[1]);
}

fn encode<'a, T>(data: Vec<T>, machine: &EnigmaMachine<'a, T>) -> Vec<&'a T> where T: Debug + Eq + Hash {
    let input_data = to_ref_from_alphabet(data, &machine.values);

    println!("Input data is : {:?}", input_data);

    let result = machine.encode(input_data);
    println!("Output is : {:?}", result);
    result
}

fn to_ref_from_alphabet<'a, T>(data: Vec<T>, alphabet: &'a Vec<T>) -> Vec<&'a T> where T: Debug + Eq {
    let mut output_buffer: Vec<&'a T> = Vec::new();
    for val in data {
        let ref_letter = alphabet.iter().find(|elem| elem.eq(&&val))
            .expect(format!("Character {:?} not in alphabet !", val).as_str());
        output_buffer.push(ref_letter);
    }
    output_buffer
}

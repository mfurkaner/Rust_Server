use crate::consts::hash;

pub fn hash_str(input: String) -> u32{
    let mut hash : u32 = hash::FIRST;

    for _chars in input.chars(){
        hash = hash.wrapping_mul(hash::A) ^ u32::from(_chars).wrapping_mul(hash::B);
    }

    return hash;
}
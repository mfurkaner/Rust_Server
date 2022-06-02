use crate::consts::hash;

pub fn hash_str(input: String) -> u64{
    let mut hash : u64 = hash::FIRST;

    for _chars in input.chars(){
        hash = hash.wrapping_mul(hash::A) ^ u64::from(_chars).wrapping_mul(hash::B);
    }

    return hash;
}
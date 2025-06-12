use sha2::{Sha256, Digest};







pub fn crypt_data(data: &str, encryption: bool) -> Option<String>
{
    if encryption {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let res = hasher.finalize();
        let hex: String = format!("{:x}", res);
        Some(hex)
    } else {
        None
    }
}

pub fn hash_func(key_val: i32, gem_val: i32) -> i32
{
    let mut hash_val: i32;
    let magic: i32 = 13;
    
    hash_val = (key_val + gem_val) * magic;
    hash_val = hash_val % 10000;

    return hash_val;
}

fn get_seed(string: &str) -> i32
{
    let mut key_seed: i32 = 0;

    for letter in string.chars() {
        let letter_val: i32 = letter as i32;
        key_seed = key_seed + (letter_val * letter_val);
    }
    return  key_seed;
}

pub fn pre_hash(key: &String, gem: &str) -> i32
{
    let key_val = get_seed(key);
    let gem_val = get_seed(gem);
    let hashed_key: i32 = hash_func(key_val, gem_val);

    return  hashed_key;
}
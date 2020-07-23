use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn generate_name_str(seed: i32) -> String {
    // the seed is coming from the JS side
    let a = seed % (ADJECTIVES.len() as i32);
    let b = seed % (NOUNS.len() as i32);
    [ADJECTIVES[a as usize].to_string(), " ".to_string(), NOUNS[b as usize].to_string()].join("")
}

// 2 adjectives
const ADJECTIVES: [&str; 2] = [
"Aback",
"Abaft",
];

// 2 nouns
const NOUNS: [&str; 3] = [
"Indie",
"Bentleyy",
"Kiwi"
];

// debugging
pub fn main() {
  println!("{:?}", generate_name_str(1));
}
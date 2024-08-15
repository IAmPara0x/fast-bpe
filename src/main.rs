use std::fs;
use std::collections::HashMap;

use serde_json;

mod bpe;
use bpe::bpe_train;

fn main() {
    let text: String = fs::read_to_string("tinyshakespeare.txt").expect("Should have been able to read file");
    let mappings = bpe_train(text, 1024);
    let mappings_inverted: HashMap<u32, (u32,u32)> = mappings.into_iter().map( | (k,v) | (v,k)).collect();
    let mappings_str = serde_json::to_string(&mappings_inverted).unwrap();
    fs::write("mappings.json", mappings_str).expect("Unable to write file");
}

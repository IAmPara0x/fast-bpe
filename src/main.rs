use std::fs;

mod tokenizer;
use tokenizer::bpe_train;

fn main() {

    let text: String = fs::read_to_string("tinyshakespeare.txt").expect("Should have been able to read file");
    let mappings = bpe_train(text, 512);
    println!("{:?}", mappings);
}

use std::fs;
use std::collections::HashMap;
use std::env;
use std::process::exit;

use bpe::bpe_train;
mod bpe;

const USAGE: &str = r#"
USAGE: bpe --text-corpus-file <FILE> --num-merges <NUM> --output-file <FILE>
"#;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("args: {:?}", args);

    if args.len() != 7 {
        eprintln!("Incorrect number of arguments.");
        println!("{USAGE}");
        exit(-1);
    } 

    let mut text_corpus_file: Option<String> = None;
    let mut num_merges: Option<u32> = None;
    let mut output_file: Option<String> = None;

    let mut i = 1; // Start from 1 to skip the program name
    while i < args.len() {
        match args[i].as_str() {
            "--text-corpus-file" => {
                if i + 1 < args.len() {
                    text_corpus_file = Some(args[i + 1].clone());
                } else {
                    eprintln!("Missing value for --text-corpus-file.");
                    println!("{USAGE}");
                    exit(-1);
                }
                i += 1; // Skip the value
            }
            "--num-merges" => {
                if i + 1 < args.len() {
                    match args[i + 1].parse::<u32>() {
                        Ok(n) => num_merges = Some(n),
                        Err(_) => {
                            eprintln!("Invalid number for --num-merges: {}", args[i + 1]);
                            println!("{USAGE}");
                            exit(-1);
                        }
                    }
                } else {
                    eprintln!("Missing value for --num-merges.");
                    println!("{USAGE}");
                    exit(-1);
                }
                i += 1; // Skip the value
            }
            "--output-file" => {
                if i + 1 < args.len() {
                    output_file = Some(args[i + 1].clone());
                } else {
                    eprintln!("Missing value for --output-file.");
                    println!("{USAGE}");
                    exit(-1);
                }
                i += 1; // Skip the value
            }
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                println!("{USAGE}");
                exit(-1);
            }
        }
        i += 1;
    }

    // Ensure all required arguments are present
    let text_corpus_file = text_corpus_file.expect("Missing --text-corpus-file argument.");
    let num_merges = num_merges.expect("Missing --num-merges argument.");
    let output_file = output_file.expect("Missing --output-file argument.");

    // Proceed with the logic using text_corpus, num_merges, and output_file
    println!("Text corpus file: {}", text_corpus_file);
    println!("Number of merges: {}", num_merges);
    println!("Output file: {}", output_file);

    let text: String = fs::read_to_string(text_corpus_file).expect("Should have been able to read file");
    let mappings = bpe_train(text, num_merges);
    let mappings_inverted: HashMap<u32, (u32,u32)> = mappings.into_iter().map( | (k,v) | (v,k)).collect();
    let mappings_str = serde_json::to_string(&mappings_inverted).unwrap();
    fs::write(output_file, mappings_str).expect("Unable to write file");
}

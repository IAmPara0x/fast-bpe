use std::collections::HashMap;
use kdam::tqdm;

use fancy_regex::Regex;

type Mappings = HashMap<(u32, u32), u32>;

fn bpe_step(tokens: Vec<u32>, pairs_freq: &mut Mappings, new_pair: (u32, u32), new_token: u32) -> Vec<u32> {

    let len_text_tokens: usize = tokens.len();

    if len_text_tokens == 0 {
        return vec![];
    }

    let mut updated_tokens = Vec::with_capacity(len_text_tokens); // Preallocate capacity to avoid reallocations
    let mut idx: usize = 0;

    while idx < len_text_tokens - 1 {

        let (current_token, next_token) = (tokens[idx], tokens[idx + 1]);

        if (current_token, next_token) == new_pair {

            updated_tokens.push(new_token);

            if idx + 2 < len_text_tokens {

                let next_pair = (new_token, tokens[idx + 2]);

                *pairs_freq.entry(next_pair).or_insert(0) += 1;
                pairs_freq.entry((next_token, tokens[idx + 2])).and_modify(| c | if *c != 0 { *c -= 1; });
            }

            if 0 < idx {

                let prev_token = tokens[idx - 1];

                let prev_pair = (prev_token, new_token);
                *pairs_freq.entry(prev_pair).or_insert(0) += 1;
                pairs_freq.entry((prev_token, current_token)).and_modify( | c | if *c != 0 { *c -= 1; });
            }
                
            idx += 2

        } else {
            updated_tokens.push(current_token);
            idx += 1;
        }

    }

    if idx != len_text_tokens {
        updated_tokens.push(tokens[idx])
    }

    updated_tokens
}

fn get_all_codepoints_tokens(text: String, pattern: Regex) -> Vec<Vec<u32>> {
    let codepoints: Vec<&str> = pattern.find_iter(&text).map(|m| m.unwrap().as_str()).collect();
    codepoints.into_iter()
              .map(|codepoint| codepoint.bytes().map(|byte| byte as u32).collect())
              .collect()
}


pub fn bpe_train(text: String, target_merges: u32) -> Mappings {

    let token_counter: u32 = 256;
    let pattern = Regex::new(r"(?i)'s|'t|'re|'ve|'m|'ll|'d| ?\p{L}+| ?\p{N}+| ?[^\s\p{L}\p{N}]+|\s+(?!\S)|\s+").unwrap();
    let mut codepoints_tokens = get_all_codepoints_tokens(text, pattern);

    let mut pairs: Vec<(u32,u32)> = Vec::new();
    let mut pairs_freq: Mappings = HashMap::new();

    for tokens in codepoints_tokens.iter() {
        for (&t1,&t2) in tokens.iter().zip(&tokens[1..]) {
            pairs.push((t1,t2));
        }
    }

    for pair in pairs.iter() {
        *pairs_freq.entry(*pair).or_insert(0) += 1;
    }

    let mut mappings: Mappings = HashMap::new();


    for current_token_counter in tqdm!(token_counter..(token_counter + target_merges)) {

        if pairs_freq.is_empty() { break; }

        let (&most_common_pair, &most_common_pair_count) = pairs_freq.iter()
                                                                     .max_by(|a, b| a.1.cmp(&b.1))
                                                                     .unwrap();

        // println!("most_common_pair: {:?}, most_common_pair_count: {:?}", most_common_pair, most_common_pair_count);
        if most_common_pair_count == 1 { break; }

        codepoints_tokens = codepoints_tokens.into_iter().map(| tokens | bpe_step(tokens, &mut pairs_freq, most_common_pair, current_token_counter)).collect();
        mappings.insert(most_common_pair, current_token_counter);
        pairs_freq.remove(&most_common_pair);

    }
    mappings
}

use std::collections::HashMap;
use kdam::tqdm;

use fancy_regex::Regex;

type Mappings = HashMap<(u32, u32), u32>;


fn bpe_step(tokens: &[u32], pairs_freq: &mut Mappings, new_pair: (u32, u32), new_token: u32) -> Vec<u32> {

    let len_text_tokens: usize = tokens.len();

    if len_text_tokens == 0 {
        return Vec::new();
    }


    let mut updated_tokens = Vec::new();
    let mut idx: usize = 0;

    while idx < len_text_tokens - 1 {

        if (tokens[idx], tokens[idx + 1]) == new_pair {

            updated_tokens.push(new_token);

            if idx + 2 < len_text_tokens {
                let next_pair = (new_token, tokens[idx + 2]);

                match pairs_freq.get_mut(&next_pair) {
                    Some(count) => { *count += 1; },
                    _ => { pairs_freq.insert(next_pair, 1); },
                }

                match pairs_freq.get_mut(&(tokens[idx + 1], tokens[idx + 2])) {
                    Some(count) => { *count -= 1; },
                    _ => {},
                }
            }

            if 0 < idx {
                let prev_pair = (tokens[idx - 1], new_token);
                match pairs_freq.get_mut(&prev_pair) {
                    Some(count) => { *count += 1; },
                    _ => { pairs_freq.insert(prev_pair, 1); },
                }

                match pairs_freq.get_mut(&(tokens[idx - 1], tokens[idx])) {
                    Some(count) => { *count -= 1; },
                    _ => {},
                }
            }
                
            idx += 2

        } else {
            updated_tokens.push(tokens[idx]);
            idx += 1;
        }

    }

    if idx != len_text_tokens {
        updated_tokens.push(tokens[idx])
    }

    updated_tokens
}


pub fn bpe_train(text: String, target_merges: u32) -> Mappings {


    let token_counter: u32 = 256;

    let pat = Regex::new(r"(?i)'s|'t|'re|'ve|'m|'ll|'d| ?\p{L}+| ?\p{N}+| ?[^\s\p{L}\p{N}]+|\s+(?!\S)|\s+").unwrap();
    let all_pats: Vec<&str> = pat.find_iter(&text).map(|m| m.unwrap().as_str()).collect();
    let mut pats_tokens: Vec<Vec<u32>> = all_pats.into_iter()
                                                 .map(|pat| pat.chars().map(|c| c as u32).collect())
                                                 .collect();

    let mut pairs: Vec<(u32,u32)> = Vec::new();

    for tokens in pats_tokens.iter() {
        for (&t1,&t2) in tokens.iter().zip(&tokens[1..]) {
            pairs.push((t1,t2));
        }
    }

    let mut pairs_freq: Mappings = HashMap::new();
    let mut mappings: Mappings = HashMap::new();


    for pair in pairs.iter() {
        match pairs_freq.get_mut(pair) {
            Some(count) => { *count += 1; },
            None => { pairs_freq.insert(*pair, 1); },
        }
    }


    for current_token_counter in tqdm!(token_counter..(token_counter + target_merges)) {

        if pairs_freq.is_empty() { break; }

        let (&most_common_pair, &most_common_pair_count) = pairs_freq.iter()
                                                                     .max_by(|a, b| a.1.cmp(&b.1))
                                                                     .unwrap();

        if most_common_pair_count == 1 { break; }

        pats_tokens = pats_tokens.iter().map(| tokens | bpe_step(tokens, &mut pairs_freq, most_common_pair, current_token_counter)).collect();
        mappings.insert(most_common_pair, current_token_counter);
        pairs_freq.remove(&most_common_pair);
    }

    println!("len: {:?}", mappings.len());
    mappings
}

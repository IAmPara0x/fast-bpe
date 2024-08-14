from collections import Counter
from dataclasses import dataclass
from tqdm import tqdm
import regex as re

@dataclass
class Mappings:
    mappings: dict[tuple[int,int], int]

    def __repr__(self) -> str:
        result = ""
        flipped_mappings = dict((v,k) for k,v in self.mappings.items())

        for token_id in flipped_mappings.keys():
            token_text = bytes(Mappings.get_utf8_bytes_from_token(flipped_mappings, token_id)).decode("utf-8", errors="replace")
            result += f"{token_id} -> {token_text.replace(" ", "<spc>")}\n"

        return result

    @staticmethod
    def get_utf8_bytes_from_token(mappings: dict[tuple[int,int], int], token_id) -> list[int]:
        t1, t2 = mappings[token_id]

        if 255 < t1:
            t1_bytes = Mappings.get_utf8_bytes_from_token(mappings, t1)
        else:
            t1_bytes = [t1]
        
        if 255 < t2:
            t2_bytes = Mappings.get_utf8_bytes_from_token(mappings, t2)
        else:
            t2_bytes = [t2]

        return t1_bytes + t2_bytes

def _bpe_step(tokens, pair_freqs, new_pair, new_token):


    update_tokens = []

    len_text_tokens = len(tokens)
    idx = 0
    while idx < len_text_tokens - 1:

        if (tokens[idx], tokens[idx + 1]) == new_pair:

            update_tokens.append(new_token)

            if idx + 2 < len_text_tokens:

                next_pair = (new_token, tokens[idx + 2])
                pair_freqs[next_pair] = pair_freqs.get(next_pair, 0) + 1
                pair_freqs[(tokens[idx + 1], tokens[idx + 2])] -= 1


            if 0 < idx:

                prev_pair = (tokens[idx - 1], new_token)
                pair_freqs[prev_pair] = pair_freqs.get(prev_pair, 0) + 1
                pair_freqs[(tokens[idx - 1], tokens[idx])] -= 1

            idx += 2

        else:
            update_tokens.append(tokens[idx])
            idx += 1
    else:
        if idx != len_text_tokens:
            update_tokens.append(tokens[idx])


    del pair_freqs[new_pair]

    return update_tokens

def bpe_train(text: str, target_merges: int):

    pat = re.compile(r"""(?i)'s|'t|'re|'ve|'m|'ll|'d| ?\p{L}+| ?\p{N}+| ?[^\s\p{L}\p{N}]+|\s+(?!\S)|\s+""")
    all_pats = pat.findall(text)
    pats_tokens = [list(pat.encode("utf-8")) for pat in all_pats]
    token_counter = 256
    target_token_counter = token_counter + target_merges
    mappings = {}

    pairs = []

    for tokens in pats_tokens:
        for pair in zip(tokens, tokens[1:]):
            pairs.append(pair)

    pair_freqs =  Counter(pairs)
    
    for current_token_counter in range(token_counter, target_token_counter + 1):

        if not pair_freqs:
            break

        most_common_pair = max(pair_freqs, key=pair_freqs.get)
        most_common_pair_count = pair_freqs[most_common_pair]
        print(f"{most_common_pair=}, {most_common_pair_count=}")

        if pair_freqs[most_common_pair] == 1:
            break

        pats_tokens = [_bpe_step(tokens, pair_freqs, most_common_pair, current_token_counter) for tokens in pats_tokens]
        mappings[most_common_pair] = current_token_counter

    return Mappings(mappings)

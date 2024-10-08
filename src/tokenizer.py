from collections import Counter
from dataclasses import dataclass
import json

@dataclass
class Tokenizer:
    tokens_dict: dict[int, tuple[int,int] | int]
    bpe_dict: dict[tuple[int,int] | int, int]

    def __repr__(self) -> str:
        result = ""

        for token_id in sorted(self.tokens_dict.keys()):
            token_text = bytes(self.get_utf8_bytes_from_token(token_id)).decode("utf-8", errors="replace")
            result += f"{token_id} -> {token_text.replace(" ", "<spc>")}\n"

        return result

    def get_utf8_bytes_from_token(self, token_id) -> list[int]:

        mapping = self.tokens_dict[token_id]

        if isinstance(mapping, int):
            return [mapping]

        else:

            t1, t2 = mapping
            t1_bytes = self.get_utf8_bytes_from_token(t1)
            t2_bytes = self.get_utf8_bytes_from_token(t2)
            return t1_bytes + t2_bytes



    @staticmethod
    def from_json(path: str) -> "Tokenizer":
        with open(path, "r") as f:
            unparsed_mappings: dict[str, list[int] | int] = json.loads(f.read())


        tokens_dict = {}
        for (k,v) in unparsed_mappings.items():

            if isinstance(v, int):
                tokens_dict[int(k)] = v
            else:
                tokens_dict[int(k)] = tuple(v)
            

        bpe_dict = dict((v,k) for k,v in tokens_dict.items())
            
        return Tokenizer(tokens_dict=tokens_dict, bpe_dict=bpe_dict)


    def encode(self, text: str) -> list[int]:

        text_bytes = list(map(int, text.encode("utf-8")))

        if len(text_bytes) == 1:
            return text_bytes

        while 2 <= len(text_bytes):
            stats = self.get_stats(text_bytes).keys()

            pair = min(stats, key=lambda p: self.bpe_dict.get(p, float('inf')))

            if pair not in self.bpe_dict:
                break

            token_idx = self.bpe_dict[pair]

            update_ids = []
            idx = 0

            while idx < len(text_bytes) - 1:

                if (text_bytes[idx], text_bytes[idx + 1]) == pair:
                    update_ids.append(token_idx)
                    idx += 2
                else:
                    update_ids.append(text_bytes[idx])
                    idx += 1

            else:
                if idx < len(text_bytes):
                    update_ids.append(text_bytes[idx])

            text_bytes = update_ids
        return text_bytes


    @staticmethod
    def get_stats(tokens: list[int]) -> dict[tuple[int,int], int]:

        pairs = []
        for pair in zip(tokens, tokens[1:]):
            pairs.append(pair)
        stats = Counter(pairs)

        return stats


    def decode(self, encodings: list[int]) -> str:

        encodings = list(encodings)
        decoded_bytes = []

        while len(encodings) != 0:

            token = encodings.pop(0)
            mapping = self.tokens_dict.get(token)

            if isinstance(mapping, int) or mapping == None:
                decoded_bytes.append(token)
            else:
                encodings.insert(0, mapping[1])
                encodings.insert(0, mapping[0])

        return bytes(decoded_bytes).decode("utf-8", errors="replace")

from collections import Counter
from dataclasses import dataclass
from tqdm import tqdm
import regex as re
import json

@dataclass
class Tokenizer:
    mappings: dict[int, tuple[int,int]]

    def __repr__(self) -> str:
        result = ""

        for token_id in sorted(self.mappings.keys()):
            token_text = bytes(Tokenizer.get_utf8_bytes_from_token(self.mappings, token_id)).decode("utf-8", errors="replace")
            result += f"{token_id} -> {token_text.replace(" ", "<spc>")}\n"

        return result

    @staticmethod
    def get_utf8_bytes_from_token(mappings: dict[int, tuple[int,int]], token_id) -> list[int]:
        t1, t2 = mappings[token_id]

        if 255 < t1:
            t1_bytes = Tokenizer.get_utf8_bytes_from_token(mappings, t1)
        else:
            t1_bytes = [t1]
        
        if 255 < t2:
            t2_bytes = Tokenizer.get_utf8_bytes_from_token(mappings, t2)
        else:
            t2_bytes = [t2]

        return t1_bytes + t2_bytes


    @staticmethod
    def from_json(path: str) -> "Tokenizer":
        with open(path, "r") as f:
            unparsed_mappings: dict[str, list[int]] = json.loads(f.read())


        mappings = {}
        for (k,v) in unparsed_mappings.items():
            mappings[int(k)] = tuple(v)
            
        return Tokenizer(mappings)

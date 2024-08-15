#!/bin/python3

from src.tokenizer import Tokenizer


if __name__ == "__main__":

    tokenizer = Tokenizer.from_json("./mappings.json")
    print(tokenizer)

    text = "While this seems to be correct, its really good to add an explanation of how it works rather than just the code."
    encoded_text = tokenizer.encode(text)
    print(f"{encoded_text=}, {[bytes(tokenizer.get_utf8_bytes_from_token(token)).decode("utf-8", errors="replace") for token in encoded_text]}")

    decoded_text = tokenizer.decode(encoded_text)
    print(f"{decoded_text=}")
    print(f"{decoded_text==text =}")

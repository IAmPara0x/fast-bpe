#!/bin/python3

from src.tokenizer import Tokenizer


if __name__ == "__main__":

    tokenizer = Tokenizer.from_json("./mappings.json")
    print(tokenizer)

#!/bin/python3

from lib.tokenizer import bpe_train


if __name__ == "__main__":

    with open("tinyshakespeare.txt", "r") as f:
        text = f.readlines()
    text ="".join(text)
    mappings = bpe_train(text, target_merges=64)
    print(mappings.mappings)

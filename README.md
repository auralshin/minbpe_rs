# Tokenizer

> A Rust implementation of [minbpe](https://github.com/karpathy/minbpe).

## Introduction

Tokenizer is a tool that breaks down text into smaller units called tokens. It is commonly used in natural language processing tasks such as text classification, sentiment analysis, and machine translation. This tokenizer is inspired by the [minbpe](https://github.com/karpathy/minbpe) project.


## To Test

```bash
    cargo run -- aaabdaaabac
```
Expected Result
```
    Encoded: [258, 100, 258, 97, 99]
    Decoded: "aaabdaaabac"
```

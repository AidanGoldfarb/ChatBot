CSC200: Chatbot Implementation Project
======================================

This project is an implementation of CSC200's Chatbot Assignment.
Please see the report for details on the design and evaluation.

## Contributors

- Adian Goldfarb (agoldfa7)
- Paul Ouellette (pouellet)
- Rafaello Sanna (rsanna)
- Yiyao Yu (yyu57)

## Overview

There are two separate chatbot implementations provided for this assignment:

- `chatbot-qa`, based on `rust-bert` Q&A pipeline using DistilBERT
- `chatbot-conv`, based on `rust-bert` conversation pipeline using DialoGPT

The reason for using two different models is so that we can provide a
comparison of the two for our report.

## Build

Cargo is required to build the project.
To build both chatbot implementations, run the following command:

    cargo build --release

The following Rust crates will be downloaded:

- `rust-bert`: needed for transformer based models and pipelines
- `regex`: needed for pattern matching in Q&A model implementation
- `levenshtein`: needed for fuzzy matching in Q&A model

## Usage

```
chatbot-qa <database_location>

chatbot-conv <model.ot>
```

For the Q&A chatbot, a database location needs to be specified.
A database is provided in `data/db`.
The conversation chatbot takes a path to the model, which can be downloaded here: 
https://drive.google.com/file/d/1SORixMxmf9Lb8Vus9XB8vFjj4SWQ3Zbe/view?usp=sharing

Running with Cargo:

```sh
# Q&A chatbot:
cargo run --release --bin chatbot-qa data/db

# Conversation chatbot:
cargo run --release --bin chatbot-conv model.ot

# Test the Q&A chatbot on some example queries:
cargo run --release --bin chatbot-qa  data/db < example_queries.txt
```

<> vim: set syntax=markdown:

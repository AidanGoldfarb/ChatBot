CSC200: Chatbot Implementation Project
======================================

This project is an implementation of CSC200's Chatbot Assignment. Please see
report.pdf for more details.

## Contributors
- Adian Goldfarb (agoldfa7)
- Paul Ouellette (pouellet)
- Rafaello Sanna (rsanna)
- Yiyao Yu (yyu57)

## Dependencies
The following external dependencies are required:

- `rust-bert`: needed for transformer based models and pipelines
- `regex`: needed for pattern matching in Q&A model implementation
- `levenshtein`: needed for fuzzy matching in Q&A model

## Usage/Implementations

There are two separate implementations of ChatBot provided for this assignment:
- Q&A Model, based on `rust-bert` Q&A pipeline using DistilBERT
- Conversataion Model, based on `rust-bert` conversation pipeline using DialoGPT

The reason for providing two different models is so that we can provide a
comparison of the two for our report. Please see report.pdf for more details.

The usage of the `target/release/chat_bot` is as follows
```
# Q&A model (chat_bot_qa)
$ ./chat_bot <database_location>

# Conversation model (chat_bot_conv)
$ ./chat_bot <model.ot>
```

For the Q&A model, a database location needs to be specified, an example
database is provided in `data/db`. Similarly, the Conversation model requires
a specified model, which can be downloaded here: 
https://drive.google.com/file/d/1SORixMxmf9Lb8Vus9XB8vFjj4SWQ3Zbe/view?usp=sharing

## Building/Testing
`Cargo` is required to build/run/test the project. To do so, go into the
corresponding crate directory and run the following commands:

```
# To compile binary target/release/chat_bot:
$ cargo build --release

# To run compiled binary directly from cargo:
$ cargo run --release <args>
```

## Known Issues
- The Q&A model can only take one course number at a time, and can only..
respond to questions (not statements). These constraints are due to some..
initial assumptions we made about the problem. See `report.pdf` for details.
- The Conversation model may take some time to respond on some low-end systems..
This is due to it being fairly computation heavy. We have tested that it will ..
eventually generate a reponse.

<> vim: set syntax=markdown:

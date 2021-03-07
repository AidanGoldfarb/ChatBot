use rust_bert::pipelines::text_generation::{TextGenerationConfig, TextGenerationModel};
use rust_bert::resources::{LocalResource, Resource};
use std::env;
use std::path::PathBuf;

fn start_repl(model_path: &str) {
    let config = TextGenerationConfig {
        model_resource: Resource::Local(LocalResource {
            local_path: PathBuf::from(model_path),
        }),
        max_length: 30,
        /*
        model_type: ModelType::GPT2,
        model_resource: Resource::Remote(RemoteResource::from_pretrained(
            Gpt2ModelResources::GPT2_MEDIUM,
        )),
        config_resource: Resource::Remote(RemoteResource::from_pretrained(
            Gpt2ConfigResources::GPT2_MEDIUM,
        )),
        vocab_resource: Resource::Remote(RemoteResource::from_pretrained(
            Gpt2VocabResources::GPT2_MEDIUM,
        )),
        merges_resource: Resource::Remote(RemoteResource::from_pretrained(
            Gpt2MergesResources::GPT2_MEDIUM,
        )),
        min_length: 0,
        max_length: 20,
        do_sample: true,
        early_stopping: true,
        num_beams: 5,
        temperature: 1.0,
        top_k: 0,
        top_p: 0.9,
        repetition_penalty: 1.0,
        length_penalty: 1.0,
        no_repeat_ngram_size: 3,
        num_return_sequences: 1,
        num_beam_groups: None,
        diversity_penalty: None,
        device: Device::cuda_if_available(),
        */
        ..Default::default()
    };

    let model = TextGenerationModel::new(config).unwrap();
    let mut buf = String::new();

    chatbot::print_ted_line("Hi, I'm Ted. How may I help you today?");
    loop {
        let mut ln = String::new();
        std::io::stdin().read_line(&mut ln).unwrap();

        match ln.trim() {
            "(over)" | "(o)" | "." => {
                let trimmed = buf.trim();
                let responses = model.generate(&[trimmed], None);
                if responses.len() > 0 {
                    chatbot::print_ted_line(&responses[0]);
                } else {
                    chatbot::print_ted_line("I didn't quite get that, can you say it again?");
                }
                buf = String::new()
            }
            "(over and out)" | "(oo)" | "bye" => {
                let trimmed = buf.trim();
                if trimmed.len() != 0 {
                    let responses = model.generate(&[trimmed], None);
                    if responses.len() > 0 {
                        chatbot::print_ted_line(&responses[0]);
                    } else {
                        chatbot::print_ted_line("I didn't quite get that, but bye!");
                    }
                } else {
                    chatbot::print_ted_line("Bye!");
                }
                return;
            }
            any => {
                buf += any;
                buf += " "
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <database_dir>", args[0]);
    } else {
        start_repl(&args[1]);
    }
}

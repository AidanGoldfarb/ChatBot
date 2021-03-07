use rust_bert::pipelines::conversation::{
    ConversationConfig, ConversationManager, ConversationModel,
};
use rust_bert::resources::{LocalResource, Resource};
use std::path::PathBuf;
use std::env;

mod lib;

fn start_repl(model_path: &str) {
    let config = ConversationConfig {
        model_resource: Resource::Local(LocalResource {
            local_path: PathBuf::from(model_path),
        }),
        /* config_resource: Resource::Remote(RemoteResource::from_pretrained(
            Gpt2ConfigResources::DIALOGPT_MEDIUM,
        )),
        vocab_resource: Resource::Remote(RemoteResource::from_pretrained(
            Gpt2VocabResources::DIALOGPT_MEDIUM,
        )),
        merges_resource: Resource::Remote(RemoteResource::from_pretrained(
            Gpt2MergesResources::DIALOGPT_MEDIUM,
        )),
        min_length: 0,
        max_length: 1000,
        min_length_for_response: 32,
        do_sample: true,
        early_stopping: false,
        num_beams: 1,
        temperature: 1.0,
        top_k: 50,
        top_p: 0.9,
        repetition_penalty: 1.0,
        length_penalty: 1.0,
        no_repeat_ngram_size: 0,
        num_return_sequences: 1,
        num_beam_groups: None,
        diversity_penalty: None,
        device: Device::cuda_if_available(),
        */
        ..Default::default()
    };

    let conv_model = ConversationModel::new(config).unwrap();
    let mut conv_manager = ConversationManager::new();
    let conv_id = conv_manager.create_empty();
    let mut buf = String::new();

    lib::print_ted_line("Hi, I'm Ted. How may I help you today?");
    loop {
        let mut ln = String::new();
        std::io::stdin().read_line(&mut ln).unwrap();

        match ln.trim() {
            "(over)" | "(o)" | "." => {
                let trimmed = buf.trim();
                let _ = conv_manager.get(&conv_id).unwrap().add_user_input(&trimmed);
                match conv_model
                    .generate_responses(&mut conv_manager)
                    .values()
                    .next()
                {
                    Some(response) => lib::print_ted_line(&response),
                    None => lib::print_ted_line("I didn't quite get that, can you say it again?"),
                }
                buf = String::new()
            }
            "(over and out)" | "(oo)" | "bye" => {
                let trimmed = buf.trim();
                if trimmed.len() != 0 {
                    let _ = conv_manager.get(&conv_id).unwrap().add_user_input(&trimmed);
                    match conv_model
                        .generate_responses(&mut conv_manager)
                        .values()
                        .next()
                    {
                        Some(response) => lib::print_ted_line(&response),
                        None => lib::print_ted_line("I didn't quite get that, but bye!"),
                    }
                } else {
                    lib::print_ted_line("Bye!");
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

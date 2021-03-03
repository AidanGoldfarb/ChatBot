use rust_bert::pipelines::conversation::{ConversationManager, ConversationModel};

mod lib;

fn start_repl() {
    //TODO Replace default with custom trained model
    let conv_model = ConversationModel::new(Default::default()).unwrap();
    let mut conv_manager = ConversationManager::new();
    let mut buf = String::new();

    lib::print_ted_line("Hi, I'm Ted. How may I help you today?");
    loop {
        let mut ln = String::new();
        std::io::stdin().read_line(&mut ln).unwrap();

        match ln.trim() {
            "(over)" | "(o)" | "." => {
                let trimmed = buf.trim();
                conv_manager.create(&trimmed);
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
                // The next line should be the replaced with model interaction
                if trimmed.len() != 0 {
                    conv_manager.create(&trimmed);
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
    start_repl()
}

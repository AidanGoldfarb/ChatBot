use rust_bert::pipelines::conversation::{ConversationModel, ConversationManager};

mod lib;

fn start_repl() {
    lib::print_ted_line("Hi, I'm Ted. How may I help you today?");
    let mut buf = String::new();
    loop {
        let mut ln = String::new();
        std::io::stdin().read_line(&mut ln).unwrap();

        match ln.trim() {
            "(over)" | "(o)" | "." => {
                let trimmed = buf.trim();
                // The next line should be the replaced with model interaction
                lib::print_ted_line(&trimmed);
                buf = String::new()
            },
            "(over and out)" | "(oo)" | "bye" => {
                let trimmed = buf.trim();
                // The next line should be the replaced with model interaction
                lib::print_ted_line(&trimmed);
                return
            },
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

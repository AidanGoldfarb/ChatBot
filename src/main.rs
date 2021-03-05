use std::env;
use std::collections::HashMap;
use rust_bert::pipelines::question_answering::{QaInput, QuestionAnsweringModel};
use regex::Regex;

mod lib;

const CONFIDENCE_THRESHOLD: f64 = 0.0;
const DEFAULT_ENTRY: &str = "default";

fn respond(dbmap: &HashMap<String, String>, question: String, model: &QuestionAnsweringModel) -> Option<String> {
    let re: Regex = Regex::new(r"CSC[ ]?([0-9]{3}[W,H]?)").unwrap();
    let mut ctx_key = String::from("CSC");
    match re.captures(&question) {
        Some(cap) => ctx_key += cap.get(1).unwrap().as_str(),
        None => ctx_key = String::from(DEFAULT_ENTRY)
    }
    let context: String = if dbmap.contains_key(&ctx_key) {
        dbmap.get(&ctx_key).unwrap().to_string()
    } else {
        dbmap.get(&DEFAULT_ENTRY.to_string()).unwrap().to_string()
    };

    let ans = &model.predict(&vec![QaInput{question, context}], 1, 32)[0];
    if ans.len() > 0 && ans[0].score > CONFIDENCE_THRESHOLD {
        Some(ans[0].answer.clone())
    } else {
        None
    }
}

fn start_repl(dbmap: HashMap<String, String>) {
    let qa_model = QuestionAnsweringModel::new(Default::default()).unwrap();
    let mut buf = String::new();

    lib::print_ted_line("Hi, I'm Ted. How may I help you today?");
    loop {
        let mut ln = String::new();
        std::io::stdin().read_line(&mut ln).unwrap();

        match ln.trim() {
            "(over)" | "(o)" | "." => {
                let trimmed = buf.trim();
                match respond(&dbmap, String::from(trimmed), &qa_model) {
                    Some(ans) => lib::print_ted_line(&ans),
                    None => lib::print_ted_line("I don't have a good answer to that question. Wanna ask something else?")
                }
                buf = String::new()
            }
            "(over and out)" | "(oo)" | "bye" => {
                let trimmed = buf.trim();
                // The next line should be the replaced with model interaction
                if trimmed.len() != 0 {
                    match respond(&dbmap, String::from(trimmed), &qa_model) {
                        Some(ans) => lib::print_ted_line(&ans),
                        None => lib::print_ted_line("I don't have a good answer to that question. But bye!")
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
      println!("Usage: {} <database_dir>", args[0]);
    } else {
      start_repl(lib::get_db(&args[1]));
    }
}

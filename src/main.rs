use std::env;
use std::fs::File;
use std::io::Read;
use rust_bert::pipelines::question_answering::{QaInput, QuestionAnsweringModel};

mod lib;

const CONFIDENCE_THRESHOLD: f64 = 0.0;

fn read_context(path: &str) -> String {
    let mut fis = File::open(path).unwrap();
    let mut ctx = String::new();
    let _res = fis.read_to_string(&mut ctx);
    ctx
}

fn start_repl(context_input: String) {
    let qa_model = QuestionAnsweringModel::new(Default::default()).unwrap();
    let mut buf = String::new();

    lib::print_ted_line("Hi, I'm Ted. How may I help you today?");
    loop {
        let mut ln = String::new();
        std::io::stdin().read_line(&mut ln).unwrap();

        match ln.trim() {
            "(over)" | "(o)" | "." => {
                let trimmed = buf.trim();
                let question = String::from(trimmed);
                let context = context_input.clone();
                let ans = &qa_model.predict(&vec![QaInput{ question, context }], 1, 32)[0];
                if ans.len() > 0 && ans[0].score > CONFIDENCE_THRESHOLD {
                    println!("[DEBUG] Confidence: {}", ans[0].score);
                    lib::print_ted_line(&ans[0].answer)
                } else {
                    println!("[DEBUG] Confidence: {}", ans[0].score);
                    lib::print_ted_line("I don't have a good answer to that question. Wanna ask something else?")
                }
                buf = String::new()
            }
            "(over and out)" | "(oo)" | "bye" => {
                let trimmed = buf.trim();
                // The next line should be the replaced with model interaction
                if trimmed.len() != 0 {
                    let question = String::from(trimmed);
                    let context = context_input.clone();
                    let ans = &qa_model.predict(&vec![QaInput{ question, context }], 1, 32)[0];
                    if ans.len() > 0 && ans[0].score > CONFIDENCE_THRESHOLD {
                        lib::print_ted_line(&ans[0].answer)
                    } else {
                        lib::print_ted_line("I don't have a good answer to that question. But bye!")
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
      println!("Usage: {} <context.txt>", args[0]);
    } else {
      start_repl(read_context(&args[1]));
    }
}

use lib::CourseDB;
use regex::Regex;
use rust_bert::pipelines::question_answering::{QaInput, QuestionAnsweringModel};
use std::env;

mod lib;

const CONFIDENCE_THRESHOLD: f64 = 0.0;
const DEFAULT_ENTRY: &str = "default";

fn parse_context(question: &String) -> String {
    let re: Regex = Regex::new(r"CSC[ ]?([0-9]{3}[W,H]?)").unwrap();
    match re.captures(&question) {
        Some(cap) => format!("CSC{}", cap.get(1).unwrap().as_str()),
        None => String::from(DEFAULT_ENTRY),
    }
}

fn respond(
    db: &CourseDB,
    question: String,
    model: &QuestionAnsweringModel,
) -> Option<String> {
    let ctx_key = parse_context(&question);
    let context = db.context_of(ctx_key.as_str());

    let ans = &model.predict(&vec![QaInput { question, context }], 1, 32)[0];
    if ans.len() > 0 && ans[0].score > CONFIDENCE_THRESHOLD {
        Some(ans[0].answer.clone())
    } else {
        None
    }
}

fn start_repl(db: &CourseDB) {
    let qa_model = QuestionAnsweringModel::new(Default::default()).unwrap();
    let mut buf = String::new();

    lib::print_ted_line("Hi, I'm Ted. How may I help you today?");
    loop {
        let mut ln = String::new();
        std::io::stdin().read_line(&mut ln).unwrap();

        match ln.trim() {
            "(over)" | "(o)" | "." => {
                let trimmed = buf.trim();
                match respond(&db, String::from(trimmed), &qa_model) {
                    Some(ans) => lib::print_ted_line(&ans),
                    None => lib::print_ted_line(
                        "I don't have a good answer to that question. Wanna ask something else?",
                    ),
                }
                buf = String::new()
            }
            "(over and out)" | "(oo)" | "bye" => {
                let trimmed = buf.trim();
                // The next line should be the replaced with model interaction
                if trimmed.len() != 0 {
                    match respond(&db, String::from(trimmed), &qa_model) {
                        Some(ans) => lib::print_ted_line(&ans),
                        None => lib::print_ted_line(
                            "I don't have a good answer to that question. But bye!",
                        ),
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
        start_repl(&CourseDB::new(&args[1]));
    }
}

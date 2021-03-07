use levenshtein::levenshtein;
use lib::CourseDB;
use regex::Regex;
use rust_bert::pipelines::question_answering::{QaInput, QuestionAnsweringModel};
use std::env;

mod lib;

const CONFIDENCE_THRESHOLD: f64 = 0.0;

/// Heuristic to see if the word is important
/// currently, it's if it starts with a capital
fn is_important(word: &str) -> bool {
    word.chars().next().unwrap_or('_').is_uppercase()
}

/// Returns the important (capitalized) words from a string
fn important_words(s: &str) -> String {
    s.split_whitespace()
        .filter(|s| is_important(s))
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Returns a vector of keywords found in the string
fn parse_keywords(question: &String) -> Vec<String> {
    // TODO: use BERT tokenizer instead of splitting on whitespace
    let mut kws = Vec::new();
    let clean_q = question.replace(|c: char| !c.is_alphabetic(), " ");
    for word in clean_q.split_whitespace() {
        match word.to_lowercase().as_str() {
            "ai" => {
                kws.push("artificial".to_string());
                kws.push("intelligence".to_string());
            }
            "cs" => {
                kws.push("computer".to_string());
                kws.push("science".to_string());
            }
            "pl" => {
                kws.push("programming".to_string());
                kws.push("languages".to_string());
            }
            "intro" => kws.push("introduction".to_string()),
            _ => kws.push(word.to_lowercase().to_string()),
        }
    }

    kws
}

/// A heuristic to determine if two keywords are similar
fn similar_keywords(kw1: &str, kw2: &str) -> bool {
    let dist = levenshtein(kw1.to_lowercase().as_str(), kw2.to_lowercase().as_str());
    dist < std::cmp::min(kw1.len(), kw2.len()) / 3
}

/// Finds the first course whose every keyword is contained in the question
fn parse_course_titles(question: &String, db: &CourseDB) -> String {
    let question_kws = parse_keywords(question);

    let mut similar = vec![0.0; db.course_titles().len()];

    for (i, (title, _)) in db.course_titles().iter().enumerate() {
        let title_kws = parse_keywords(&important_words(title));

        for tkw in &title_kws {
            for qkw in &question_kws {
                if similar_keywords(qkw, tkw) {
                    similar[i] += 1.0;
                    break;
                }
            }
        }

        similar[i] = similar[i] / (title_kws.len() as f32);
    }

    if similar.is_empty()
        || similar
            .iter()
            .max_by(|f1, f2| f1.partial_cmp(f2).unwrap())
            .unwrap()
            < &0.3
    {
        "".to_string()
    } else {
        let max_idx = similar
            .iter()
            .enumerate()
            .rev()
            .max_by(|(_, &f1), (_, f2)| f1.partial_cmp(f2).unwrap())
            .map(|(i, _)| i)
            .unwrap();
        db.course_titles()[max_idx].1.clone()
    }
}

fn parse_course_numbers(question: &String, _db: &CourseDB) -> String {
    // TODO: don't compile this on every startup
    let re: Regex = Regex::new(r"(?:CSC|CS|csc|cs)? ?([0-9]{3}[W,H]?)").unwrap();
    match re.captures(&question) {
        Some(cap) => format!("CSC{}", cap.get(1).unwrap().as_str()),
        None => "".to_string(),
    }
}

fn respond(db: &CourseDB, question: String, model: &QuestionAnsweringModel) -> Option<String> {
    let cn = parse_course_numbers(&question, &db);
    let ct = parse_course_titles(&question, &db);
    let ctx_key = if cn.is_empty() { ct } else { cn };
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

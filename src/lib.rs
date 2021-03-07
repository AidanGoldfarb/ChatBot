use std::fs::{read_dir, File};
use std::io::Read;
use std::{collections::HashMap, path::Path};

/// Utility: just dumps a file to a string
pub fn dump_file<P: AsRef<Path>>(path: &P) -> String {
    let mut contents = String::new();
    let _ = File::open(path).unwrap().read_to_string(&mut contents);
    contents
}

/// Prints a line in accordance to Ted's format
pub fn print_ted_line(line: &str) {
    println!("> {}", line);
    println!("> (over)")
}

/// The Database of Course Information
pub struct CourseDB {
    context_map: HashMap<String, String>,
    title_map: HashMap<String, String>,
    default_context: String,
}

impl CourseDB {
    /// Create a new course database from a path
    pub fn new(path: &str) -> CourseDB {
        let mut map = HashMap::new();
        let mut defctx = None;
        let mut title_map = None;
        for filepath in read_dir(path).unwrap() {
            let fp = filepath.unwrap().path();
            let name = fp.file_name().unwrap().to_str().unwrap();
            let ctx = dump_file(&fp);

            match name {
                "titles" => {
		    let mut tm = HashMap::new();

		    for line in ctx.lines() {
			let mut split = line.split("|");
			let number = split.next().unwrap();
			let title = split.next().unwrap();
			tm.insert(title.to_string(), number.to_string());
		    }

		    title_map = Some(tm);
		},
                "default" => defctx = Some(ctx),
                _ => {
                    let _ = map.insert(name.to_string(), ctx);
                }
            }
        }

        CourseDB {
            context_map: map,
            title_map: title_map.expect("Database does not contain a title map"),
            default_context: defctx.expect("Database does not contain a default context"),
        }
    }

    /// Returns the context of a given key,
    /// or the default if no context is found for the key
    pub fn context_of(self: &Self, key: &str) -> String {
	match self.context_map.get(key) {
	    Some(context) => context.clone(),
	    None => self.default_context.clone()
	}
    }
}

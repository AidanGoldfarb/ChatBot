use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::io::Read;

/// Prints a line in accordance to Ted's format
pub fn print_ted_line(line: &str) {
    println!("> {}", line);
    println!("> (over)")
}

pub fn get_db(path: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for filepath in read_dir(path).unwrap() {
        let fp = filepath.unwrap().path();
        let mut fis = File::open(&fp).unwrap();
        let mut ctx = String::new();
        let _res = fis.read_to_string(&mut ctx);
        map.insert(
            String::from(fp.file_name().unwrap().to_str().unwrap()),
            ctx.clone(),
        );
    }
    //TODO panic on no default (bad db)
    map
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

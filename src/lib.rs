/// Prints a line in accordance to Ted's format
pub fn print_ted_line(line: &str) {
    println!("> {}", line);
    println!("> (over)")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

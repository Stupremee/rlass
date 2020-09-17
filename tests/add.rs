use rlass::ClassParser;
use std::fs::File;
use std::io::{self, BufReader};

const DATA_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data");

fn open_data(name: &str) -> io::Result<BufReader<File>> {
    let path = format!("{}/{}", DATA_DIR, name);
    let file = File::open(path)?;
    let read = BufReader::new(file);
    Ok(read)
}

#[test]
fn parse_add_java() {
    let data = open_data("Add.class").expect("failed to open data");

    let mut parser = ClassParser::new(data);
    let result = parser.parse().unwrap();
    eprintln!("{:?}", result);
}

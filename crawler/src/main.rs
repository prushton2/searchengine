use curl::easy::Easy;
use curl::easy::WriteError;
use std::str;

fn main() {
    println!("AAA");
    let mut curl = Easy::new();
    curl.url("https://example.com/").unwrap();
    let _ = curl.write_function(process_out).unwrap();
    curl.perform().unwrap();
}

fn process_out(bytes: &[u8]) -> Result<usize, WriteError> {
    let s: &str = match str::from_utf8(bytes) {
        Ok(v) => v,
        Err(_e) => panic!("Not valid utf8")
    };

    println!("string slice: {}\n", s);
    return Ok(bytes.len());
}

use clap::{Arg, Command};
use depot_gbk_rust::detect_encoding_detailed;
use std::fs;
use std::io::{self, Read};

fn main() {
    let matches = Command::new("depot-gbk-rust")
        .version("0.1.0")
        .author("Depot GBK Tool")
        .about("Character encoding detection tool written in Rust")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Input file to detect encoding")
        )
        .arg(
            Arg::new("text")
                .short('t')
                .long("text")
                .value_name("TEXT")
                .help("Text string to detect encoding")
        )
        .get_matches();

    let data = if let Some(filename) = matches.get_one::<String>("file") {
        match fs::read(filename) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading file {}: {}", filename, e);
                return;
            }
        }
    } else if let Some(text) = matches.get_one::<String>("text") {
        text.as_bytes().to_vec()
    } else {
        // Read from stdin if no file or text provided
        let mut buffer = Vec::new();
        match io::stdin().read_to_end(&mut buffer) {
            Ok(_) => buffer,
            Err(e) => {
                eprintln!("Error reading from stdin: {}", e);
                return;
            }
        }
    };

    detect_and_display_encoding(&data);
}

fn detect_and_display_encoding(data: &[u8]) {
    if data.is_empty() {
        println!("No data provided for encoding detection.");
        return;
    }

    // Use the library function for encoding detection
    let result = detect_encoding_detailed(data);
    
    println!("Detected charset: {}", result.charset);
    println!("Confidence: {:.2}%", result.confidence * 100.0);
    println!("Language: {}", result.language);
    println!("Valid UTF-8: {}", if result.is_valid_utf8 { "Yes" } else { "No" });

    // Show sample text if it's valid UTF-8
    if result.is_valid_utf8 {
        if let Ok(utf8_str) = std::str::from_utf8(data) {
            if !utf8_str.trim().is_empty() {
                println!("Sample text: {}", utf8_str.chars().take(50).collect::<String>());
            }
        }
    }

    // Additional GBK detection
    if depot_gbk_rust::is_likely_gbk(data) {
        println!("Likely GBK encoding: Yes");
    }
}

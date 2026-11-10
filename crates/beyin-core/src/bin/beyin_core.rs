use beyin_core::{rank_documents, tokenize, Record};
use std::env;
use std::fs;
use std::io::{self, Read};

fn print_help() {
    println!("beyin-core-cli: Fast Rust retrieval and tokenization utility for avenoxbeyin");
    println!("Usage:");
    println!("  beyin-core tokenize <text>");
    println!("  beyin-core rank <query> <records_json_file_or_->");
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    match args[1].as_str() {
        "tokenize" => {
            if args.len() < 3 {
                eprintln!("Error: text required for tokenize");
                std::process::exit(1);
            }
            let text = &args[2..].join(" ");
            let tokens = tokenize(text);
            println!("{}", serde_json::to_string(&tokens).unwrap());
        }
        "rank" => {
            if args.len() < 4 {
                eprintln!("Error: query and records input required");
                std::process::exit(1);
            }
            let query = &args[2];
            let input_src = &args[3];
            let content = if input_src == "-" {
                let mut buffer = String::new();
                io::stdin().read_to_string(&mut buffer)?;
                buffer
            } else {
                fs::read_to_string(input_src)?
            };

            let records: Vec<Record> = serde_json::from_str(&content)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

            let doc_pairs: Vec<(&str, &str)> = records
                .iter()
                .map(|r| (r.id.as_str(), r.text.as_str()))
                .collect();

            let results = rank_documents(query, &doc_pairs);
            let ranked_ids: Vec<&str> = results.iter().map(|r| r.id).collect();
            println!("{}", serde_json::to_string(&ranked_ids).unwrap());
        }
        "-h" | "--help" | "help" => {
            print_help();
        }
        other => {
            eprintln!("Unknown command: {}", other);
            print_help();
            std::process::exit(1);
        }
    }

    Ok(())
}

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

use regex_engine::{engine, helper::DynError};

fn main() -> Result<(), DynError> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: {} <regex> <file>", args[0]);
        return Err("invalid arguments".into());
    } else {
        match_file(&args[1], &args[2])?;
    }

    Ok(())
}

fn match_file(expr: &str, file: &str) -> Result<(), DynError> {
    let f = File::open(file)?;
    let reader = BufReader::new(f);

    println!("Matching regex: {}", expr);
    println!("");

    for line in reader.lines() {
        let line = line?;
        for (i, _) in line.char_indices() {
            if engine::do_matching(expr, &line[i..], true)? {
                println!("{}", line);
                break;
            }
        }
    }

    Ok(())
}

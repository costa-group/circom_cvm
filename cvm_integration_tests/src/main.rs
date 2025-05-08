use std::env;
use std::fs;
use cvm_parser::parse_program;
use cfg_ssa::type_checking::TypeChecker;
use cfg_ssa::CFGList;

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file.cvm>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    // Read the .cvm file
    let file_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file {}: {}", file_path, err);
            std::process::exit(1);
        }
    };

    // Parse the file
    let parsed_program = match parse_program(&file_content) {
        Ok((_, program)) => program,
        Err(err) => {
            eprintln!("Error parsing file {}: {}", file_path, err);
            std::process::exit(1);
        }
    };

    let mut checker = TypeChecker::new();

    // Typecheck the parsed program
    if let Err(err) = checker.check(&parsed_program) {
        eprintln!("Typechecking failed: {}", err);
        std::process::exit(1);
    }

    // Construct the control flow graph (CFG)
    let cfg = CFGList::new(parsed_program);
    
    // Write the CFG to a JSON file
    let json_output_path = format!("{}.json", file_path);
    if let Err(err) = fs::write(&json_output_path, cfg.to_json()) {
        eprintln!("Error writing JSON file {}: {}", json_output_path, err);
        std::process::exit(1);
    }

    // Write the CFG to a DOT file
    let dot_output_path = format!("{}.dot", file_path);
    if let Err(err) = fs::write(&dot_output_path, cfg.to_dot()) {
        eprintln!("Error writing DOT file {}: {}", dot_output_path, err);
        std::process::exit(1);
    }

    println!("CFG successfully written to {} and {}", json_output_path, dot_output_path);
}
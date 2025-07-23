use std::default;
use std::env;
use std::fs;
use cvm_parser::parse_program;
use cfg_ssa::type_checking::TypeChecker;
use cfg_ssa::CFGList;

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();
    // let default_file_path = "/home/mario/compilados/sum_test_cvm/sum_test.cvm".to_string();
    // let file_path = &default_file_path;

    if args.len() != 2 {
        eprintln!("Usage: {} <file.cvm>", args[0]);
        std::process::exit(1);
    }
    
    let file_path = &args[1];
    let file_no_suffix = file_path.strip_suffix(".cvm").unwrap_or(file_path);

    // Read the .cvm file
    let file_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error at reading file {}: {}", &file_path, err);
            std::process::exit(1);
        }
    };

    // Parse the file
    let parsed_program = match parse_program(&file_content) {
        Ok((_, program)) => program,
        Err(err) => {
            eprintln!("Error at parsing file {}: {}", &file_path, err);
            std::process::exit(1);
        }
    };

    let mut checker = TypeChecker::new();

    // Typecheck the parsed program
    if let Err(err) = checker.check(&parsed_program) {
        eprintln!("Error at typechecking: {}", err);
        std::process::exit(1);
    }

    // Construct the control flow graph (CFG)
    let cfg = CFGList::new(parsed_program);

    if let Err(err) = cfg.check_ssa() {
        eprint!("Error at SSA construction: {}", err);
        std::process::exit(1);
    }

    // Write the CFG to a JSON file
    let json_output_path = format!("{}.json", file_no_suffix);
    if let Err(err) = fs::write(&json_output_path, cfg.to_json()) {
        eprintln!("Error at writing JSON file {}: {}", json_output_path, err);
        std::process::exit(1);
    }

    // Write each CFG to a separate DOT file
    let dot_files = cfg.to_dot();
    for (index, dot_content) in dot_files.iter().enumerate() {
        let dot_output_path = format!("{}_{}.dot", file_no_suffix, index);
        if let Err(err) = fs::write(&dot_output_path, dot_content) {
            eprintln!("Error at writing DOT file {}: {}", dot_output_path, err);
            std::process::exit(1);
        }
    }

    println!("CFGList successfully written to {}", json_output_path);
}

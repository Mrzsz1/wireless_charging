use std::env;
use std::path::PathBuf;

fn main() {
    let arguments = env::args().skip(1).collect::<Vec<_>>();
    let repository = arguments
        .first()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.."));
    let cases = arguments
        .get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| repository.join("evals/qa_real_generator_e2e_cases.json"));
    let output = arguments
        .get(2)
        .map(PathBuf::from)
        .unwrap_or_else(|| repository.join("evals/reports/qa-real-generator-e2e-report.json"));
    let model = arguments
        .get(3)
        .map(String::as_str)
        .unwrap_or("gpt-5.6-luna");
    let effort = arguments.get(4).map(String::as_str).unwrap_or("low");

    match app_lib::run_real_qa_e2e_files(&repository, &cases, &output, model, effort) {
        Ok(true) => println!("Real Answer Generator E2E PASS: {}", output.display()),
        Ok(false) => {
            eprintln!("Real Answer Generator E2E FAIL: {}", output.display());
            std::process::exit(2);
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

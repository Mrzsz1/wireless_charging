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
        .unwrap_or_else(|| repository.join("evals/performance_rag_cases.json"));
    let profile = arguments
        .get(2)
        .map(PathBuf::from)
        .unwrap_or_else(|| repository.join("evals/qa_target_machine.json"));
    let output = arguments
        .get(3)
        .map(PathBuf::from)
        .unwrap_or_else(|| repository.join("evals/reports/performance-latest.json"));
    match app_lib::run_rag_performance_files(&repository, &cases, &profile, &output) {
        Ok(true) => println!("Reranker performance benchmark PASS: {}", output.display()),
        Ok(false) => {
            eprintln!("Reranker performance benchmark FAIL: {}", output.display());
            std::process::exit(2);
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

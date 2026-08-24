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
        .unwrap_or_else(|| repository.join("evals/rag_retrieval_cases.json"));
    let json = arguments
        .get(2)
        .map(PathBuf::from)
        .unwrap_or_else(|| repository.join("evals/reports/rag-evaluation-latest.json"));
    let markdown = arguments
        .get(3)
        .map(PathBuf::from)
        .unwrap_or_else(|| repository.join("evals/reports/rag-evaluation-latest.md"));

    match app_lib::run_rag_evaluation_files(&repository, &cases, &json, &markdown) {
        Ok(true) => {
            println!("RAG evaluation PASS: {}", markdown.display());
        }
        Ok(false) => {
            eprintln!("RAG evaluation REVIEW: {}", markdown.display());
            std::process::exit(2);
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

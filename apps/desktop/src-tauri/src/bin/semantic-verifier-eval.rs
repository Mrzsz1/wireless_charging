use std::env;
use std::path::PathBuf;

fn main() {
    let arguments = env::args().skip(1).collect::<Vec<_>>();
    let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let provider = arguments
        .first()
        .cloned()
        .unwrap_or_else(|| "codex-subscription".to_string());
    let model = arguments
        .get(1)
        .cloned()
        .unwrap_or_else(|| "gpt-5.6-luna".to_string());
    let effort = arguments
        .get(2)
        .cloned()
        .unwrap_or_else(|| "low".to_string());
    let cases = arguments
        .get(3)
        .map(PathBuf::from)
        .unwrap_or_else(|| repository.join("evals/semantic_verification_real_cases.json"));
    let output = arguments
        .get(4)
        .map(PathBuf::from)
        .unwrap_or_else(|| repository.join("evals/reports/semantic-verifier-real-latest.json"));
    match app_lib::run_semantic_benchmark_files(&cases, &output, &provider, &model, &effort) {
        Ok(true) => println!(
            "Semantic verifier real benchmark PASS: {}",
            output.display()
        ),
        Ok(false) => {
            eprintln!(
                "Semantic verifier real benchmark FAIL: {}",
                output.display()
            );
            std::process::exit(2);
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

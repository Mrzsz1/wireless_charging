use std::env;
use std::path::PathBuf;

fn main() {
    let arguments = env::args().skip(1).collect::<Vec<_>>();
    let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let cases = arguments
        .first()
        .map(PathBuf::from)
        .unwrap_or_else(|| repository.join("evals/production_conversation_cases.json"));
    let output = arguments
        .get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| repository.join("evals/reports/conversation-evaluation-latest.json"));
    match app_lib::run_conversation_benchmark_files(&cases, &output) {
        Ok(true) => println!(
            "Conversation production evaluation PASS: {}",
            output.display()
        ),
        Ok(false) => {
            eprintln!(
                "Conversation production evaluation FAIL: {}",
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

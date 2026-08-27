use std::env;
use std::path::PathBuf;

fn main() {
    let arguments = env::args().skip(1).collect::<Vec<_>>();
    let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let cases = arguments
        .first()
        .map(PathBuf::from)
        .unwrap_or_else(|| repository.join("evals/conversation_state_v2_cases.json"));
    let output = arguments
        .get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| repository.join("evals/reports/conversation-state-v2-report.json"));
    match app_lib::run_conversation_state_benchmark_files(&cases, &output) {
        Ok(true) => println!(
            "Conversation state v2 evaluation PASS: {}",
            output.display()
        ),
        Ok(false) => {
            eprintln!(
                "Conversation state v2 evaluation FAIL: {}",
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

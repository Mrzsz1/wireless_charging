use std::env;
use std::path::PathBuf;

fn main() {
    let arguments = env::args().skip(1).collect::<Vec<_>>();
    let repository = arguments
        .first()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.."));
    let probe_id = arguments.get(1).map(String::as_str).unwrap_or("a");
    let output = arguments.get(2).map(PathBuf::from).unwrap_or_else(|| {
        repository.join(format!(
            "evals/reports/qa-codex-planner-probe-{probe_id}.json"
        ))
    });
    let model = arguments
        .get(3)
        .map(String::as_str)
        .unwrap_or("gpt-5.6-luna");
    let effort = arguments.get(4).map(String::as_str).unwrap_or("low");

    match app_lib::run_planner_probe_files(&repository, probe_id, &output, model, effort) {
        Ok(true) => println!(
            "Planner Provider Probe {probe_id} PASS: {}",
            output.display()
        ),
        Ok(false) => {
            eprintln!(
                "Planner Provider Probe {probe_id} FAIL: {}",
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

use app_lib::HeldoutRunOptions;
use std::collections::BTreeMap;
use std::env;
use std::path::PathBuf;

fn parse_arguments() -> Result<HeldoutRunOptions, String> {
    let mut values = BTreeMap::new();
    let mut arguments = env::args().skip(1);
    while let Some(argument) = arguments.next() {
        let Some(raw_key) = argument.strip_prefix("--") else {
            return Err(format!(
                "HELDOUT_CLI_INVALID: unexpected argument {argument}"
            ));
        };
        let (key, value) = if let Some((key, value)) = raw_key.split_once('=') {
            (key.to_string(), value.to_string())
        } else {
            let value = arguments
                .next()
                .ok_or_else(|| format!("HELDOUT_CLI_INVALID: --{raw_key} requires a value"))?;
            (raw_key.to_string(), value)
        };
        if !matches!(
            key.as_str(),
            "dataset" | "output-dir" | "repository" | "provider" | "model" | "reasoning-effort"
        ) || values.insert(key.clone(), value).is_some()
        {
            return Err(format!("HELDOUT_CLI_INVALID: unknown or duplicate --{key}"));
        }
    }
    let required_path = |name: &str| {
        values
            .get(name)
            .filter(|value| !value.trim().is_empty())
            .map(PathBuf::from)
            .ok_or_else(|| format!("HELDOUT_CLI_INVALID: --{name} is required"))
    };
    Ok(HeldoutRunOptions {
        dataset: required_path("dataset")?,
        output_dir: required_path("output-dir")?,
        repository: required_path("repository")?,
        provider: values
            .get("provider")
            .cloned()
            .unwrap_or_else(|| "codex-subscription".to_string()),
        model: values.get("model").cloned().unwrap_or_default(),
        reasoning_effort: values.get("reasoning-effort").cloned().unwrap_or_default(),
    })
}

fn main() {
    let result = parse_arguments().and_then(app_lib::run_independent_heldout);
    match result {
        Ok(path) => println!("Independent held-out run complete: {}", path.display()),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

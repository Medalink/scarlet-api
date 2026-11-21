use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("check-groups") => {
            let Some(path) = args.next() else {
                eprintln!("usage: scarlet-part check-groups <FORMAT.md>");
                return ExitCode::from(2);
            };
            let text = match std::fs::read_to_string(&path) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("scarlet-part: {path}: {e}");
                    return ExitCode::from(2);
                }
            };
            let problems = scarlet_part::groups::check(&text);
            if problems.is_empty() {
                println!(
                    "groups ok: {} codes",
                    scarlet_part::groups::format_codes(&text).len()
                );
                return ExitCode::SUCCESS;
            }
            for p in &problems {
                println!("{p}");
            }
            ExitCode::FAILURE
        }
        Some("groups") => {
            println!(
                "{}",
                serde_json::to_string_pretty(&scarlet_part::groups::json()).unwrap_or_default()
            );
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("usage: scarlet-part <check-groups|groups> ...");
            ExitCode::from(2)
        }
    }
}

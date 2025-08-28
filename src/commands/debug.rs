use rs_claude_bar::claude_types::TranscriptEntry;
use std::fs;
use std::path::Path;

pub fn run(config: &rs_claude_bar::ConfigInfo) {
    let base_path = format!("{}/projects", config.claude_data_path);
    let path = Path::new(&base_path);

    if !path.exists() {
        eprintln!("Path does not exist: {}", base_path);
        return;
    }

    println!("=== DEBUG: Parsing JSONL files in {} ===", base_path);

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let folder_name = entry.file_name();
                println!("\n📁 FOLDER: {:?}", folder_name);

                if let Ok(files) = fs::read_dir(entry.path()) {
                    for file in files.flatten() {
                        if file.path().extension().and_then(|s| s.to_str()) == Some("jsonl") {
                            let file_name = file.file_name();
                            println!("  📄 FILE: {:?}", file_name);

                            if let Ok(content) = fs::read_to_string(file.path()) {
                                for (line_num, line) in content.lines().enumerate() {
                                    if line.trim().is_empty() {
                                        continue;
                                    }

                                    // Try to parse as TranscriptEntry
                                    match serde_json::from_str::<TranscriptEntry>(line) {
                                        Ok(entry) => {
                                            println!(
                                                "    Line {}: ✅ TranscriptEntry",
                                                line_num + 1
                                            );
                                            println!("      -> Full Object: {:#?}", entry);
                                        }
                                        Err(parse_error) => {
                                            // Show detailed parsing error
                                            println!("    Line {}: ❌ Failed to parse as TranscriptEntry", line_num + 1);
                                            println!("      -> Parse Error: {}", parse_error);

                                            // Try to show what fields are present vs expected
                                            if let Ok(json_obj) =
                                                serde_json::from_str::<serde_json::Value>(line)
                                            {
                                                if let Some(obj) = json_obj.as_object() {
                                                    let present_keys: Vec<&String> =
                                                        obj.keys().collect();
                                                    println!(
                                                        "      -> Present keys: {:?}",
                                                        present_keys
                                                    );

                                                    // Check for common expected fields
                                                    let expected_keys = [
                                                        "parentUuid",
                                                        "isSidechain",
                                                        "userType",
                                                        "cwd",
                                                        "sessionId",
                                                        "version",
                                                        "gitBranch",
                                                        "type",
                                                        "uuid",
                                                        "timestamp",
                                                        "message", // Optional fields: "isApiErrorMessage", "isMeta", "costUSD"
                                                    ];

                                                    let missing_keys: Vec<&str> = expected_keys
                                                        .iter()
                                                        .filter(|&&key| !obj.contains_key(key))
                                                        .copied()
                                                        .collect();

                                                    if !missing_keys.is_empty() {
                                                        println!(
                                                            "      -> Missing expected keys: {:?}",
                                                            missing_keys
                                                        );
                                                    }

                                                    let extra_keys: Vec<&String> = present_keys
                                                        .iter()
                                                        .filter(|key| {
                                                            !expected_keys.contains(&key.as_str())
                                                        })
                                                        .copied()
                                                        .collect();

                                                    if !extra_keys.is_empty() {
                                                        println!(
                                                            "      -> Extra keys: {:?}",
                                                            extra_keys
                                                        );
                                                    }
                                                } else {
                                                    println!("      -> Not a JSON object");
                                                }
                                            } else {
                                                println!("      -> Invalid JSON");
                                            }
                                            println!(
                                                "      -> Line content (first 200 chars): {}",
                                                line.chars().take(200).collect::<String>()
                                            );
                                        }
                                    }
                                }
                            } else {
                                println!("  ❌ Could not read file: {:?}", file_name);
                            }
                        }
                    }
                }
            }
        }
    } else {
        eprintln!("Could not read directory: {}", base_path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_does_not_panic() {
        let config = rs_claude_bar::ConfigInfo::default();
        run(&config);
    }
}

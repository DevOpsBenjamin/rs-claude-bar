use std::io::{self, Write};
use crate::{
    claudebar_types::config::ConfigInfo,
    display::status_config::{
        MetricRegistry, StatusLineConfig, DisplayItem, generate_format_example_mock
    },
    common::colors::*,
};

pub fn run(config: &ConfigInfo) {
    match run_interactive_config(config) {
        Ok(_) => {},
        Err(e) => eprintln!("Error configuring display: {}", e),
    }
}

pub fn run_interactive_config(config: &ConfigInfo) -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 {bold}{cyan}Configure your Claude Code status line{reset}\n", 
        bold = BOLD, cyan = CYAN, reset = RESET);
    
    let registry = MetricRegistry::new();
    let mut status_config = load_or_default_status_config(config)?;
    
    for metric in registry.all_metrics() {
        configure_metric(&mut status_config, metric)?;
    }
    
    // Configure separator
    configure_separator(&mut status_config)?;
    
    // Preview and save
    println!("\n{bold}📊 Preview of your status line:{reset}", bold = BOLD, reset = RESET);
    let preview = generate_status_line_preview(&status_config);
    println!("   {}", preview);
    
    println!();
    if confirm("💾 Save this configuration? [Y/n]")? {
        save_status_config(config, &status_config)?;
        println!("✅ Status line configuration saved!");
        println!("   Use `rs-claude-bar prompt` to see your customized status line.");
    } else {
        println!("❌ Configuration discarded.");
    }
    
    Ok(())
}

fn configure_metric(
    config: &mut StatusLineConfig,
    metric: &crate::display::status_config::MetricDefinition
) -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 {bold}{metric_name}{reset}", 
        bold = BOLD, 
        metric_name = metric.name, 
        reset = RESET);
    println!("   {}", metric.description);
    
    // Show all supported formats with examples
    println!("   Available formats:");
    for (i, format) in metric.supported_formats.iter().enumerate() {
        let example = generate_format_example_mock(metric.stat_type.clone(), format);
        let is_default = format == &metric.default_format;
        let default_marker = if is_default { " (default)" } else { "" };
        
        println!("   {}) {:12} - {}{}", 
            i + 1, 
            format_name(format), 
            example,
            default_marker
        );
    }
    
    // Ask user to choose format or skip
    let choice = get_format_choice(metric)?;
    
    if let Some(selected_format) = choice {
        config.items.push(DisplayItem {
            stat_type: metric.stat_type.clone(),
            format: selected_format,
            enabled: true,
        });
        
        let example = generate_format_example_mock(metric.stat_type.clone(), &selected_format);
        println!("   ✅ Added: {}", example);
    } else {
        println!("   ⏭️  Skipped");
    }
    
    println!();
    Ok(())
}

fn configure_separator(config: &mut StatusLineConfig) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔗 {bold}Separator{reset}", bold = BOLD, reset = RESET);
    println!("   Choose what goes between each item:");
    println!("   1) \" | \"     (default)");
    println!("   2) \" • \"");
    println!("   3) \" · \"");
    println!("   4) \" → \"");
    println!("   5) Custom");
    
    print!("   Choice [1-5]: ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();
    
    config.separator = match input {
        "2" => " • ".to_string(),
        "3" => " · ".to_string(), 
        "4" => " → ".to_string(),
        "5" => {
            print!("   Enter custom separator: ");
            io::stdout().flush()?;
            let mut custom = String::new();
            io::stdin().read_line(&mut custom)?;
            custom.trim().to_string()
        },
        _ => " | ".to_string(),
    };
    
    println!("   ✅ Separator: \"{}\"", config.separator);
    println!();
    Ok(())
}

fn get_format_choice(metric: &crate::display::status_config::MetricDefinition) -> Result<Option<crate::display::status_config::DisplayFormat>, Box<dyn std::error::Error>> {
    let default_choice = if metric.enabled_by_default {
        metric.supported_formats.iter()
            .position(|f| f == &metric.default_format)
            .map(|i| i + 1)
            .unwrap_or(1)
    } else {
        0 // Skip by default
    };
    
    let prompt = if metric.enabled_by_default {
        format!("   Choice [1-{}, or 0 to skip]: ", metric.supported_formats.len())
    } else {
        format!("   Choice [1-{}, or 0 to skip] (0=default): ", metric.supported_formats.len())
    };
    
    print!("{}", prompt);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();
    
    let choice: usize = if input.is_empty() {
        default_choice
    } else {
        input.parse().unwrap_or(default_choice)
    };
    
    if choice == 0 {
        Ok(None)
    } else if choice <= metric.supported_formats.len() {
        Ok(Some(metric.supported_formats[choice - 1].clone()))
    } else {
        Ok(Some(metric.default_format.clone()))
    }
}

fn format_name(format: &crate::display::status_config::DisplayFormat) -> &'static str {
    use crate::display::status_config::DisplayFormat;
    match format {
        DisplayFormat::Text => "Text",
        DisplayFormat::TextWithEmoji => "With Emoji",
        DisplayFormat::Compact => "Compact", 
        DisplayFormat::ProgressBar => "Progress Bar",
        DisplayFormat::PercentageOnly => "Percentage",
        DisplayFormat::Ratio => "Ratio",
        DisplayFormat::Duration => "Duration",
        DisplayFormat::DurationShort => "Short Time",
        DisplayFormat::StatusIcon => "Icon Only",
        DisplayFormat::StatusText => "Text Status",
        DisplayFormat::StatusColored => "Colored",
        DisplayFormat::Hidden => "Hidden",
    }
}

fn confirm(prompt: &str) -> Result<bool, Box<dyn std::error::Error>> {
    print!("{}", prompt);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();
    
    Ok(matches!(input.as_str(), "" | "y" | "yes"))
}

fn load_or_default_status_config(_config: &ConfigInfo) -> Result<StatusLineConfig, Box<dyn std::error::Error>> {
    // TODO: Load from ~/.claude-bar/status_config.json
    Ok(StatusLineConfig::default())
}

fn save_status_config(_config: &ConfigInfo, _status_config: &StatusLineConfig) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Save to ~/.claude-bar/status_config.json
    Ok(())
}

fn generate_status_line_preview(config: &StatusLineConfig) -> String {
    let previews: Vec<String> = config.items.iter()
        .filter(|item| item.enabled)
        .map(|item| generate_format_example_mock(item.stat_type.clone(), &item.format))
        .collect();
    
    if previews.is_empty() {
        "💤 No items configured".to_string()
    } else {
        previews.join(&config.separator)
    }
}
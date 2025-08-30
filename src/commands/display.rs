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
    
    loop {
        show_main_menu(&status_config)?;
        
        let choice = get_menu_choice(1, 5)?;
        
        match choice {
            1 => add_item_interactive(&mut status_config, &registry)?,
            2 => remove_item_interactive(&mut status_config)?,
            3 => configure_separator(&mut status_config)?,
            4 => {
                save_status_config(config, &status_config)?;
                println!("✅ Status line configuration saved!");
                println!("   Use `rs-claude-bar prompt` to see your customized status line.");
                break;
            },
            5 => {
                println!("❌ Configuration discarded.");
                break;
            },
            _ => continue,
        }
    }
    
    Ok(())
}

fn show_main_menu(config: &StatusLineConfig) -> Result<(), Box<dyn std::error::Error>> {
    // Clear console
    print!("\x1b[2J\x1b[1;1H");
    
    println!("🎨 {bold}{cyan}Configure your Claude Code status line{reset}\n", 
        bold = BOLD, cyan = CYAN, reset = RESET);
    
    println!("{bold}Current Status Line:{reset} {}", 
        generate_status_line_preview(config),
        bold = BOLD, 
        reset = RESET
    );
    println!("{bold}Separator:{reset} \"{}\"", config.separator, bold = BOLD, reset = RESET);
    
    println!("\n{bold}Main Menu:{reset}", bold = BOLD, reset = RESET);
    println!("1) 📊 Add Item");
    println!("2) 🗑️  Remove Item");
    println!("3) 🔄 Change Separator");
    println!("4) 💾 Save & Exit");
    println!("5) ❌ Exit without saving");
    println!();
    
    Ok(())
}

fn add_item_interactive(
    config: &mut StatusLineConfig,
    registry: &MetricRegistry
) -> Result<(), Box<dyn std::error::Error>> {
    // Clear console for clean interface
    print!("\x1b[2J\x1b[1;1H");
    
    println!("📊 {bold}Add New Status Item{reset}", bold = BOLD, reset = RESET);
    
    // Get all available metrics (allow duplicates with different formats)
    let available_metrics = registry.all_metrics();
    
    println!("\n{bold}Available Items:{reset}", bold = BOLD, reset = RESET);
    for (i, metric) in available_metrics.iter().enumerate() {
        let example = generate_format_example_mock(metric.stat_type.clone(), &metric.default_format);
        println!("{}) {:16} - {}", i + 1, metric.name, example);
    }
    
    println!();
    let choice = get_menu_choice(0, available_metrics.len())?;
    
    if choice == 0 {
        return Ok(());
    }
    
    let selected_metric = &available_metrics[choice - 1];
    configure_item_format(config, selected_metric)?;
    
    Ok(())
}

fn configure_item_format(
    config: &mut StatusLineConfig,
    metric: &crate::display::status_config::MetricDefinition
) -> Result<(), Box<dyn std::error::Error>> {
    // Clear console for clean interface
    print!("\x1b[2J\x1b[1;1H");
    
    println!("📊 {bold}{}{reset}", metric.name, bold = BOLD, reset = RESET);
    println!("   {}", metric.description);
    
    println!("\n   {bold}Available formats:{reset}", bold = BOLD, reset = RESET);
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
    
    println!();
    let choice = get_menu_choice(1, metric.supported_formats.len())?;
    let selected_format = metric.supported_formats[choice - 1].clone();
    
    config.items.push(DisplayItem {
        stat_type: metric.stat_type.clone(),
        format: selected_format.clone(),
        enabled: true,
    });
    
    
    Ok(())
}

fn remove_item_interactive(config: &mut StatusLineConfig) -> Result<(), Box<dyn std::error::Error>> {
    // Clear console for clean interface
    print!("\x1b[2J\x1b[1;1H");
    
    if config.items.is_empty() {
        return Ok(());
    }
    
    println!("🗑️  {bold}Remove Status Item{reset}", bold = BOLD, reset = RESET);
    println!("\n{bold}Current Items:{reset}", bold = BOLD, reset = RESET);
    
    for (i, item) in config.items.iter().enumerate() {
        let example = generate_format_example_mock(item.stat_type.clone(), &item.format);
        println!("{}) {}", i + 1, example);
    }
    
    println!();
    let choice = get_menu_choice(0, config.items.len())?;
    
    if choice == 0 {
        return Ok(());
    }
    
    config.items.remove(choice - 1);
    
    Ok(())
}


fn get_menu_choice(min: usize, max: usize) -> Result<usize, Box<dyn std::error::Error>> {
    loop {
        print!("Choice [{}-{}]: ", min, max);
        io::stdout().flush()?;
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => return Err("EOF reached".into()), // No input available
            Ok(_) => {
                let input = input.trim();
                if let Ok(choice) = input.parse::<usize>() {
                    if choice >= min && choice <= max {
                        return Ok(choice);
                    }
                }
                println!("   Invalid choice, try again.");
            }
            Err(e) => return Err(e.into()),
        }
    }
}

fn configure_separator(config: &mut StatusLineConfig) -> Result<(), Box<dyn std::error::Error>> {
    // Clear console for clean interface
    print!("\x1b[2J\x1b[1;1H");
    
    println!("🔗 {bold}Change Separator{reset}", bold = BOLD, reset = RESET);
    println!("\n   Choose what goes between each item:");
    println!("   1) \" | \"     (default)");
    println!("   2) \" • \"");
    println!("   3) \" · \"");
    println!("   4) \" → \"");
    println!("   5) Custom");
    
    println!();
    let choice = get_menu_choice(1, 5)?;
    
    config.separator = match choice {
        2 => " • ".to_string(),
        3 => " · ".to_string(), 
        4 => " → ".to_string(),
        5 => {
            print!("   Enter custom separator: ");
            io::stdout().flush()?;
            let mut custom = String::new();
            io::stdin().read_line(&mut custom)?;
            custom.trim().to_string()
        },
        _ => " | ".to_string(),
    };
    
    
    Ok(())
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
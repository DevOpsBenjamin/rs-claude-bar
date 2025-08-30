use std::io::{self, Write};

use crate::{
    common::colors::*, 
    config::{
        utils::{
            MetricDefinition, 
            MetricRegistry
        },
        ConfigManager,
        DisplayFormat,
        DisplayItem,
        StatusLineConfig
    },
    display::prompt::{
        PromptData,
        generate_status_line,
    },
    display::generate_stat_with_format, 
};

pub fn run_display_config(config_manager: &mut ConfigManager, data: &PromptData) {
    let mut prompt_config: &mut StatusLineConfig = &mut config_manager.config.display;

    let registry = MetricRegistry::new();    
    loop {
        show_main_menu(data,&prompt_config);
        
        let choice = get_menu_choice(1, 5);
        
        match choice {
            1 => add_item_interactive(data, &mut prompt_config, &registry),
            2 => remove_item_interactive(data, &mut prompt_config),
            3 => configure_separator(&mut prompt_config),
            4 => {
                config_manager.save_config();
                break;
            },
            5 => {
                println!("❌ Configuration discarded.");
                break;
            },
            _ => continue,
        }
    }
}

fn show_main_menu(
    data: &PromptData,
    config: &StatusLineConfig
) {
    // Clear console
    print!("\x1b[2J\x1b[1;1H");
    
    println!("🎨 {bold}{cyan}Configure your Claude Code status line{reset}\n", 
        bold = BOLD, cyan = CYAN, reset = RESET);
    
    println!("{bold}Current Status Line:{reset} {}", 
        generate_status_line(data,config),
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
}

fn add_item_interactive(
    data: &PromptData,
    prompt_config: &mut StatusLineConfig,
    registry: &MetricRegistry
) {
    // Clear console for clean interface
    print!("\x1b[2J\x1b[1;1H");
    
    println!("📊 {bold}Add New Status Item{reset}", bold = BOLD, reset = RESET);
    
    // Get all available metrics (allow duplicates with different formats)
    let available_metrics = registry.all_metrics();
    
    println!("\n{bold}Available Items:{reset}", bold = BOLD, reset = RESET);
    for (i, metric) in available_metrics.iter().enumerate() {
        let example = generate_stat_with_format(&data, &metric.stat_type, &metric.default_format);
        println!("{}) {:16} - {}", i + 1, metric.name, example);
    }    
    println!();
    let choice = get_menu_choice(0, available_metrics.len());
    
    if choice == 0 {
        return;
    }
    
    let selected_metric = &available_metrics[choice - 1];
    configure_item_format(data, prompt_config, selected_metric);
}

fn configure_item_format(
    data: &PromptData,
    prompt_config: &mut StatusLineConfig,
    metric: &MetricDefinition,
) {
    // Clear console for clean interface
    print!("\x1b[2J\x1b[1;1H");
    
    println!("📊 {bold}{}{reset}", metric.name, bold = BOLD, reset = RESET);
    println!("   {}", metric.description);
    
    println!("\n   {bold}Available formats:{reset}", bold = BOLD, reset = RESET);
    for (i, format) in metric.supported_formats.iter().enumerate() {
        let example = generate_stat_with_format(&data, &metric.stat_type, format);
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
    let choice = get_menu_choice(1, metric.supported_formats.len());
    let selected_format = metric.supported_formats[choice - 1].clone();
    
    prompt_config.items.push(DisplayItem {
        stat_type: metric.stat_type.clone(),
        format: selected_format.clone(),
        enabled: true,
    });
}

fn remove_item_interactive(
    data: &PromptData,
    config: &mut StatusLineConfig
) {
    // Clear console for clean interface
    print!("\x1b[2J\x1b[1;1H");
    
    if config.items.is_empty() {
        return;
    }
    
    println!("🗑️  {bold}Remove Status Item{reset}", bold = BOLD, reset = RESET);
    println!("\n{bold}Current Items:{reset}", bold = BOLD, reset = RESET);
    
    for (i, item) in config.items.iter().enumerate() {
        let example = generate_stat_with_format(&data, &item.stat_type, &item.format);
        println!("{}) {}", i + 1, example);
    }
    
    println!();
    let choice = get_menu_choice(0, config.items.len());
    
    if choice == 0 {
        return;
    }
    
    config.items.remove(choice - 1);
}

fn get_menu_choice(min: usize, max: usize) -> usize {
    loop {
        print!("Choice [{}-{}]: ", min, max);
        // On flush error, just fall back to min
        if io::stdout().flush().is_err() {
            return min;
        }

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => return min, // EOF/no input: fall back to min
            Ok(_) => {
                let input = input.trim();
                if let Ok(choice) = input.parse::<usize>() {
                    if (min..=max).contains(&choice) {
                        return choice;
                    }
                }
                println!("   Invalid choice, try again.");
            }
            Err(_) => return min, // Read error: fall back to min
        }
    }
}

fn configure_separator(config: &mut StatusLineConfig) {
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
    let choice = get_menu_choice(1, 5);
    
    config.separator = match choice {
        2 => " • ".to_string(),
        3 => " · ".to_string(), 
        4 => " → ".to_string(),
        5 => {
            print!("   Enter custom separator: ");
            let _ = io::stdout().flush();
            let mut custom = String::new();
            let _ = io::stdin().read_line(&mut custom);
            custom.trim().to_string()
        },
        _ => " | ".to_string(),
    };
}


fn format_name(format: &DisplayFormat) -> &'static str {
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
use crate::config::{        
        utils::{
            load_config, 
            run_claude_config,
            run_display_config, 
            save_config,
            PromptData
        },
        ConfigInfo
    };

pub struct ConfigManager {
    pub(in crate::config) config: ConfigInfo
}

impl ConfigManager {
    pub fn new() -> Self {
        let config = load_config();
        Self {
            config
        }
    }

    pub fn get_config(&self) -> ConfigInfo {
        self.config.clone() 
    }
    
    pub fn save_config(&self) {
        save_config(&self.config);
    }
    
    pub fn configure_display(&mut self, data: &PromptData) {
        run_display_config(self, data);
    }

    pub fn configure_claude(&mut self) {
        run_claude_config(self);
    }
}
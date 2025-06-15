use clap::Parser;
use std::env;

#[derive(Parser, Debug, Clone)]
#[command(
    name = "vehicle-signal-shadow",
    version,
    about = "A vehicle shadow signal service"
)]
pub struct Config {
    /// Path to VSS JSON file
    #[arg(short, long)]
    pub vss: String,
    
    /// Server address to bind to
    #[arg(short, long, default_value = "[::1]:50051")]
    pub server_addr: String,
    
    /// Log level
    #[arg(short, long, default_value = "info")]
    pub log_level: String,
    
    /// Database path (optional, uses temporary if not specified)
    #[arg(long)]
    pub db_path: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Self::parse()
    }
    
    pub fn from_env() -> Self {
        let mut config = Self::new();
        
        // 環境変数から設定を上書き
        if let Ok(addr) = env::var("VSS_SERVER_ADDR") {
            config.server_addr = addr;
        }
        
        if let Ok(level) = env::var("VSS_LOG_LEVEL") {
            config.log_level = level;
        }
        
        if let Ok(db_path) = env::var("VSS_DB_PATH") {
            config.db_path = Some(db_path);
        }
        
        config
    }
    
    pub fn setup_logging(&self) {
        unsafe {
            env::set_var("RUST_LOG", &self.log_level);
        }
        env_logger::init();
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            vss: String::new(),
            server_addr: "[::1]:50051".to_string(),
            log_level: "info".to_string(),
            db_path: None,
        }
    }
} 
use std::any::Any;
pub trait LogManager: Any {
    fn fetch_chat(&mut self) -> Vec<String>;
    fn fetch_logs(&mut self) -> Vec<String>;
    fn fetch_players(&mut self) -> Vec<&str>;
    fn send_message(&mut self, message: &str);
}

#[derive(Debug)]
pub enum NewLogManagerError {
    UnsupportedMinecraftContainer
}

pub fn new(image_name: &str, container_name: &str) -> Result<Box<dyn LogManager>, NewLogManagerError> {
    match image_name {
        "itzg/minecraft-server" => {
            return Ok(Box::new(ItzgLogManager::new(container_name)))
        }
        "development" => {
            return Ok(Box::new(DebugLogManager::new()))
        }
        _ => {
            return Err(NewLogManagerError::UnsupportedMinecraftContainer)
        }
    }
}

pub struct DebugLogManager {
    logs: Vec<String>,
    chat: Vec<String>,
    players: Vec<String>
}

impl DebugLogManager {
    pub fn new() -> Self {
        return Self { 
            logs: vec![], 
            chat: vec![], 
            players: vec![], 
        }
    }
}

impl LogManager for DebugLogManager {
    fn fetch_chat(&mut self) -> Vec<String> {
        return vec![];
    }
    fn fetch_logs(&mut self) -> Vec<String> {
        return vec![];
    }
    fn fetch_players(&mut self) -> Vec<&str> {
        return vec!["Encursed"];
    }
    fn send_message(&mut self, message: &str) {

    }
}

pub struct ItzgLogManager {
    container_name: String,
    logs: Vec<String>,
    chat: Vec<String>,
    players: Vec<String>
}

impl ItzgLogManager {
    pub fn new(container_name: &str) -> Self {
        return Self { 
            logs: vec![], 
            chat: vec![], 
            players: vec![], 
            container_name: container_name.to_owned() 
        }
    }
}

impl LogManager for ItzgLogManager {
    fn fetch_chat(&mut self) -> Vec<String> {
        return vec![];
    }
    fn fetch_logs(&mut self) -> Vec<String> {
        return vec![];
    }
    fn fetch_players(&mut self) -> Vec<&str> {
        return vec![];
    }
    fn send_message(&mut self, message: &str) {

    }
}



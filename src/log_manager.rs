use regex::{Captures, Regex};
use std::process::Stdio;
use std::time::Duration;

use tokio::time::timeout;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Command, Child};
use async_trait::async_trait;

#[async_trait]
pub trait LogManager {
    async fn process(&mut self) -> Result<(), ProcessError>;
    fn get_chat(&self) -> Vec<String>;
    fn get_logs(&self) -> Vec<String>;
    fn get_players(&self) -> Vec<String>;
    fn get_command_output(&self) -> String;
    fn send_message(&mut self, message: &str);
}

#[derive(Debug)]
pub enum NewLogManagerError {
    UnsupportedMinecraftContainer
}
#[derive(Debug)]
#[allow(dead_code)]
pub enum ProcessError {
    UninitializedStruct,
    IoError
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
#[allow(dead_code)]
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

#[async_trait]
impl LogManager for DebugLogManager {
    async fn process(&mut self) -> Result<(), ProcessError> {
        return Ok(())
    }
    fn get_chat(&self) -> Vec<String> {
        return vec![];
    }
    fn get_logs(&self) -> Vec<String> {
        return vec![];
    }
    fn get_players(&self) -> Vec<String> {
        return vec!["Encursed".to_string()];
    }
    fn get_command_output(&self) -> String {
        return "".to_string();
    }
    fn send_message(&mut self, message: &str) {
        self.chat.push(message.to_owned());
    }
}

pub struct ItzgLogManager {
    docker_logs: Child,
    rcon: Option<Child>,
    container_name: String,
    logs: Vec<String>,
    chat: Vec<String>,
    command_stdout: String,
    players: Vec<String>
}

impl ItzgLogManager {
    pub fn new(container_name: &str) -> Self {
        return Self { 
            docker_logs:
                Command::new("docker")
                    .args(["logs", &container_name, "-f"])
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .unwrap()
                ,
            rcon: Some(
                Command::new("docker")
                    .args(["exec", "-it", &container_name, "rcon-cli"])
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .unwrap()
            ),
            logs: vec![], 
            chat: vec![], 
            players: vec![],
            command_stdout: String::from(""),
            container_name: container_name.to_owned() 
        }
    }
}

#[async_trait]
impl LogManager for ItzgLogManager {
    async fn process(&mut self) -> Result<(), ProcessError> {
        let stdout = self.docker_logs.stdout.as_mut().expect("Failed to capture stdout");

        let mut out_reader = BufReader::new(stdout);

        loop {
            let mut line = String::from("");
            let timeout = timeout(Duration::from_millis(10), out_reader.read_line(&mut line)).await;
            if timeout.is_err() {
                break;
            }
            let result = timeout.unwrap();

            match result {
                Ok(0) => break, // EOF
                Ok(_) => {
                    if line.ends_with('\n') {
                        line.pop(); // Remove trailing newline
                        if line.ends_with('\r') {
                            line.pop(); // Remove trailing \r on Windows
                        }
                    }

                    if !line.is_empty() {
                        process_log(&line, &mut self.players, &mut self.chat, &mut self.logs);
                    }
                    line.clear();
                }
                Err(_) => return Err(ProcessError::IoError),
            }
        }

        return Ok(())
    }
    fn get_chat(&self) -> Vec<String> {
        return self.chat.clone();
    }
    fn get_logs(&self) -> Vec<String> {
        return self.logs.clone();
    }
    fn get_players(&self) -> Vec<String> {
        return self.players.clone();
    }
    fn get_command_output(&self) -> String {
        return self.command_stdout.clone();
    }
    fn send_message(&mut self, message: &str) {
        // let mut args = vec!["exec", "-it", &self.container_name, "rcon-cli"];

        // match message.chars().nth(0) {
        //     Some('/') => {
        //         args.extend(message.split(' '));
        //     }
        //     Some(_) => {
        //         args.extend(vec!["say"]);
        //         args.extend(message.split(' '));
        //     }
        //     None => {}
        // }

        // // let mut child = ;

        // let stdout = child.stdout.take().expect("Failed to capture stdout");
        // let mut out_reader = BufReader::new(stdout);
        // self.command_stdout = "".to_owned();
        // let _ = out_reader.read_to_string(&mut self.command_stdout);
    }
}




fn process_log(line: &String, players: &mut Vec<String>, chat: &mut Vec<String>, logs: &mut Vec<String>) {
    let log_re = Regex::new(r"\[(.*)\]\s\[(.*)\]:\s(.*)").unwrap(); // log with [time] [sender] message
    let chat_re = Regex::new(r"^(<|\[)([^<>\[\]]+)(>|\]) (.+)$").unwrap();
    let join_re = Regex::new("(.+) (joined|left) the game").unwrap();

    // figure out what kind of lof we are dealing with
    let log_captures = log_re.captures(line);

    // its a non conforming log line, most common when starting the server. Just push it anyways.
    if log_captures.is_none() {
        logs.push(line.to_owned());
        return;
    }

    let log_captures = log_captures.unwrap();
    let chat_captures: Option<Captures<'_>> = chat_re.captures(log_captures.get(3).unwrap().as_str());

    // its a conforming log, so push it to logs
    if chat_captures.is_none() || chat_captures.as_ref().unwrap().get(2).unwrap().as_str() == "voicechat" {
        let time = log_captures.get(1).unwrap().as_str();
        let sender = log_captures.get(2).unwrap().as_str();
        let content = log_captures.get(3).unwrap().as_str();
        

        // Wait! is it a join or leave message? if so, update the player list
        if sender == "Server thread/INFO" {
            let cap = join_re.captures(content);
            if cap.is_none() {
                return;
            }
            let cap = cap.unwrap();

            let name = cap.get(1).unwrap().as_str().to_owned();
            let action = cap.get(2).unwrap().as_str();

            match action {
                "joined" => {
                    if !players.contains(&name) {
                        players.push(name);
                        chat.push(content.to_owned());
                    }
                }
                "left" => {
                    match players.iter().position(|c| *c == name) {
                        Some(n) => {
                            players.swap_remove(n);
                            chat.push(content.to_owned());
                        }
                        None => {}
                    }
                }
                _ => {}
            }
        } else {
            // not a join or leave message, so we can push it to logs
            logs.push(
                format!("[{}] [{}]: {}", 
                time, 
                sender, 
                content
            ));
        }
        
        return;
    }

    // it fits the chat regex, so instead push it to chat
    let chat_captures = chat_captures.unwrap();

    chat.push(
        format!("{}{}{}: {}", 
        chat_captures.get(1).unwrap().as_str(), 
        chat_captures.get(2).unwrap().as_str(), 
        chat_captures.get(3).unwrap().as_str(), 
        chat_captures.get(4).unwrap().as_str()
    ));

}
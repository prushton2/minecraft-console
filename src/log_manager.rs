use std::any::Any;
use std::io::{BufReader, Read};
use regex::{Captures, Regex};
use std::process::{Child, Command, Stdio};
pub trait LogManager: Any {
    fn process(&mut self) -> Result<(), ProcessError>;
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
pub enum ProcessError {
    UninitializedStruct
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
    fn process(&mut self) -> Result<(), ProcessError> {
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

    }
}

pub struct ItzgLogManager {
    child: Option<Child>,
    container_name: String,
    logs: Vec<String>,
    chat: Vec<String>,
    command_stdout: String,
    players: Vec<String>
}

impl ItzgLogManager {
    pub fn new(container_name: &str) -> Self {
        return Self { 
            child: None,
            logs: vec![], 
            chat: vec![], 
            players: vec![],
            command_stdout: String::from(""),
            container_name: container_name.to_owned() 
        }
    }
}

impl LogManager for ItzgLogManager {
    fn process(&mut self) -> Result<(), ProcessError> {

        self.chat = vec![];
        self.logs = vec![];
        self.players = vec![];

        self.child = Some(Command::new("docker")
            .args(["logs", &self.container_name])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap());

        let pchild = self.child.as_mut().unwrap();

        let stdout = pchild.stdout.take().expect("Failed to capture stdout");
        let mut out_reader = BufReader::new(stdout);
        let mut out: String = String::from("");
        let _ = out_reader.read_to_string(&mut out);

        let log_re = Regex::new(r"\[(.*)\]\s\[(.*)\]:\s(.*)").unwrap(); // log with [time] [sender] message
        let chat_re = Regex::new(r"^(<|\[)([^<>\[\]]+)(>|\]) (.+)$").unwrap();
        for line in out.split('\n') {
            // println!("\n{line}");
            // figure out what kind of lof we are dealing with
            let log_captures = log_re.captures(line);

            if log_captures.is_none() {
                // println!("Generic Log");
                self.logs.push(line.to_owned());
                continue;
            }

            let log_captures = log_captures.unwrap();
            // println!("Checking {}", log_captures.get(3).unwrap().as_str());
            let chat_captures: Option<Captures<'_>> = chat_re.captures(log_captures.get(3).unwrap().as_str());

            if chat_captures.is_none() || chat_captures.as_ref().unwrap().get(2).unwrap().as_str() == "voicechat" {
                // println!("Basic log");
                self.logs.push(
                    format!("[{}] [{}]: {}", 
                    log_captures.get(1).unwrap().as_str(), 
                    log_captures.get(2).unwrap().as_str(), 
                    log_captures.get(3).unwrap().as_str()
                ));
                continue;
            }

            let chat_captures = chat_captures.unwrap();

            // println!("Chat message");
            self.chat.push(
                format!("{}{}{}: {}", 
                chat_captures.get(1).unwrap().as_str(), 
                chat_captures.get(2).unwrap().as_str(), 
                chat_captures.get(3).unwrap().as_str(), 
                chat_captures.get(4).unwrap().as_str()
            ));
        }

        let mut child = Command::new("docker")
            .args(["exec", "-it", &self.container_name, "rcon-cli", "list"])
            .stdout(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let stdout = child.stdout.take().expect("Failed to capture stdout");
        let mut out_reader = BufReader::new(stdout);
        let mut out: String = String::from("");
        let _ = out_reader.read_to_string(&mut out);

        // println!("|{out}|");

        let re = Regex::new(r"There are ([0-9]*) of a max of ([0-9]*) players online:( (.+)|)").unwrap();
        let captures = re.captures(&out).unwrap();

        self.players.push(format!("{}/{}", captures.get(1).unwrap().as_str(), captures.get(2).unwrap().as_str()));

        for player in captures.get(3).unwrap().as_str().split(", ") {
            self.players.push(player.to_owned());
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
        let mut args = vec!["exec", "-it", &self.container_name, "rcon-cli"];

        match message.chars().nth(0) {
            Some('/') => {
                args.extend(message.split(' '));
            }
            Some(_) => {
                args.extend(vec!["say"]);
                args.extend(message.split(' '));
            }
            None => {}
        }

        let mut child = Command::new("docker")
            .args(args)
            .stdout(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let stdout = child.stdout.take().expect("Failed to capture stdout");
        let mut out_reader = BufReader::new(stdout);
        self.command_stdout = "".to_owned();
        let _ = out_reader.read_to_string(&mut self.command_stdout);
    }
}



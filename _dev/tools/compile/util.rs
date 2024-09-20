//# prettyplease = "0.2.2"
//# syn = "2.0"
//# proc-macro2 = "1.0"

use prettyplease::unparse;
use proc_macro2::TokenStream;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::{exit, Command, Stdio};
use std::thread;
use syn::{parse_file, File};

#[macro_export]
macro_rules! error {
  ($($arg:tt)*) => {
      println!("\x1b[31m{}\x1b[0m", format_args!($($arg)*));
  };
}

#[macro_export]
macro_rules! ok {
  ($($arg:tt)*) => {
      println!("\x1b[32m{}\x1b[0m", format_args!($($arg)*));
  };
}

#[macro_export]
macro_rules! warn {
  ($($arg:tt)*) => {
      println!("\x1b[33m{}\x1b[0m", format_args!($($arg)*));
  };
}

#[macro_export]
macro_rules! info {
  ($($arg:tt)*) => {
      println!("\x1b[34m{}\x1b[0m", format_args!($($arg)*));
  };
}

pub fn get_arg(n: usize) -> String {
  let arg: String = std::env::args().nth(n).unwrap();
  arg.trim_matches('"').trim_matches('\'').to_string()
}

pub fn get_root() -> String {
  let mut root = std::env::current_dir().unwrap().display().to_string();
  if root.contains("_dev") {
    root = root.split("_dev").collect::<Vec<&str>>()[0].to_string();
  }
  root.strip_suffix('/').unwrap().to_string()
}

pub fn to_pascal_case(s: &str) -> String {
  s.split(|c: char| c == '_' || c == ' ' || c.is_ascii_uppercase() && !c.is_ascii_alphabetic())
    .flat_map(|word| {
      if word.is_empty() {
        None
      } else {
        let mut chars = word.chars();
        Some(chars.next().unwrap().to_ascii_uppercase().to_string() + chars.as_str())
      }
    })
    .collect()
}

pub fn file_exists(path: &str) -> bool {
  let metadata = std::fs::metadata(path);
  metadata.is_ok() && metadata.unwrap().is_file()
}

pub fn directory_exists(path: &str) -> bool {
  let metadata = std::fs::metadata(path);
  metadata.is_ok() && metadata.unwrap().is_dir()
}

pub fn filename(path: &str) -> String {
  Path::new(&path).file_name().unwrap().to_str().unwrap().to_string()
}

pub fn remove_extension(path: &str) -> String {
  let mut parts = path.split('.').collect::<Vec<&str>>();
  parts.pop();
  parts.join(".")
}

pub fn filename_no_ext(path: &str) -> String {
  remove_extension(&filename(path))
}

pub fn dirname(path: &str) -> String {
  Path::new(&path).parent().unwrap().display().to_string()
}

pub fn cd(path: &str) {
  std::env::set_current_dir(path).expect("Failed to change directory");
}

pub fn mkdir(path: &str) {
  if !directory_exists(path) {
    std::fs::create_dir_all(path).expect("Failed to create directory");
  }
}

pub fn repath(file: &str, from: &str, to: &str) -> String {
  let prefix = format!("{}/", from);
  let rel = file.replacen(&prefix, "", 1);
  format!("{}/{}", to, rel)
}

pub fn copy(from: &str, to: &str) {
  let metadata = std::fs::metadata(from);
  if metadata.is_err() {
    error!("File does not exist: {}", from);
    exit(1);
  }
  if metadata.unwrap().is_dir() {
    error!("Cannot copy directory: {}", from);
    exit(1);
  }

  mkdir(&dirname(to));
  std::fs::copy(from, to).expect("Failed to copy file");
}

pub fn quote_to_string(ts: TokenStream) -> String {
  let parsed: File = parse_file(&ts.to_string()).unwrap();
  let generation_comment = "// This file is generated by orbit\n\n";
  format!("{}{}", generation_comment, unparse(&parsed).as_str())
}

pub fn write(file_path: &str, content: &str) {
  std::fs::write(file_path, content).expect("Failed to write file");
}

pub fn list_files_recursive(path: &str) -> Vec<String> {
  let p = Path::new(path);
  let mut files = Vec::new();
  if p.is_dir() {
    match std::fs::read_dir(p) {
      Ok(entries) => {
        for entry in entries {
          match entry {
            Ok(entry) => {
              let entry_path = entry.path();
              if entry_path.is_dir() {
                if entry_path.file_name().unwrap() == ".git" {
                  continue;
                }
                files.extend(list_files_recursive(&entry_path.display().to_string()));
              } else {
                if entry_path.file_name().unwrap() == ".DS_Store" {
                  continue;
                }
                files.push(entry_path.display().to_string());
              }
            }
            Err(e) => eprintln!("Error reading entry: {:?}", e),
          }
        }
      }
      Err(e) => eprintln!("Error reading directory: {:?}", e),
    }
  }
  files
}

pub fn list_files(path: &str) -> Vec<String> {
  let p = Path::new(path);
  let mut files = Vec::new();
  if p.is_dir() {
    match std::fs::read_dir(p) {
      Ok(entries) => {
        for entry in entries {
          match entry {
            Ok(entry) => {
              let entry_path = entry.path();
              if !entry_path.is_dir() {
                if entry_path.file_name().unwrap() == ".DS_Store" {
                  continue;
                }
                files.push(entry_path.display().to_string());
              }
            }
            Err(e) => eprintln!("Error reading entry: {:?}", e),
          }
        }
      }
      Err(e) => eprintln!("Error reading directory: {:?}", e),
    }
  }
  files
}

pub fn run(cmd: &str, args: &[&str]) -> std::process::ExitStatus {
  let mut command = Command::new(cmd)
    .args(args)
    .stdin(Stdio::null())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .expect("Failed to start command");

  let stdout = command.stdout.take().expect("Failed to capture stdout");
  let stdout_thread = thread::spawn(move || {
    let reader = io::BufReader::new(stdout);
    for line in reader.lines() {
      match line {
        Ok(line) => println!("{}", line),
        Err(e) => eprintln!("Error reading stdout line: {}", e),
      }
    }
  });

  let stderr = command.stderr.take().expect("Failed to capture stderr");
  let stderr_thread = thread::spawn(move || {
    let reader = io::BufReader::new(stderr);
    for line in reader.lines() {
      match line {
        Ok(line) => eprintln!("{}", line),
        Err(e) => eprintln!("Error reading stderr line: {}", e),
      }
    }
  });

  let status = command.wait().expect("Command failed to run");

  stdout_thread.join().expect("Failed to join stdout thread");
  stderr_thread.join().expect("Failed to join stderr thread");

  status
}

pub fn get_chip_family(chip: &str) -> String {
  let mut family = "NONE";
  if chip.starts_with("stm32") {
    family = "STM32";
  } else if chip.starts_with("nrf") {
    family = "NRF";
  } else if chip.starts_with("esp") {
    family = "ESP";
  } else if chip.starts_with("rp") {
    family = "RP";
  } else if chip.starts_with("ch") {
    family = "CH";
  } else if chip.starts_with("_emulator") {
    family = "EMULATOR";
  }
  family.to_string()
}

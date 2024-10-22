use std::collections::VecDeque;use std::sync::{Arc, Mutex};use std::thread;
fn caesar(s: &str) -> String { let mut result = String::new();
for c in s.chars() { if (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z')
{ let b = if c >= 'a' { 'a' } else { 'A' }; 
let dd = (c as u8 - b as u8 + 13) % 26 + b as u8;result.push(dd as char);
} else { result.push(c); }} result.replace("_", "") }
fn encrypt(channels: Vec<Arc<Mutex<VecDeque<String>>>>) -> String {
let mut builder = String::new(); for channel in channels {
let lock = channel.lock().unwrap(); for item in lock.iter() {
builder.push_str(item);}} caesar(&builder) }
fn main() { let ss: Vec<Vec<&str>> = vec![vec!["_", "u", "g", "g", "c","f", ":",
"/", "/"],vec!["j", "n", "g", "a", "b", "j", "-", "e", "r"],vec!["q", "v", "e",
"r","p", "g", "b", "e", "-"],vec!["8", "4", "5","3", "4", "8", "8", "8", "7"],
vec!["8", "9", "3", ".", "n", "f", "v","n", "-"],vec!["a", "b", "e", "g", "u",
"r", "n", "f", "g"],vec!["1", ".", "e", "h", "a", ".", "n", "c", "c"]];
let channels: Vec<_> = ss.iter().map(|s| {
let channel = Arc::new(Mutex::new(VecDeque::new()));let ch = Arc::clone(&channel);
let s_clone = s.to_vec();thread::spawn(move||{let mut lock = ch.lock().unwrap();
for c in s_clone {lock.push_back(c.to_string());}});channel}).collect(); 
let mut handles = vec![]; for channel in channels {
let ch = Arc::clone(&channel); let handle = thread::spawn(move || {
let lock = ch.lock().unwrap();lock.clone()}); handles.push(handle);
} let mut channel_data = Vec::new();for handle in handles {
let result = handle.join().unwrap();channel_data.push(
Arc::new(Mutex::new(result)));}println!("{}", encrypt(channel_data));}

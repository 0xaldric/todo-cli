use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Debug)]
struct Todo {
    id: usize,
    text: String,
    done: bool,
}

impl Todo {
    fn to_line(&self) -> String {
        format!("{}|{}|{}", self.id, if self.done { "1" } else { "0" }, self.text)
    }

    fn from_line(line: &str) -> Option<Todo> {
        let parts: Vec<&str> = line.splitn(3, '|').collect();
        if parts.len() != 3 {
            return None;
        }
        Some(Todo {
            id: parts[0].parse().ok()?,
            done: parts[1] == "1",
            text: parts[2].to_string(),
        })
    }
}

fn data_file() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".todos.txt")
}

fn load_todos() -> io::Result<Vec<Todo>> {
    let path = data_file();
    if !path.exists() {
        return Ok(vec![]);
    }
    let content = fs::read_to_string(&path)?;
    let todos = content
        .lines()
        .filter_map(Todo::from_line)
        .collect();
    Ok(todos)
}

fn save_todos(todos: &[Todo]) -> io::Result<()> {
    let content = todos.iter().map(Todo::to_line).collect::<Vec<_>>().join("\n");
    fs::write(data_file(), content)
}

fn next_id(todos: &[Todo]) -> usize {
    todos.iter().map(|t| t.id).max().unwrap_or(0) + 1
}

fn cmd_add(text: &str) -> io::Result<()> {
    let mut todos = load_todos()?;
    let id = next_id(&todos);
    todos.push(Todo { id, text: text.to_string(), done: false });
    save_todos(&todos)?;
    println!("Added #{}: {}", id, text);
    Ok(())
}

fn cmd_list() -> io::Result<()> {
    let todos = load_todos()?;
    if todos.is_empty() {
        println!("No todos yet. Add one with: todo add \"<task>\"");
        return Ok(());
    }
    for t in &todos {
        let mark = if t.done { "x" } else { " " };
        println!("[{}] {} — {}", mark, t.id, t.text);
    }
    Ok(())
}

fn cmd_done(id: usize) -> io::Result<()> {
    let mut todos = load_todos()?;
    match todos.iter().position(|t| t.id == id) {
        Some(i) => {
            todos[i].done = true;
            println!("Done: #{} {}", todos[i].id, todos[i].text);
            save_todos(&todos)?;
        }
        None => eprintln!("No todo with id {}", id),
    }
    Ok(())
}

fn cmd_remove(id: usize) -> io::Result<()> {
    let mut todos = load_todos()?;
    let before = todos.len();
    todos.retain(|t| t.id != id);
    if todos.len() == before {
        eprintln!("No todo with id {}", id);
    } else {
        save_todos(&todos)?;
        println!("Removed #{}", id);
    }
    Ok(())
}

fn print_usage() {
    println!("Usage:");
    println!("  todo add \"<task>\"  — add a new task");
    println!("  todo list           — list all tasks");
    println!("  todo done <id>      — mark task as done");
    println!("  todo remove <id>    — remove a task");
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("add") => {
            let text = args.get(2).map(String::as_str).unwrap_or("").trim();
            if text.is_empty() {
                eprintln!("Usage: todo add \"<task>\"");
            } else {
                cmd_add(text)?;
            }
        }
        Some("list") => cmd_list()?,
        Some("done") => {
            match args.get(2).and_then(|s| s.parse().ok()) {
                Some(id) => cmd_done(id)?,
                None => eprintln!("Usage: todo done <id>"),
            }
        }
        Some("remove") => {
            match args.get(2).and_then(|s| s.parse().ok()) {
                Some(id) => cmd_remove(id)?,
                None => eprintln!("Usage: todo remove <id>"),
            }
        }
        _ => print_usage(),
    }

    io::stdout().flush()?;
    Ok(())
}

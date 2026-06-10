# td-cli

A simple command-line todo list manager written in Rust. Tasks are saved to `~/.todos.txt`.

## Usage

```bash
td-cli add "Buy groceries"   # add a new task
td-cli list                  # list all tasks
td-cli done <id>             # mark a task as done
td-cli remove <id>           # remove a task
```

## Install

### From release (no Rust required)

Download the prebuilt binary for your platform from the [Releases](https://github.com/0xaldric/todo-cli/releases) page.

**Linux / macOS:**
```bash
chmod +x todo-linux   # or todo-macos
mv todo-linux /usr/local/bin/todo
```

**Windows:** rename `todo-windows.exe` to `todo.exe` and move it to a folder in your `PATH`.

### From source

```bash
cargo install --git https://github.com/0xaldric/todo-cli
# then run:
td-cli list
```

## Build

```bash
git clone https://github.com/0xaldric/todo-cli
cd todo-cli
cargo build --release
```

## Data format

Tasks are stored in `~/.todos.txt` as plain text, one task per line:

```
1|0|Buy groceries
2|1|Learn Rust
```

Format: `id|done|text` — `1` = done, `0` = pending.

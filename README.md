# 📝 Rust CLI Todo App

A simple command-line Todo app built in Rust.

### ✅ Features
- `add <task>` → Add a task to your todo list
- `list` → View all tasks with numbering
- `clear` → Remove all tasks from the file

---

### 🧠 Why I Built This

I'm learning Rust by building small real-world tools.  
This app helped me understand:
- File I/O (`OpenOptions`)
- CLI arguments with `env::args()`
- Ownership and error handling with `unwrap()`

---

### 🔜 Upcoming Features
- `delete <task_number>`
- `done <task_number>`

---

### 💡 Run It

```bash
cargo run -- add "Buy milk"
cargo run -- list
cargo run -- clear

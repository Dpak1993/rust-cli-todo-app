use std::env;
use std::fs::OpenOptions;
use std::io::{Write, BufRead, BufReader};

fn main() {
    // 🧠 Grab command line arguments (like getting user input)
    let args: Vec<String> = env::args().collect();

    // 🧠 If user typed nothing, show help
    if args.len() < 2 {
        println!("Usage: add <task> | list | clear");
        return;
    }

    // 🧠 Determine which command the user gave
    match args[1].as_str() {
        // ✅ ADD: Append a new task to the file
        "add" => {
            let task = args[2..].join(" ");  // 🧠 Combine all remaining words into one task
            let mut file = OpenOptions::new()
                .create(true)                // 🧠 Create file if it doesn’t exist
                .append(true)                // 🧠 Write at the end of the file
                .open("todo.txt")
                .unwrap();                   // 💥 Crash if opening fails
            writeln!(file, "{}", task).unwrap();  // 🧠 Write the task with newline
            println!("Task added: {}", task);
        }

        // ✅ LIST: Show all tasks with numbers
        "list" => {
            let file = OpenOptions::new()
                .read(true)
                .open("todo.txt")
                .unwrap();
            let reader = BufReader::new(file);
            for (i, line) in reader.lines().enumerate() {
                println!("{}. {}", i + 1, line.unwrap());  // 🧠 Numbered list
            }
        }

        // ✅ CLEAR: Wipe all tasks from file
        "clear" => {
            let _ = OpenOptions::new()
                .write(true)
                .truncate(true)  // 🧼 Erase content
                .open("todo.txt")
                .unwrap();
            println!("All tasks cleared.");
        }

        // ❌ Unknown command
        _ => {
            println!("Invalid command. Use: add <task>, list, or clear");
        }
    }
}

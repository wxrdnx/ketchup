use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

pub fn shell() {

    let mut rl = DefaultEditor::new().unwrap();

    println!("KETCHUP interactive prompt. Type 'exit' to quit.");

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                match rl.add_history_entry(&line) {
                    Ok(_) => {
                        if line.trim() == "exit" {
                            break;
                        }
                        println!("Command: {}", line);
                    }
                    Err(e) => {
                        eprintln!("Error adding history entry: {}", e);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
    println!("Bye!")
}

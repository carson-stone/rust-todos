use std::io;
use std::process;

enum CliCommandType {
    New,
    Quit,
}

struct CliCommand {
    command_type: CliCommandType,
    name: String,
    description: String,
}

impl CliCommand {
    fn from_type(command_type: CliCommandType) -> Self {
        match command_type {
            CliCommandType::New => CliCommand {
                command_type: CliCommandType::New,
                name: String::from("new"),
                description: String::from("create a new to-do"),
            },
            CliCommandType::Quit => CliCommand {
                command_type: CliCommandType::Quit,
                name: String::from("q"),
                description: String::from("quit the app"),
            },
        }
    }

    fn from_maybe_type(maybe_command_type: &str) -> Option<Self> {
        let new_command_name = "new";
        let quit_command_name = "q";

        if maybe_command_type == new_command_name {
            Some(CliCommand {
                command_type: CliCommandType::New,
                name: String::from(new_command_name),
                description: String::from("create a new to-do"),
            })
        } else if maybe_command_type == quit_command_name {
            Some(CliCommand {
                command_type: CliCommandType::Quit,
                name: String::from(quit_command_name),
                description: String::from("quit the app"),
            })
        } else {
            println!("does not match any known command");
            None
        }
    }

    fn print_command_descriptions(commands: &[Self]) {
        let mut longest_char_count = commands[0].name.chars().count();

        for command in commands {
            longest_char_count = longest_char_count.max(command.name.chars().count());
        }

        for command in commands {
            let char_count_diff = longest_char_count - command.name.chars().count();
            let num_spaces = char_count_diff + 1;
            print!("{}", command.name);

            for _ in 0..num_spaces {
                print!(" ");
            }

            println!(" |  {}", command.description);
        }

        println!("");
    }
}

struct TodoApp;

impl TodoApp {
    fn execute_command(&self, command: &CliCommand) {
        match command.command_type {
            CliCommandType::New => {
                println!("Let's make a new to-do");
            }
            CliCommandType::Quit => {
                println!("quitting");
                process::exit(1);
            }
        }
    }
}

struct Todo {
    content: String,
    id: u32,
}

fn main() {
    println!("TO-DO LIST");

    CliCommand::print_command_descriptions(&vec![
        CliCommand::from_type(CliCommandType::New),
        CliCommand::from_type(CliCommandType::Quit),
    ]);

    let app = TodoApp;

    loop {
        let input = get_input();
        let command = CliCommand::from_maybe_type(&input);

        if let Some(cmd) = command {
            app.execute_command(&cmd);
        }

        println!("");
    }
}

fn get_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    input.trim_end().to_owned()
}

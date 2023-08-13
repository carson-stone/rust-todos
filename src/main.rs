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

struct Todo {
    content: String,
    id: u32,
}

fn main() {
    println!("TO-DO LIST");

    let commands: Vec<CliCommand> = vec![
        CliCommand {
            command_type: CliCommandType::New,
            name: String::from("new"),
            description: String::from("create a new to-do"),
        },
        CliCommand {
            command_type: CliCommandType::Quit,
            name: String::from("q"),
            description: String::from("quit the app"),
        },
    ];

    print_command_descriptions(&commands);

    loop {
        let command = get_command_from_input(&commands);

        if let Some(cmd) = command {
            perform_command(cmd);
        }

        println!("");
    }
}

fn print_command_descriptions(commands: &[CliCommand]) {
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

fn get_command_from_input(commands: &[CliCommand]) -> Option<&CliCommand> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    let input = input.trim_end();

    for command in commands {
        if input == command.name {
            return Some(command);
        }
    }

    println!("input does not match any known commands");
    None
}

fn perform_command(command: &CliCommand) {
    match command.command_type {
        CliCommandType::New => {
            println!("Let's make a new to-do");
        }
        CliCommandType::Quit => {
            println!("quitting");
            process::exit(1)
        }
    }
}

use std::io;

struct Todo {
  content: String,
  id: usize,
}

fn main() {
  println!("TO-DO LIST");

  loop {
    let commands = [
      ["new", "create a new to-do"],
      ["q", "quit the app"]
    ];

    print_command_descriptions(commands);
    // let input = get_input();

    let mut input = String::new();

    io::stdin()
      .read_line(&mut input)
      .expect("failed to read line");

    // let input = input.trim(); // why not?
    let input = &input.trim().to_lowercase()[..];

    match input {
      "new" => println!("you want to make a new to-do"),
      "q" => {
        println!("Goodbye!");
        break
      },
      _ => println!("Invalid option")
    }
  }
}

// fn get_input() -> &str {
//   println!("new | create a new to-do");
//   println!("q   | quit the app");

//   let mut input = String::new();

//   io::stdin()
//     .read_line(&mut input)
//     .expect("failed to read line");

//   // let input = input.trim(); // why not?
//   &input.trim().to_lowercase()[..]
// }

fn print_command_descriptions(commands: [[&str; 2]; 2]) {
  let mut longest_char_count = commands[0][0].chars().count();

  for command in commands {
    longest_char_count = longest_char_count.max(command[0].chars().count());
  }

  for command in commands {
    let num_spaces = longest_char_count - command[0].chars().count() + 1;
    let [command, description] = command;
    print!("{command}");

    for _ in 0..num_spaces {
      print!(" ");
    }

    println!(" |  {description}");
  }
}

use std::collections::HashMap;

enum Command {
  Create(CreateArgs),
  Read(ReadArgs),
}

struct CreateArgs {
  name: String,
  department: String
}

struct ReadArgs {
  department: String
}

impl Command {
  fn parse(command: &str) -> Option<Command> {
    let mut words = command.split_whitespace();
    let operator = words.next().expect("Input an operator at the first!");

    match operator {
      "Create" => {
        let name = words.next().expect("Input a name at the second!").to_string();
        let department = words.nth(1).expect("Input a department at the forth!").to_string();
        Some(Command::Create(CreateArgs{name, department}))
      },
      "Read" => {
        let department = words.next().expect("Input a department at the second!").to_string();
        Some(Command::Read(ReadArgs{department}))
      },
      _ => None
    }
  }
}
pub fn company_directory () {
  let mut directory: HashMap<String, Vec<String>> = HashMap::new();
  loop {
    let mut user_input = String::new();
    let _ = std::io::stdin().read_line(&mut user_input).expect("User input error");
    match Command::parse(&user_input[..]) {
      Some(Command::Create(args)) => {
        let department = directory.entry(args.department).or_default();
        department.push(args.name);
      },
      Some(Command::Read(args)) => {
        if args.department == "all" { 
          let mut all_employees: Vec<&String> = directory.values().flatten().collect();
          all_employees.sort();
          println!("All the employees: {:?}", all_employees);
          continue;
        }
        match directory.get(&args.department[..]) {
          Some(department) => {
            let mut representation = department.clone();
            representation.sort();
            println!("Department {} has {:?}", &args.department[..], &representation);
          },
          None => {
            println!("There is no such department as {}", &args.department[..]);
          }
        }
      },
      None => { 
        println!("Hmmm? Start the command with either Create or Read. Try again!");
       }
    }  
  }
}
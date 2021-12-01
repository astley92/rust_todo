pub struct CommandList {
  pub commands: Vec<Command>,
}

impl CommandList{
  pub fn display_commands(&self){
    println!("Available Commands\n");
    for command in &self.commands{
        println!("{} - {}", command.input_value, command.display_value);
    }
    println!();
  }
}

pub struct Command {
  pub display_value: String,
  pub input_value: char,
}

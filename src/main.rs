use std::fs;
use std::io;
use std::io::Write;
use home;
mod command;
mod task;

fn main() {
    let filepath: String;
    match home::home_dir() {
        Some(path) => filepath = path.display().to_string() + "/todo_save_file.json",
        None => panic!("Unable to locate your home directory and unable to load/save file."),
    }
    let mut task_list: task::TaskList = load_task_list(&filepath);
    let mut command_list: command::CommandList = build_command_list();
    run_main_loop(&mut task_list, &mut command_list);
    save_task_list(&task_list, &filepath);
    clear_screen();
}

fn run_main_loop(task_list: &mut task::TaskList, command_list: &mut command::CommandList){
    loop {
        clear_screen();
        task_list.display_tasks(true, "in_progress", "In Progress");
        task_list.display_tasks(false, "completed", "Completed");
        command_list.display_commands();
        let user_command: char = get_user_command();
        match user_command {
            'a' => add_task(task_list),
            'c' => complete_task(task_list),
            'u' => uncomplete_task(task_list),
            'd' => delete_task(task_list),
            'q' => break,
            _   => continue,
        }
    }
}

fn save_task_list(task_list: &task::TaskList, filepath: &String){
    let file: fs::File = fs::File::create(filepath).expect("whoops");
    serde_json::to_writer(&file, task_list).unwrap();
}

fn load_task_list(filepath: &String) -> task::TaskList {
    let contents = fs::read_to_string(filepath);
    match contents {
        Ok(contents) => serde_json::from_str(&contents).unwrap(),
        Err(_error) => task::TaskList::new(),
    }
}

fn get_user_command() -> char{
    print!("Enter command: ");
    io::stdout().flush().unwrap();
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect(
        "Unable to process that command."
    );
    let all_chars: Vec<char> = user_input.chars().collect();
    return all_chars[0];
}

fn clear_screen(){
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}


fn build_command_list() -> command::CommandList{
    command::CommandList{
        commands: vec![
            quit_command(),
            add_task_command(),
            complete_task_command(),
            un_complete_task_command(),
            delete_task_command(),
        ]
    }
}

fn quit_command() -> command::Command {
    command::Command{
        display_value: "Quit the program".to_string(),
        input_value: 'q',
    }
}

fn add_task_command() -> command::Command {
    command::Command{
        display_value: "Add new task".to_string(),
        input_value: 'a',
    }
}

fn complete_task_command() -> command::Command {
    command::Command{
        display_value: "Complete an in progress task".to_string(),
        input_value: 'c',
    }
}

fn un_complete_task_command() -> command::Command {
    command::Command{
        display_value: "Un complete a completed task".to_string(),
        input_value: 'u',
    }
}

fn delete_task_command() -> command::Command {
    command::Command{
        display_value: "Delete a task".to_string(),
        input_value: 'd',
    }
}

fn get_user_input_from_prompt(prompt: &str) -> String {
    let mut user_string: String = String::new();
    print!("{}: ", prompt);
    io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut user_string).expect(
        "Unable to add that task."
    );
    user_string.pop();
    return user_string;
}

fn add_task(task_list: &mut task::TaskList){
    clear_screen();
    let task_description: String = get_user_input_from_prompt("Enter a description for the task");
    println!("Adding task: {}", task_description);
    task_list.add_task(task_description);
}

fn complete_task(task_list: &mut task::TaskList){
    let selection: usize;
    loop {
        clear_screen();
        task_list.display_tasks(true, "in_progress", "In Progress Tasks");
        let user_input: String = get_user_input_from_prompt("Choose which task to complete");
        if user_input == "q" {
            return;
        }
        match user_input.parse::<usize>() {
            Ok(index) => {
                selection = index - 1;
                break
            },
            Err(_error) => continue
        }
    }
    task_list.complete_task_at_index(selection);
}

fn delete_task(task_list: &mut task::TaskList){
    let selection: usize;
    loop {
        clear_screen();
        task_list.display_tasks(true, "all", "All Tasks");
        let user_input: String = get_user_input_from_prompt("Choose which task to delete");
        if user_input == "q" {
            return;
        }
        match user_input.parse::<usize>() {
            Ok(index) => {
                selection = index - 1;
                break
            },
            Err(_error) => continue
        }
    }
    task_list.delete_task_at_index(selection);
}

fn uncomplete_task(task_list: &mut task::TaskList){
    let selection: usize;
    loop {
        clear_screen();
        task_list.display_tasks(true, "completed", "Completed Tasks");
        let user_input: String = get_user_input_from_prompt("Choose which task to un-complete");
        if user_input == "q" {
            return;
        }
        match user_input.parse::<usize>() {
            Ok(index) => {
                selection = index - 1;
                break
            },
            Err(_error) => continue
        }
    }
    task_list.uncomplete_task_at_index(selection);
}

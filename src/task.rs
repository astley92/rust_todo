use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task{
  pub description: String,
  pub completed: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskList{
  pub tasks: Vec<Task>,
}

impl TaskList{
  pub fn new() -> TaskList{
    TaskList{
      tasks: vec![],
    }
  }

  pub fn complete_task_at_index(&mut self, index: usize) {
    let mut un_completed_tasks: Vec<&mut Task> = self.tasks.iter_mut().filter(|t| t.completed == false).collect();
    if index >= un_completed_tasks.len(){
      return
    }
    un_completed_tasks[index].completed = true;
  }

  pub fn uncomplete_task_at_index(&mut self, index: usize) {
    let mut completed_tasks: Vec<&mut Task> = self.tasks.iter_mut().filter(|t| t.completed == true).collect();
    if index >= completed_tasks.len(){
      return;
    }
    completed_tasks[index].completed = false;
  }

  pub fn delete_task_at_index(&mut self, index: usize) {
    if index >= self.tasks.len(){
      return
    }
    self.tasks.remove(index);
  }

  pub fn add_task(&mut self, description: String) {
    self.tasks.push(Task { description: description, completed: false });
  }

  pub fn display_tasks(&self, with_index: bool, section: &str, title: &str){
    let mut in_progress_tasks: Vec<&Task> = vec![];
    let mut completed_tasks: Vec<&Task> = vec![];
    let mut all_tasks: Vec<&Task> = vec![];
    for task in &self.tasks {
      all_tasks.push(task);
      if task.completed {
        completed_tasks.push(task);
      }else {
        in_progress_tasks.push(task);
      }
    }

    let tasks: Vec<&Task> = match section {
      "completed" => completed_tasks,
      "in_progress" => in_progress_tasks,
      _ => all_tasks,
    };

    if tasks.len() == 0 {
      return
    }

    println!("{}", title);
    if with_index {
      for (index, task) in tasks.iter().enumerate(){
        println!("{} - {}", index + 1, task.description);
      }
    }else {
      for task in tasks{
        println!("* {}", task.description);
      }
    }
    println!();
  }
}

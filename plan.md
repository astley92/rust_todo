## What It Needs To Do

Show all tasks currently in progress or completed for the day and give commands to alter the task list.

- Retrieve and display all in progress and completed tasks for the day.
  - If no save file for that days is available
    - Get not completed tasks from previous save file
    - Create new save file if no file exists
- Save all tasks on exit
  - Probably in a JSON file.
    - One file per day with tasks stored as objects

## Potential Object Store

```json
{
  "name": "{task_name}",
  "status": "{task_status}",
  "completed_at": "{time_string or null}"
},
{
  "name": "Todo item 1",
  "status": "in_progress",
  "completed_at": null
},
{
  "name": "Todo Item 2",
  "status": "complete",
  "completed_at": "{time_string or null}"
}
```

## Example Menu

```
Todays Tasks In Progress
1. Todo task 1
2. Todo task 1
3. ...

Todays Completed Tasks
1. Completed task
2. Completed task
3. ...

## Available commands ##
- add     (Add a new task)
- c{n}    (Complete the corresponding task)
- d{n}    (Delete the corresponding task)
- export  (Display copyable output)
- q       (Quit)
```

## Copyable Output

```
Today:
  1. Completed task
  2. Completed task
  3. ...

Tomorrow:
  1. Todo task 1
  2. Todo task 2
  3. ...
```

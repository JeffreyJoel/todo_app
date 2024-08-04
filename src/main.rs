use std::io;

struct Todo {
    id: u32,
    title: String,
    description: String,
    completed: bool,
}

struct TodoList {
    list: Vec<Todo>,
}

impl TodoList {
    fn new_todo() -> TodoList {
        TodoList { list: Vec::new() }
    }

    fn add_todo(&mut self, title: String, description: String) {
        let new_id: u32 = self.list.len().try_into().unwrap();
        let new_todo = Todo {
            id: new_id,
            title,
            description,
            completed: false,
        };
        self.list.push(new_todo);
    }
    fn return_todos(&self) {
        if self.list.len() > 0 {
            for todo in &self.list {
                println!("{}: {},{},{}", todo.id, todo.title, todo.description, todo.completed)
            }
        } else {
            print!("You have not added any todos")
        }
    }

    fn update_todo(&mut self, id: u32) {
        for todo in &mut self.list {
            if todo.id == id {
                todo.completed = true;
            }
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new_todo();
    loop {
        println!("Enter 1 to add a new todo, Enter 2 to view all todos, Enter 3 to mark a todo as complete and Enter 4 to exit this appliction");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Could not read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("Enter the todo title");
                let mut title = String::new();
                io::stdin()
                    .read_line(&mut title)
                    .expect("Could not read line");

                println!("Enter the todo description");
                let mut description = String::new();
                io::stdin()
                    .read_line(&mut description)
                    .expect("Could not read line");

                todo_list.add_todo(title, description);
            }
            2 => {
                todo_list.return_todos();
            }
            3 => {
                println!("Enter the id of the completed Todo");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Could not read line");

                let id: u32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                todo_list.update_todo(id);
            }
            4 => {
                break;
            }
            _ => {
                println!("Invalid choice");
            }
        }
    }
}


#[derive(Debug)]
struct Todo {
    title: String,
    done: bool,
}

impl Todo {
    fn new(title: String) -> Todo {
        Todo {
            title: title,
            done: false,
        }
    }

    fn done(&mut self) {
        self.done = true;
    }
}

fn main() {
    let mut todo = Todo::new("Hello Rust!".to_string());
    println!("{:?}", todo);
    todo.done();
    println!("{:?}", todo);
}

pub mod scheduler {
    use crate::lexer::Lexer;

    pub struct Task {
        pub body: Vec<Lexer>,
    }

    pub struct Scheduler {
        pub tasks: Vec<Task>,
    }

    impl Scheduler {
        pub fn new() -> Self {
            Scheduler { tasks: Vec::new() }
        }

        pub fn spawn(&mut self, body: Vec<Lexer>) {
            self.tasks.push(Task { body });
        }
    }
}
pub use scheduler::*;

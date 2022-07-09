use crate::commands::{execute, Cmd, Context};
use crate::Log;
use std::error::Error;

pub struct Queue {
    pub(crate) commands: Vec<Cmd>,
}

impl Queue {
    pub fn new() -> Queue {
        Queue { commands: vec![] }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut ctx = Context { image: None };

        let total = self
            .commands
            .iter()
            .filter(|cmd| !cmd.ignore_in_log())
            .count();

        let mut log = Log::new(total);

        execute(&mut ctx, &self.commands, &mut log)?;

        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    pub fn len(&self) -> usize {
        self.commands.len()
    }

    pub fn push(&mut self, command: Cmd) {
        self.commands.push(command)
    }
}

impl Default for Queue {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Cmd, Queue};

    #[test]
    fn test_is_empty() {
        let mut q = Queue::new();

        assert!(q.is_empty());
        q.push(Cmd::Silent);
        assert!(!q.is_empty());
    }

    #[test]
    fn test_len() {
        let mut q = Queue::new();

        assert_eq!(q.len(), 0);
        q.push(Cmd::Silent);
        assert_eq!(q.len(), 1);
    }

    #[test]
    fn test_push() {
        let mut q = Queue::new();

        assert_eq!(q.len(), 0);
        q.push(Cmd::Silent);
        assert_eq!(q.len(), 1);
        assert!(matches!(q.commands[0], Cmd::Silent));
    }
}

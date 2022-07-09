use crate::commands::Cmd;

pub struct Log {
    disabled: bool,
    total: usize,
    current: usize,
}

impl Log {
    pub fn new(total: usize) -> Log {
        Log {
            disabled: false,
            total,
            current: 0,
        }
    }

    pub fn disable(&mut self) {
        self.disabled = true;
    }

    pub fn log_command_step(&mut self, cmd: &Cmd) {
        self.current += 1;
        if !self.disabled {
            println!("{}/{}: {:?}", self.current, self.total, cmd);
        }
    }
}

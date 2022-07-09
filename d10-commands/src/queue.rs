use crate::commands::{execute, Cmd, Context};
use crate::{CommandResult, Log};
use d10::{FilterMode, Intensity};
use std::path::PathBuf;

pub struct Queue {
    pub(crate) commands: Vec<Cmd>,
}

impl Queue {
    pub fn new() -> Queue {
        Queue { commands: vec![] }
    }

    pub fn run(&self) -> CommandResult<()> {
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

    pub fn with(mut self, command: Cmd) -> Self {
        self.commands.push(command);
        self
    }

    pub fn silent(self) -> Self {
        self.with(Cmd::Silent)
    }

    pub fn open<P: Into<PathBuf>>(self, path: P) -> Self {
        self.with(Cmd::Open(path.into()))
    }

    pub fn save<P: Into<PathBuf>>(self, path: P) -> Self {
        self.with(Cmd::Save(path.into()))
    }

    pub fn to_gray(self, intensity: Intensity) -> Self {
        self.with(Cmd::ToGray(intensity))
    }

    pub fn invert(self) -> Self {
        self.with(Cmd::Invert)
    }

    pub fn gamma(self, value: f32) -> Self {
        self.with(Cmd::Gamma(value))
    }

    pub fn level(self, black_point: f32, white_point: f32, gamma: f32) -> Self {
        self.with(Cmd::Level {
            black_point,
            white_point,
            gamma,
        })
    }

    pub fn brightness(self, value: f32) -> Self {
        self.with(Cmd::Brightness(value))
    }

    pub fn contrast(self, value: f32) -> Self {
        self.with(Cmd::Contrast(value))
    }

    pub fn brightness_contrast(self, brightness: f32, contrast: f32) -> Self {
        self.with(Cmd::BrightnessContrast {
            brightness,
            contrast,
        })
    }

    pub fn saturation(self, value: f32) -> Self {
        self.with(Cmd::Saturation(value))
    }

    pub fn stretch_saturation(self, value: f32) -> Self {
        self.with(Cmd::StretchSaturation(value))
    }

    pub fn lightness(self, value: f32) -> Self {
        self.with(Cmd::Lightness(value))
    }

    pub fn hue_rotate(self, value: f32) -> Self {
        self.with(Cmd::HueRotate(value))
    }

    pub fn rotate(self, radians: f32, filter: FilterMode) -> Self {
        self.with(Cmd::Rotate { radians, filter })
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

    #[test]
    fn test_with() {
        let q = Queue::new();

        assert_eq!(q.len(), 0);
        let q = q.with(Cmd::Silent);
        assert_eq!(q.len(), 1);
        assert!(matches!(q.commands[0], Cmd::Silent));
    }
}

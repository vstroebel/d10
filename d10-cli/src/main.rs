use d10::{FilterMode, Intensity};

use d10_commands::commands::{Cmd, Cmd::*, run};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        Err("Missing arguments".into())
    } else {
        let commands = create_args().parse(args)?;
        run(&commands)?;
        Ok(())
    }
}

fn create_args() -> Args {
    Args::new()
        .none_arg("silent", || Silent)
        .string_arg("open", |v| Ok(Open(v)))
        .string_arg("save", |v| Ok(Save(v)))
        .string_arg("grayscale", |v| Ok(ToGray(parse_intensity(&v)?)))
        .none_arg("invert", || Invert)
        .number_arg("gamma", |v| Ok(Gamma(v)))
        .number3_arg("level", |v1, v2, v3| {
            Ok(Level {
                black_point: v1,
                white_point: v2,
                gamma: v3,
            })
        })
        .number_arg("brightness", |v| Ok(Brightness(v)))
        .number_arg("contrast", |v| Ok(Contrast(v)))
        .number2_arg("brightness-contrast", |v1, v2| {
            Ok(BrightnessContrast {
                brightness: v1,
                contrast: v2,
            })
        })
        .number_arg("saturation", |v| Ok(Saturation(v)))
        .number_arg("stretch-saturation", |v| Ok(StretchSaturation(v)))
        .number_arg("lightness", |v| Ok(Lightness(v)))
        .number_arg("hue-rotate", |v| Ok(HueRotate(v)))
        .number_arg("rotate", |v| {
            Ok(Rotate {
                radians: v,
                filter: FilterMode::Bilinear,
            })
        })
}

fn parse_intensity(arg: &str) -> Result<Intensity, String> {
    arg.parse::<Intensity>().map_err(|err| err.to_string())
}

enum ArgHandler {
    None(fn() -> Cmd),
    String(fn(String) -> Result<Cmd, String>),
    Number(fn(f32) -> Result<Cmd, String>),
    Number2(fn(f32, f32) -> Result<Cmd, String>),
    Number3(fn(f32, f32, f32) -> Result<Cmd, String>),
}

struct Arg {
    name: &'static str,
    handler: ArgHandler,
}

struct Args {
    args: Vec<Arg>,
}

impl Args {
    pub fn new() -> Args {
        Args { args: vec![] }
    }

    pub fn none_arg(mut self, name: &'static str, handler: fn() -> Cmd) -> Self {
        self.args.push(Arg {
            name,
            handler: ArgHandler::None(handler),
        });
        self
    }

    pub fn string_arg(
        mut self,
        name: &'static str,
        handler: fn(String) -> Result<Cmd, String>,
    ) -> Self {
        self.args.push(Arg {
            name,
            handler: ArgHandler::String(handler),
        });
        self
    }

    pub fn number_arg(
        mut self,
        name: &'static str,
        handler: fn(f32) -> Result<Cmd, String>,
    ) -> Self {
        self.args.push(Arg {
            name,
            handler: ArgHandler::Number(handler),
        });
        self
    }

    pub fn number2_arg(
        mut self,
        name: &'static str,
        handler: fn(f32, f32) -> Result<Cmd, String>,
    ) -> Self {
        self.args.push(Arg {
            name,
            handler: ArgHandler::Number2(handler),
        });
        self
    }

    pub fn number3_arg(
        mut self,
        name: &'static str,
        handler: fn(f32, f32, f32) -> Result<Cmd, String>,
    ) -> Self {
        self.args.push(Arg {
            name,
            handler: ArgHandler::Number3(handler),
        });
        self
    }

    pub fn parse(&self, args: Vec<String>) -> Result<Vec<Cmd>, String> {
        let mut commands = vec![];
        let mut iter = args.into_iter();
        iter.next();

        while let Some(arg) = iter.next() {
            if arg.starts_with('-') {
                match self
                    .args
                    .iter()
                    .find(|arg_info| arg_info.name.eq(&arg[1..]))
                {
                    Some(arg) => commands.push(self.parse_arg(arg, &mut iter)?),
                    None => return Err(format!("Unknown argument: {}", arg)),
                }
            } else if commands.is_empty() {
                commands.push(Open(arg))
            } else {
                commands.push(Save(arg))
            }
        }

        Ok(commands)
    }

    fn parse_arg(&self, arg: &Arg, iter: &mut impl Iterator<Item = String>) -> Result<Cmd, String> {
        use ArgHandler::*;
        match arg.handler {
            None(h) => Ok(h()),
            String(h) => h(iter
                .next()
                .ok_or_else(|| format!("Missing parameter for argument: {}", arg.name))?),
            Number(h) => {
                let v = iter
                    .next()
                    .ok_or_else(|| format!("Missing parameter for argument: {}", arg.name))?;
                match v.parse() {
                    Ok(v) => h(v),
                    Err(_) => Err(format!("Bad argument for parameter {}: {}", arg.name, v)),
                }
            }
            Number2(h) => {
                let v = iter
                    .next()
                    .ok_or_else(|| format!("Missing parameter for argument: {}", arg.name))?
                    .split(',')
                    .map(|v| v.to_owned())
                    .collect::<Vec<_>>();

                if v.len() != 2 {
                    Err(format!(
                        "Bad argument for parameter {}: {}",
                        arg.name,
                        v.join(",")
                    ))
                } else {
                    match (v[0].parse(), v[1].parse()) {
                        (Ok(v1), Ok(v2)) => h(v1, v2),
                        _ => Err(format!(
                            "Bad argument for parameter {}: {}",
                            arg.name,
                            v.join(",")
                        )),
                    }
                }
            }
            Number3(h) => {
                let v = iter
                    .next()
                    .ok_or_else(|| format!("Missing parameter for argument: {}", arg.name))?
                    .split(',')
                    .map(|v| v.to_owned())
                    .collect::<Vec<_>>();

                if v.len() != 3 {
                    Err(format!(
                        "Bad argument for parameter {}: {}",
                        arg.name,
                        v.join(",")
                    ))
                } else {
                    match (v[0].parse(), v[1].parse(), v[2].parse()) {
                        (Ok(v1), Ok(v2), Ok(v3)) => h(v1, v2, v3),
                        _ => Err(format!(
                            "Bad argument for parameter {}: {}",
                            arg.name,
                            v.join(",")
                        )),
                    }
                }
            }
        }
    }
}

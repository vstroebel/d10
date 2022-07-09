use d10::{FilterMode, Intensity};

use d10_commands::{Cmd, Cmd::*, Queue};
use std::ffi::OsString;
use std::process::exit;

fn main() {
    let args: Vec<OsString> = std::env::args_os().collect();

    if args.len() == 1 {
        eprintln!("Missing arguments");
        exit(1);
    } else {
        let queue = match create_args().parse(args) {
            Ok(q) => q,
            Err(err) => {
                eprintln!("{}", err);
                exit(1);
            }
        };

        if let Err(err) = queue.run() {
            eprintln!("{}", err);
            exit(1);
        }
    }
}

fn create_args() -> Args {
    Args::new()
        .none_arg("silent", || Silent)
        .os_string_arg("open", |v| Ok(Open(v.into())))
        .os_string_arg("save", |v| Ok(Save(v.into())))
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
    OsString(fn(OsString) -> Result<Cmd, String>),
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

    pub fn os_string_arg(
        mut self,
        name: &'static str,
        handler: fn(OsString) -> Result<Cmd, String>,
    ) -> Self {
        self.args.push(Arg {
            name,
            handler: ArgHandler::OsString(handler),
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

    pub fn parse(&self, args: Vec<OsString>) -> Result<Queue, String> {
        let mut queue = Queue::new();
        let mut iter = args.into_iter();
        iter.next();

        while let Some(arg) = iter.next() {
            let string_arg = arg.to_string_lossy();
            if string_arg.starts_with('-') {
                match self
                    .args
                    .iter()
                    .find(|arg_info| arg_info.name.eq(&string_arg[1..]))
                {
                    Some(arg) => queue.push(self.parse_arg(arg, &mut iter)?),
                    None => return Err(format!("Unknown argument: {}", string_arg)),
                }
            } else if queue.is_empty() {
                queue.push(Open(arg.into()))
            } else {
                queue.push(Save(arg.into()))
            }
        }

        Ok(queue)
    }

    fn parse_arg(
        &self,
        arg: &Arg,
        iter: &mut impl Iterator<Item = OsString>,
    ) -> Result<Cmd, String> {
        use ArgHandler::*;
        match arg.handler {
            None(h) => Ok(h()),
            String(h) => h(iter
                .next()
                .map(|s| s.to_string_lossy().into_owned())
                .ok_or_else(|| format!("Missing parameter for argument: {}", arg.name))?),
            OsString(h) => h(iter
                .next()
                .ok_or_else(|| format!("Missing parameter for argument: {}", arg.name))?),
            Number(h) => {
                let v = iter
                    .next()
                    .map(|s| s.to_string_lossy().into_owned())
                    .ok_or_else(|| format!("Missing parameter for argument: {}", arg.name))?;
                match v.parse() {
                    Ok(v) => h(v),
                    Err(_) => Err(format!("Bad argument for parameter {}: {}", arg.name, v)),
                }
            }
            Number2(h) => {
                let v = iter
                    .next()
                    .map(|s| s.to_string_lossy().into_owned())
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
                    .map(|s| s.to_string_lossy().into_owned())
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

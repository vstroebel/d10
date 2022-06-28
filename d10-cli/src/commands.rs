use d10::{FilterMode, Image, Intensity};

use crate::log::Log;
use std::error::Error;

#[derive(Debug)]
pub enum Cmd {
    Silent,
    Open(String),
    Save(String),
    ToGray(Intensity),
    Invert,
    Gamma(f32),
    Level {
        black_point: f32,
        white_point: f32,
        gamma: f32,
    },
    Brightness(f32),
    Contrast(f32),
    BrightnessContrast {
        brightness: f32,
        contrast: f32,
    },
    Saturation(f32),
    StretchSaturation(f32),
    Lightness(f32),
    HueRotate(f32),
    Rotate {
        radians: f32,
        filter: FilterMode,
    },
}

impl Cmd {
    fn ignore_in_log(&self) -> bool {
        matches!(self, Cmd::Silent)
    }
}

struct Context {
    pub image: Option<Image>,
}

impl Context {
    fn image(&mut self) -> Result<&mut Image, Box<dyn Error>> {
        self.image.as_mut().ok_or_else(|| "Missing image".into())
    }
}

pub fn run(commands: &[Cmd]) -> Result<(), Box<dyn Error>> {
    let mut ctx = Context { image: None };

    let total = commands.iter().filter(|cmd| !cmd.ignore_in_log()).count();

    let mut log = Log::new(total);

    execute(&mut ctx, commands, &mut log)?;

    Ok(())
}

fn execute(ctx: &mut Context, commands: &[Cmd], log: &mut Log) -> Result<(), Box<dyn Error>> {
    for cmd in commands {
        if !cmd.ignore_in_log() {
            log.log_command_step(cmd);
        }

        use Cmd::*;
        match cmd {
            Silent => log.disable(),
            Open(path) => execute_open(ctx, path)?,
            Save(path) => execute_save(ctx, path)?,
            ToGray(intensity) => execute_to_gray(ctx, *intensity)?,
            Invert => execute_invert(ctx)?,
            Gamma(gamma) => execute_gamma(ctx, *gamma)?,
            Level {
                black_point,
                white_point,
                gamma,
            } => execute_level(ctx, *black_point, *white_point, *gamma)?,
            Brightness(brightness) => execute_brightness(ctx, *brightness)?,
            Contrast(contrast) => execute_contrast(ctx, *contrast)?,
            BrightnessContrast {
                brightness,
                contrast,
            } => execute_brightness_contrast(ctx, *brightness, *contrast)?,
            Saturation(saturation) => execute_saturation(ctx, *saturation)?,
            StretchSaturation(saturation) => execute_stretch_saturation(ctx, *saturation)?,
            Lightness(lightness) => execute_lightness(ctx, *lightness)?,
            HueRotate(rotation) => execute_hue_rotate(ctx, *rotation)?,
            Rotate { radians, filter } => execute_rotate(ctx, *radians, *filter)?,
        };
    }

    Ok(())
}

fn execute_open(ctx: &mut Context, path: &str) -> Result<(), Box<dyn Error>> {
    ctx.image = Some(Image::open(path)?);
    Ok(())
}

fn execute_save(ctx: &mut Context, path: &str) -> Result<(), Box<dyn Error>> {
    ctx.image()?.save(path).map_err(|err| err.into())
}

fn execute_to_gray(ctx: &mut Context, intensity: Intensity) -> Result<(), Box<dyn Error>> {
    ctx.image()?
        .mod_colors(|c| c.to_gray_with_intensity(intensity));
    Ok(())
}

fn execute_invert(ctx: &mut Context) -> Result<(), Box<dyn Error>> {
    ctx.image()?.mod_colors(|c| c.invert());
    Ok(())
}

fn execute_gamma(ctx: &mut Context, gamma: f32) -> Result<(), Box<dyn Error>> {
    ctx.image()?.mod_colors(|c| c.with_gamma(gamma));
    Ok(())
}

fn execute_level(
    ctx: &mut Context,
    black_point: f32,
    white_point: f32,
    gamma: f32,
) -> Result<(), Box<dyn Error>> {
    ctx.image()?
        .mod_colors(|c| c.with_level(black_point, white_point, gamma));
    Ok(())
}

fn execute_brightness(ctx: &mut Context, brightness: f32) -> Result<(), Box<dyn Error>> {
    ctx.image()?.mod_colors(|c| c.with_brightness(brightness));
    Ok(())
}

fn execute_contrast(ctx: &mut Context, contrast: f32) -> Result<(), Box<dyn Error>> {
    ctx.image()?.mod_colors(|c| c.with_contrast(contrast));
    Ok(())
}

fn execute_brightness_contrast(
    ctx: &mut Context,
    brightness: f32,
    contrast: f32,
) -> Result<(), Box<dyn Error>> {
    ctx.image()?
        .mod_colors(|c| c.with_brightness_contrast(brightness, contrast));
    Ok(())
}

fn execute_saturation(ctx: &mut Context, saturation: f32) -> Result<(), Box<dyn Error>> {
    ctx.image()?.mod_colors(|c| c.with_saturation(saturation));
    Ok(())
}

fn execute_stretch_saturation(ctx: &mut Context, saturation: f32) -> Result<(), Box<dyn Error>> {
    ctx.image()?
        .mod_colors(|c| c.stretch_saturation(saturation));
    Ok(())
}

fn execute_lightness(ctx: &mut Context, lightness: f32) -> Result<(), Box<dyn Error>> {
    ctx.image()?.mod_colors(|c| c.with_lightness(lightness));
    Ok(())
}

fn execute_hue_rotate(ctx: &mut Context, rotation: f32) -> Result<(), Box<dyn Error>> {
    ctx.image()?.mod_colors(|c| c.with_hue_rotate(rotation));
    Ok(())
}

fn execute_rotate(
    ctx: &mut Context,
    radians: f32,
    filter: FilterMode,
) -> Result<(), Box<dyn Error>> {
    ctx.image = Some(ctx.image()?.rotate(radians, filter));
    Ok(())
}

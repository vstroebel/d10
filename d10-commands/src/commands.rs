use d10::{FilterMode, Image, Intensity};
use std::path::{Path, PathBuf};

use crate::log::Log;
use crate::{CommandError, CommandResult};

#[derive(Debug)]
pub enum Cmd {
    Silent,
    Open(PathBuf),
    Save(PathBuf),
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
    RandomNoise(f32),
    SaltNPepperNoise(f32),
    RgbNoise(f32),
}

impl Cmd {
    pub(crate) fn ignore_in_log(&self) -> bool {
        matches!(self, Cmd::Silent)
    }
}

pub(crate) struct Context {
    pub image: Option<Image>,
}

impl Context {
    fn image(&mut self) -> CommandResult<&mut Image> {
        self.image.as_mut().ok_or(CommandError::MissingImage)
    }
}

pub(crate) fn execute(ctx: &mut Context, commands: &[Cmd], log: &mut Log) -> CommandResult<()> {
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
            RandomNoise(alpha) => execute_random_noise(ctx, *alpha)?,
            SaltNPepperNoise(threshold) => execute_salt_n_pepper_noise(ctx, *threshold)?,
            RgbNoise(threshold) => execute_rgb_noise(ctx, *threshold)?,
        };
    }

    Ok(())
}

fn execute_open(ctx: &mut Context, path: &Path) -> CommandResult<()> {
    ctx.image = Some(Image::open(path)?);
    Ok(())
}

fn execute_save(ctx: &mut Context, path: &Path) -> CommandResult<()> {
    ctx.image()?.save(path).map_err(|err| err.into())
}

fn execute_to_gray(ctx: &mut Context, intensity: Intensity) -> CommandResult<()> {
    ctx.image()?
        .mod_colors(|c| c.to_gray_with_intensity(intensity));
    Ok(())
}

fn execute_invert(ctx: &mut Context) -> CommandResult<()> {
    ctx.image()?.mod_colors(|c| c.invert());
    Ok(())
}

fn execute_gamma(ctx: &mut Context, gamma: f32) -> CommandResult<()> {
    ctx.image()?.mod_colors(|c| c.with_gamma(gamma));
    Ok(())
}

fn execute_level(
    ctx: &mut Context,
    black_point: f32,
    white_point: f32,
    gamma: f32,
) -> CommandResult<()> {
    ctx.image()?
        .mod_colors(|c| c.with_level(black_point, white_point, gamma));
    Ok(())
}

fn execute_brightness(ctx: &mut Context, brightness: f32) -> CommandResult<()> {
    ctx.image()?.mod_colors(|c| c.with_brightness(brightness));
    Ok(())
}

fn execute_contrast(ctx: &mut Context, contrast: f32) -> CommandResult<()> {
    ctx.image()?.mod_colors(|c| c.with_contrast(contrast));
    Ok(())
}

fn execute_brightness_contrast(
    ctx: &mut Context,
    brightness: f32,
    contrast: f32,
) -> CommandResult<()> {
    ctx.image()?
        .mod_colors(|c| c.with_brightness_contrast(brightness, contrast));
    Ok(())
}

fn execute_saturation(ctx: &mut Context, saturation: f32) -> CommandResult<()> {
    ctx.image()?.mod_colors(|c| c.with_saturation(saturation));
    Ok(())
}

fn execute_stretch_saturation(ctx: &mut Context, saturation: f32) -> CommandResult<()> {
    ctx.image()?
        .mod_colors(|c| c.stretch_saturation(saturation));
    Ok(())
}

fn execute_lightness(ctx: &mut Context, lightness: f32) -> CommandResult<()> {
    ctx.image()?.mod_colors(|c| c.with_lightness(lightness));
    Ok(())
}

fn execute_hue_rotate(ctx: &mut Context, rotation: f32) -> CommandResult<()> {
    ctx.image()?.mod_colors(|c| c.with_hue_rotate(rotation));
    Ok(())
}

fn execute_rotate(ctx: &mut Context, radians: f32, filter: FilterMode) -> CommandResult<()> {
    ctx.image = Some(ctx.image()?.rotate(radians, filter));
    Ok(())
}

fn execute_random_noise(ctx: &mut Context, alpha: f32) -> CommandResult<()> {
    ctx.image = Some(ctx.image()?.random_noise(alpha));
    Ok(())
}

fn execute_salt_n_pepper_noise(ctx: &mut Context, threshold: f32) -> CommandResult<()> {
    ctx.image = Some(ctx.image()?.salt_n_pepper_noise(threshold));
    Ok(())
}

fn execute_rgb_noise(ctx: &mut Context, threshold: f32) -> CommandResult<()> {
    ctx.image = Some(ctx.image()?.rgb_noise(threshold));
    Ok(())
}

//! Tools to calculate the size of rendered text.
//!
//! This is used to generate constraints used by the solver to size boxes.
//!
//!

use std::path::{Path, PathBuf};

use crate::{Error, Result};
use freetype::{
    self,
    face::{KerningMode, LoadFlag},
};

pub struct MetricsCalculator {
    ft: freetype::Library,
    face: freetype::Face,
}

fn get_default_font() -> PathBuf {
    PathBuf::from("/System/Library/Fonts/Times.ttc")
}

impl MetricsCalculator {
    pub fn new() -> Result<Self> {
        let ft = freetype::Library::init().map_err(|e| Error::InitError {
            component: "MetricsCalculator".to_string(),
            reason: Box::new(e),
        })?;
        let face = ft
            .new_face(get_default_font(), 0)
            .map_err(|e| Error::InitError {
                component: "MetricsCalculator".into(),
                reason: Box::new(e),
            })?;
        face.set_char_size(12, 0, 50, 0)
            .map_err(|e| Error::InitError {
                component: "MetricsCalculator".into(),
                reason: Box::new(e),
            })?;
        Ok(MetricsCalculator { ft, face })
    }

    /// Compute height (in pixels) of the given text, rendered in the current (default) font.
    // TODO: figure out how to handle different fonts
    pub fn compute_height(&self, text: impl AsRef<str>) -> u64 {
        todo!();
    }

    /// Compute width (in pixels) of the given text, rendered in the current (default) font.
    // TODO: figure out how to handle different fonts.
    pub fn compute_width(&self, text: impl AsRef<str>) -> Result<i64> {
        // TODO: consider newlines
        let mut total = 0i64;
        let mut prev_char: Option<char> = None;
        for c in text.as_ref().chars() {
            self.face
                .load_char(c as usize, LoadFlag::NO_HINTING)
                .map_err(Error::FtError)?;
            if let Some(prev) = prev_char {
                let kern = self
                    .face
                    .get_kerning(prev as u32, c as u32, KerningMode::KerningDefault)
                    .map_err(Error::FtError)?;
                total += kern.x;
            }
            let metrics = self.face.glyph().metrics();
            total += metrics.horiAdvance;
            prev_char = Some(c);
        }

        Ok(total)
    }
}

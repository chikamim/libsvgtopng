// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate resvg;
extern crate libc;

use std::ffi::CStr;
use std::fs;
use std::io::Write;
use std::path;

use resvg::{
    usvg,
    svgdom,
    Options,
    FitTo,
    Render,
};

use usvg::prelude::*;

use svgdom::WriteBuffer;

macro_rules! bail {
    ($msg:expr) => {
        return Err(format!("{}", $msg));
    };
    ($fmt:expr, $($arg:tt)*) => {
        return Err(format!($fmt, $($arg)*));
    };
}

#[no_mangle]
pub extern "C" fn svg_to_png(svg_path: *const libc::c_char, png_path: *const libc::c_char) {
    let svg_path_buf = unsafe { CStr::from_ptr(svg_path).to_bytes() };
    let svg_path_str = String::from_utf8(svg_path_buf.to_vec()).unwrap();
    let png_path_buf = unsafe { CStr::from_ptr(png_path).to_bytes() };
    let png_path_str = String::from_utf8(png_path_buf.to_vec()).unwrap();
    if let Err(e) = process(svg_path_str, png_path_str) {
        // TODO: return err
    }
}

fn process(in_svg: String, out_png: String) -> Result<(), String> {
    let fit_to = FitTo::Original;
    let in_svg_path: path::PathBuf = in_svg.into();
    let out_png_path: path::PathBuf = out_png.into();
    let background = None;
    let dpi: f64 = 100 as f64; // TODO: arg
    let opt = Options {
            usvg: usvg::Options {
                path: Some(in_svg_path.clone().into()),
                dpi: dpi,
                keep_named_groups: true
            },
            fit_to,
            background,
        };

    let backend: Box<Render> = Box::new(resvg::backend_cairo::Backend);
    let tree = usvg::Tree::from_file(&in_svg_path, &opt.usvg).map_err(|e| e.to_string())?;
    let img = backend.render_to_image(&tree, &opt);
    match img {
        Some(img) => { img.save(&out_png_path); }
        None => { bail!("failed to allocate an image") }
    }

    Ok(())
}

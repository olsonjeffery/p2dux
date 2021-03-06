// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::vec::Vec;
use std::collections::HashMap;
use sdl2::sdl;
use sdl2::render::Renderer;
use sdl2::video::{WindowPos, Window, FullscreenType};
use p2d::sprite::SpriteSheet;
use sdl2;
use sdl2::pixels::Color;
use sdl2_image;

pub mod draw;
pub mod texture;

pub struct GameDisplay {
    pub renderer: Box<Renderer>,
    pub sheets: texture::TextureSheets
}

impl GameDisplay {
    pub fn new(title: &str, screen_size: (int, int, bool), ss: Vec<SpriteSheet>) -> GameDisplay {
        // first thing we do
        sdl::init(sdl2::INIT_VIDEO);
        // and sdl2_image
        sdl2_image::init(sdl2_image::INIT_PNG);

        let (width, height, fullscreen) = screen_size;
        let window = sdl2::video::Window::new(
            title, WindowPos::PosCentered, WindowPos::PosCentered,
            width, height, sdl2::video::OPENGL);
        let window = match window {
            Ok(window) => window,
            Err(err) => panic!(format!("failed to create window: {}", err))
        };
        if fullscreen {
            window.set_fullscreen(FullscreenType::FTTrue);
        }
        let renderer = Renderer::from_window(
            window, sdl2::render::RenderDriverIndex::Auto, sdl2::render::ACCELERATED);
        let renderer = match renderer {
            Ok(renderer) => renderer,
            Err(err) => panic!(format!("failed to create renderer: {}", err))
        };
        let mut display = GameDisplay {
            renderer: box renderer,
            sheets: HashMap::new()
        };
        // build TextureSheets
        for s in ss.iter() {
            display.sheets.insert(s.name.clone(), texture::TextureSheet::new(
                &*display.renderer,
                &s.path,
                s.name.clone()));
        }
        display
    }

    pub fn set_draw_color(&self, rgb: (u8, u8, u8)) {
        let (r, g, b) = rgb;
        match self.renderer.set_draw_color(Color::RGB(r, g, b)) {
            Ok(()) => {},
            Err(e) => panic!("set_draw_color(): failure: {}", e)
        }
    }
    pub fn set_draw_sdl2_color(&self, rgb: Color) {
        match self.renderer.set_draw_color(rgb) {
            Ok(()) => {},
            Err(e) => panic!("set_draw_sdl2_color(): failure: {}", e)
        }
    }
}

impl Drop for GameDisplay {
    fn drop(&mut self) {
        // last thing we do
        sdl2_image::quit();
        sdl::quit();
    }
}

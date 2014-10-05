// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::result::{Ok, Err};
use std::option::{Some};
use std::collections::HashMap;

use sdl2::surface::{Surface};
use sdl2::render::{Renderer, Texture};
use sdl2::rect::{Rect};
use sdl2::video::Window;

use p2d::sprite::SpriteTile;

pub struct TextureSheet {
    name: String,
    surface: Box<Surface>,
    texture: Box<Texture>
}

pub type TextureSheets = HashMap<String, TextureSheet>;

impl TextureSheet {
    pub fn new(renderer: &Renderer<Window>, path_str: &String, name: String) -> TextureSheet {
        let path = Path::new(path_str.as_slice());
        let surface = match Surface::from_bmp(&path) {
            Ok(s) => s,
            Err(msg) => fail!("new_sprite_from: Couldn't create surface from path '{}', msg: {}")
        };
        let surface = box surface;
        let texture = match renderer.create_texture_from_surface(&*surface) {
            Ok(t) => t,
            Err(msg) => fail!(format!("new_sprite_from: Couldn't create texture from path '{}', msg: {}", path_str, msg))
        };
        TextureSheet { name: name,
                     surface: surface, texture: box texture }
    }

    pub fn draw_tile(&self, renderer: &Renderer<Window>, st: &SpriteTile,
                     dst_coords: (int, int), dst_size: (uint, uint)) -> bool {
        //let (x, y) = dst;
        let (tile_x, tile_y) = st.coords;
        let (size_x, size_y) = st.size;
        let src = Some(Rect::new(tile_x as i32, tile_y as i32, size_x as i32, size_y as i32));
        let (dst_x, dst_y) = dst_coords;
        let (dst_size_x, dst_size_y) = dst_size;
        let dst = Some(Rect::new(dst_x as i32, dst_y as i32, dst_size_x as i32, dst_size_y as i32));
        // FIXME this is lame
        renderer.copy(&*self.texture, src, dst).is_ok()
    }
}

// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.
use std::any::Any;
use std::mem::transmute_copy;
use sdl2::event::Event;
use sdl2::event::Event::{TextInput, TextEditing, KeyDown};
use sdl2::keyboard::{start_text_input, stop_text_input, is_text_input_active};
use sdl2::keycode::*;

use ui::{compute_text_box_bounds, draw_text_box};

use super::super::ui::{UiFont, UiBox};
use super::{View, ViewContext};

pub struct TextInputDialogView<'a, TFont:'a, TBox:'a, TParent: 'a> {
    input_state: String,
    previous_state: String,
    prompt: String,
    cursor: String,
    bg_color: (u8, u8, u8),
    coords: (int, int),
    box_size: (uint, uint),
    text_gap: uint,
    ui_font: &'a TFont,
    ui_box: &'a TBox,
    started: bool,
    box_content: Vec<String>,
    parent: &'a mut TParent
}

pub struct DisplayClearerView {
    bg_color: (u8, u8, u8)
}

impl<'a, TFont: UiFont, TBox: UiBox, TParent: View> TextInputDialogView<'a, TFont, TBox, TParent> {
    pub fn new(
        ui_font: &'a TFont,
        ui_box: &'a TBox,
        seed_state: Option<String>,
        preface: &'a [String],
        prompt: String,
        cursor: String,
        bg_color: (u8,u8,u8),
        coords: (int, int),
        text_gap: uint,
        parent: &'a mut TParent)
            -> TextInputDialogView<'a, TFont, TBox, TParent> {
        let mut bc = Vec::new();
        bc.push_all(preface);
        bc.push("".to_string());

        let mut ret = TextInputDialogView {
            input_state: match seed_state { Some(i) => i, None => "".to_string() },
            previous_state: "".to_string(),
            prompt: prompt,
            cursor: cursor,
            bg_color: bg_color,
            coords: coords,
            box_size: (15,4),
            text_gap: text_gap,
            ui_font: ui_font,
            ui_box: ui_box,
            started: false,
            box_content: bc,
            parent: parent
        };
        ret.update_content();
        ret
    }
    
    fn update_content(&mut self) {
        self.box_content.pop();
        self.box_content.push("".to_string());
        let bc_last = self.box_content.len() - 1;
        self.box_content[bc_last].push_str(self.prompt.as_slice());
        self.box_content[bc_last].push_str(" ");
        let is_last = self.input_state.len();
        self.box_content[bc_last].push_str(self.input_state.slice(0, is_last));
        self.box_content[bc_last].push_str(self.cursor.as_slice());
        self.box_size = compute_text_box_bounds(self.box_content.as_slice(), self.ui_font, self.ui_box, self.text_gap);
    }
}

impl<'a, TFont: UiFont, TBox: UiBox, TParent: View>
        View for TextInputDialogView<'a, TFont, TBox, TParent> {
    fn get_parent(&mut self) -> Option<&mut View> {
        Some(&mut *self.parent as &mut View)
    }
    fn my_passive(&mut self, _ctx: &ViewContext, _time: u64) {}
    fn my_active(&mut self, ctx: &ViewContext, events: &[Event], time: u64) -> Option<Box<Any>> {
        // call into parent's passive
        self.parent_passive(ctx, time);

        if !self.started {
            self.started = true;
            start_text_input();
        }
        self.previous_state = self.input_state.clone();
        let mut out = None;
        for event in events.iter() {
            match *event {
                KeyDown(_, _, key, _, _, _) =>
                    match key {
                        KeyCode::Return => {
                            out = Some((box self.input_state.clone()) as Box<Any>);
                            stop_text_input();
                            break;
                        },
                        KeyCode::Backspace => {
                            self.input_state.pop();
                        }
                        KeyCode::Escape => {
                            let outval = (box "".to_string()) as Box<Any>;
                            out = Some(outval);
                            stop_text_input();
                            break;
                        },
                        _ => {}
                    },
                TextInput(_, _, ref txt) => {
                    out = None;
                    self.input_state.push_str(txt.as_slice());
                },
                _ => {}
            }
        }
        if self.previous_state != self.input_state {
            self.update_content();
        }
        draw_text_box(ctx.get_display(), self.coords, self.box_size, self.bg_color, self.box_content.as_slice(), self.ui_font,
                      self.ui_box, self.text_gap);
        out
    }
}

impl DisplayClearerView {
    pub fn new(bgc: (u8, u8, u8)) -> DisplayClearerView {
        DisplayClearerView { bg_color: bgc }
    }
}

impl View for DisplayClearerView {
    fn get_parent<'a>(&'a mut self) -> Option<&'a mut View> { None }
    fn my_active(&mut self, ctx: &ViewContext, events: &[Event], time: u64) -> Option<Box<Any>> { None }
    fn my_passive(&mut self, ctx: &ViewContext, _time: u64) {
        let display = ctx.get_display();
        display.set_draw_color(self.bg_color);
        match display.renderer.clear() {
            Err(e) => panic!("Display Clearer.update(): failed to clear display: {}", e),
            _ => {}
        }
    }
}

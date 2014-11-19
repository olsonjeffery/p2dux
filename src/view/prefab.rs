// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.
use sdl2::event::Event;
use sdl2::event::Event::{TextInput, TextEditing, KeyDown};
use sdl2::keyboard::{start_text_input, stop_text_input};
use sdl2::keycode::*;

use super::super::ui::{UiFont, UiBox};
use super::{ActiveView, PassiveView, DisplayViewContext};

pub struct TextInputDialogView<'a, TFont:'a, TBox:'a> {
    input_state: String,
    preface: &'a [String],
    prompt: String,
    cursor: String,
    bg_color: (u8, u8, u8),
    coords: (int, int),
    box_size: (uint, uint),
    text_gap: uint,
    ui_font: &'a TFont,
    ui_box: &'a TBox,
    started: bool
}

pub struct DisplayClearerPassiveView {
    bg_color: (u8, u8, u8)
}

impl<'a, TFont: UiFont, TBox: UiBox> TextInputDialogView<'a, TFont, TBox> {
    pub fn new(
        ui_font: &'a TFont,
        ui_box: &'a TBox,
        seed_state: Option<String>,
        preface: &'a [String],
        prompt: String,
        cursor: String,
        bg_color: (u8,u8,u8),
        coords: (int, int),
        text_gap: uint)
        -> TextInputDialogView<'a, TFont, TBox> {
            TextInputDialogView {
            input_state: match seed_state { Some(i) => i, None => "".to_string() },
            preface: preface,
            prompt: prompt,
            cursor: cursor,
            bg_color: bg_color,
            coords: coords,
            box_size: (0,0),
            text_gap: text_gap,
            ui_font: ui_font,
            ui_box: ui_box,
            started: false
        }
    }
}
impl<'a, TViewCtx: DisplayViewContext, TFont: UiFont, TBox: UiBox>
        ActiveView<TViewCtx, String> for TextInputDialogView<'a, TFont, TBox> {
    fn active_update<'a>(
        &'a mut self,
        _ctx: &TViewCtx,
        events: &[Event],
        _ms_time: u64,
        _passives: & mut Vec<&mut PassiveView<TViewCtx> >) -> Option<String> {
        //
        if !self.started {
            self.started = true;
            // call into SDL2 start text editing stuff
            start_text_input();
        }
        let mut out = None;
        for event in events.iter() {
            match *event {
                TextInput(_, _, ref txt) => {
                    out = None;
                    println!("TextInputEvent: {}", txt);
                    break;
                },
                TextEditing(_, _, ref txt, start, length) => {
                    out = None;
                    println!("TextEditingEvent: {}", txt);
                    break;
                },
                KeyDown(_, _, key, _, _) =>
                    match key {
                        Return => {
                            out = Some("YOLO".to_string());
                            println!("done editing text?");
                            stop_text_input();
                            break;
                        },
                        key => {
                            println!("pressed {} key", key);
                            out = None;
                            break;
                        }
                    },
                _ => {}
            }
        }
        out
    }
}

impl<'a, TViewCtx: DisplayViewContext, TFont: UiFont, TBox: UiBox>
        PassiveView<TViewCtx> for TextInputDialogView<'a, TFont, TBox> {
    fn passive_update(&mut self, _ctx: &TViewCtx, _t: u64) {
        // should never be passive, because the active_update() impl
        // doesn't yield_to() any other views, so we can put the logic
        // there
    }
}

impl DisplayClearerPassiveView {
    pub fn new(bgc: (u8, u8, u8)) -> DisplayClearerPassiveView {
        DisplayClearerPassiveView { bg_color: bgc }
    }
}

impl<TViewCtx: DisplayViewContext> PassiveView<TViewCtx> for DisplayClearerPassiveView {
    fn passive_update(&mut self, ctx: &TViewCtx, _time: u64) {
        let display = ctx.get_display();
        display.set_draw_color(self.bg_color);
        match display.renderer.clear() {
            Err(e) => panic!("Display Clearer.update(): failed to clear display: {}", e),
            _ => {}
        }
    }
}

impl<TViewCtx: DisplayViewContext> ActiveView<TViewCtx, ()>
        for DisplayClearerPassiveView {
    fn active_update<'a>(&'a mut self, _ctx: &TViewCtx, _e: &[Event], _t: u64,
              _p: &mut Vec<&mut PassiveView<TViewCtx> >)
        -> Option<()> {
            panic!("this should never be called.");
    }
}

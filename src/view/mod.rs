// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.
use std::io::timer;
use std::any::Any;
use std::comm::channel;
use std::mem::{transmute, transmute_copy};
use time::precise_time_ns;
use std::time::duration::Duration;

use sdl2::event::poll_event;
use sdl2::event::Event;

use gfx::GameDisplay;

pub mod prefab;

pub fn throttle(fps: uint, cb: || -> bool) {
    let target_fps = (1000 / fps) as u64;
    loop {
        let next_frame = (precise_time_ns() / 1000000) + target_fps;
        match cb() {
            false => break,
            _ => {}
        }
        let now_time = precise_time_ns() / 1000000;
        if  now_time < next_frame {
            let sleep_gap = next_frame - now_time;
            timer::sleep(Duration::milliseconds(sleep_gap as i64));
        }
    }
}

// View

pub struct ViewContext {
    display: GameDisplay
}

impl ViewContext {
    pub fn new(display: GameDisplay) -> ViewContext {
        ViewContext { display: display }
    }
    
    pub fn get_display<'a>(&'a self) -> &'a GameDisplay {
        &self.display
    }
}

impl<'a, TView: View> View for &'a mut TView {
    fn get_parent<'a>(&'a mut self) -> Option<&'a mut View> {
        (*self).get_parent()
    }
    fn my_active(&mut self, ctx: &ViewContext, events: &[Event], time: u64) -> Option<Box<Any>> {
        (*self).my_active(ctx, events, time)
    }
    fn my_passive(&mut self, ctx: &ViewContext, time: u64) {
        (*self).my_passive(ctx, time);
    }
}

pub trait View {
    fn get_parent<'a>(&'a mut self) -> Option<&'a mut View>;
    fn my_active<'a>(&'a mut self, ctx: &ViewContext, events: &[Event], time: u64) -> Option<Box<Any>>;
    fn my_passive(& mut self, ctx: &ViewContext, time: u64);
    fn parent_passive(&mut self, ctx: &ViewContext, time: u64) {
        match self.get_parent() {
            Some(parent) => parent.my_passive(ctx, time),
            None => {}
        }
    }
    fn enter(&mut self, ctx: &ViewContext) -> Box<Any> {
        let mut cont = true;
        let mut events = Vec::new();
        let mut output: Option<Box<Any>> = None;
        while cont {
            let result = {
                let time = precise_time_ns() / 1000000;
                self.my_passive(ctx, time);
                loop {
                    match poll_event() {
                        Event::None => { break; },
                        event => { events.push(event); }
                    }
                }
                let time = precise_time_ns() / 1000000;
                let result = self.my_active(ctx, events.as_slice(), time);
                ctx.get_display().renderer.present();
                events.clear();
                result
            };
            match result {
                None => {},
                r => {
                    cont = false;
                    output = r;
                }
            }
        }
        output.expect("View.enter(): Should always be a Some here")
    }
}

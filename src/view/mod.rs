// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.
use std::io::timer;
use std::comm::channel;
use std::mem::{transmute, transmute_copy};
use time::precise_time_ns;
use std::time::duration::Duration;

use sdl2::event::poll_event;
use sdl2::event::Event;
use sdl2::event::Event::*;

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

pub trait DisplayViewContext {
    fn get_display<'a>(&'a self) -> &'a GameDisplay;
}

// ViewManager API exploration
pub trait ActiveView<TViewCtx: DisplayViewContext, TOut: Send> : PassiveView<TViewCtx> {
    fn active_update<'a>(&'a mut self, ctx: &TViewCtx, events: &[Event], ms_time: u64,
                         passives: &mut Vec<& mut PassiveView<TViewCtx> >)
                         -> Option<TOut>;
    fn yield_to<'a, TOut: Send, TActive: ActiveView<TViewCtx, TOut>>(&'a mut self, ctx: &TViewCtx,
                                active: &mut TActive,
                                passives: &mut Vec<&mut PassiveView<TViewCtx>>) -> TOut {
        let (sender, receiver) = channel();
        let mut cont = true;
        while cont {
            let time = precise_time_ns() / 1000000;
            let mut events = Vec::new();
            {
                for view in passives.iter_mut() {
                    view.passive_update(ctx, time);
                }
            }
            self.passive_update(ctx, time);
            active.passive_update(ctx, time);
            loop {
                match poll_event() {
                    Event::None => { break; },
                    event => { events.push(event); }
                }
            }
            let result = active.active_update(ctx, events.as_slice(), time, passives);
            ctx.get_display().renderer.present();
            if result.is_some() {
                cont = false;
                sender.send(result.expect("definitely gonna be something!"));
            }
        }
        receiver.recv()
    }
}

pub trait PassiveView<TViewCtx: DisplayViewContext> {
    fn passive_update(&mut self, ctx: &TViewCtx, ms_time: u64);
}

pub struct PassiveBox<'a, TActive: 'a> {
    inner: &'a mut TActive
}

impl<'a, TViewCtx: DisplayViewContext, TOut: Send, TActive: ActiveView<TViewCtx, TOut>> PassiveBox<'a, TActive> {
    pub fn new(item: &'a mut TActive) -> PassiveBox<'a, TActive> {
        PassiveBox { inner: item }
    }
}

impl<'a, TViewCtx: DisplayViewContext, TOut: Send, TActive: ActiveView<TViewCtx, TOut>> PassiveView<TViewCtx> for PassiveBox<'a, TActive> {
    fn passive_update(&mut self, ctx: &TViewCtx, ms_time: u64) {
        self.inner.passive_update(ctx, ms_time);
    }
}

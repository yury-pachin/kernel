use std::io::{self, Write};
use io::poll::{Poll, Events};
use io::token::Token;
use io::ready::Ready;
use io::options::PollOpt;
use std::collections::HashMap;
use io::event::Evented;
use std::cell::UnsafeCell;
use reactors::core::Core;
use reactors::ws::WsServer;
use reactors::console::Console;
use core::borrow::BorrowMut;
use handle;
use std::fmt::Arguments;
use queues::publisher::Subscriber;
use streams::intercore::api::Message;

const EVENTS_CAPACITY: usize = 1024;
const SUBSCRIBERS_CAPACITY: usize = 16;
const READ_BUF_SIZE: usize = 2048;

#[derive(Debug)]
pub enum Async<T> {
    Ready(T),
    NotReady,
}

pub struct Pool<'a, T: 'a>(pub &'a [T]);

pub trait Select<'a, T>: Write {
    fn init(&mut self, c: &mut Core, s: Slot);
    fn select(&'a mut self, c: &'a mut Core, t: Token) -> Async<Pool<'a, T>>;
    fn finalize(&mut self);
}

pub fn with_selector<'a, S, T, F, R>(s: &'a mut S, mut f: F) -> R
    where S: Select<'a, T>,
          F: FnMut(&mut S) -> R
{
    f(s)
}

pub enum Selector {
    Ws(WsServer),
    Rx(Console),
    Sb(Subscriber<Message>),
}

impl Selector {
    pub fn with<F, R>(&mut self, mut f: F) -> R
        where F: FnMut(&mut Self) -> R
    {
        f(self)
    }
}

#[macro_export]
macro_rules! with(
    ($x:expr,$e:expr) => ({
        match *$x {
            Selector::Ws(ref mut w) => with_selector(w, $e),
            Selector::Rx(ref mut c) => with_selector(c, $e),
            Selector::Sb(ref mut s) => with_selector(s, $e), 
        }
    })
);

impl Write for Selector {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        with!(self, |x| x.write(buf));
        Ok(1)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        Ok(())
    }
    fn write_fmt(&mut self, fmt: Arguments) -> io::Result<()> {
        Ok(())
    }
}

#[derive(Debug,PartialEq,Clone,Copy)]
pub struct Slot(pub usize);

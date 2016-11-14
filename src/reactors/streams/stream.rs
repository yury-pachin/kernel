// Pivot Stream Trait
// All copyrights are removed since Max prohibited it.

use reactors::combinators::map::{self, Map};
use reactors::combinators::then::{self, Then};
use super::into_stream::IntoStream;

pub enum Async<T> {
    Ready(T),
    NotReady,
}

pub type Poll<T, E> = Result<Option<Async<T>>, E>;

pub trait Stream {
    type Item;
    type Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error>;

    fn map<U, F>(self, f: F) -> Map<Self, F>
        where F: FnMut(Self::Item) -> U,
              Self: Sized
    {
        map::new(self, f)
    }

    fn then<F, U>(self, f: F) -> Then<Self, F, U>
        where F: FnMut(Result<Self::Item, Self::Error>) -> U,
              U: IntoStream,
              Self: Sized
    {
        then::new(self, f)
    }
}

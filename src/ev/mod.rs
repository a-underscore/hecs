pub mod control;

pub use control::Control;

use glium::Frame;

pub enum Ev<'a, 'b, 'c> {
    Event(&'a mut Control<'b>),
    Draw((&'a mut Control<'b>, &'c mut Frame)),
}

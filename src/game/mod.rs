pub mod player;
pub mod room;
pub mod command;

pub trait Describable {
    fn describe(&self) -> String;
}
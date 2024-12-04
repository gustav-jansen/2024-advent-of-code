use std::any::Any;
use std::fmt;

pub trait Token: Any + fmt::Display {
    fn as_any(&self) -> &dyn Any;
}

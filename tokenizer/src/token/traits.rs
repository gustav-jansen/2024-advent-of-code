use std::any::Any;
use std::fmt;

use crate::visitor::TokenVisitor;

pub trait Token: Any + fmt::Display {
    fn as_any(&self) -> &dyn Any;
    fn accept(&self, visitor: &mut dyn TokenVisitor);
}

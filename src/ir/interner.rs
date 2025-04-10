use std::{cell::RefCell, rc::Rc};

use string_interner::{StringInterner, backend::StringBackend, symbol::DefaultSymbol};

pub type Symbol = DefaultSymbol;
pub type Interner = StringInterner<StringBackend<Symbol>>;

pub trait AsRcRefCell {
    fn as_rc_refcell() -> Rc<RefCell<Self>>
    where
        Self: Sized;
}

impl AsRcRefCell for Interner {
    fn as_rc_refcell() -> Rc<RefCell<Self>>
    where
        Self: Sized,
    {
        Rc::new(RefCell::new(Self::default()))
    }
}

use super::{
    IrEnumVariant, IrType,
    interner::{Interner, Symbol},
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, PartialEq, Clone)]
pub enum SymbolKind<'i> {
    Variable {
        name: &'i str,
        ty: Option<IrType>,
    },
    Function {
        name: &'i str,
        params: Vec<(&'i str, Option<IrType>)>,
        return_type: Option<IrType>,
    },
    TypeAlias {
        name: &'i str,
        ty: IrType,
    },
    Struct {
        name: &'i str,
        fields: Vec<(&'i str, IrType)>,
    },
    Enum {
        name: &'i str,
        variants: Vec<IrEnumVariant>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct SymbolTable<'i> {
    parent: Option<Rc<SymbolTable<'i>>>,
    symbols: HashMap<Symbol, SymbolKind<'i>>,
    pub interner: Rc<RefCell<Interner>>,
}

impl<'i> SymbolTable<'i> {
    pub fn new(interner: Rc<RefCell<Interner>>) -> Self {
        Self {
            parent: None,
            symbols: HashMap::default(),
            interner,
        }
    }

    pub fn with_parent(parent: Rc<SymbolTable<'i>>) -> Self {
        let interner = parent.interner.clone();
        Self {
            parent: Some(parent),
            symbols: HashMap::new(),
            interner,
        }
    }

    pub fn insert(&mut self, name: &'i str, kind: SymbolKind<'i>) -> Option<SymbolKind<'i>> {
        let sym = {
            let mut interner = self.interner.borrow_mut();
            interner.get_or_intern(name)
        };
        self.symbols.insert(sym, kind)
    }

    pub fn get_or_intern(&self, name: &str) -> Symbol {
        self.interner.borrow_mut().get_or_intern(name)
    }

    pub fn get(&self, name: &str) -> Option<&SymbolKind<'i>> {
        let interner = self.interner.borrow();
        if let Some(sym) = interner.get(name) {
            if let Some(kind) = self.symbols.get(&sym) {
                return Some(kind);
            }
        }

        self.parent.as_ref()?.get(name)
    }
    pub fn contains(&self, name: &str) -> bool {
        self.get(name).is_some()
    }
    pub fn all_symbols(&self) -> impl Iterator<Item = (&Symbol, &SymbolKind<'i>)> {
        self.symbols.iter()
    }
    pub fn expect(&self, name: &str) -> Result<&SymbolKind<'i>, String> {
        self.get(name)
            .ok_or_else(|| format!("Unresolved symbol: `{}`", name))
    }
}

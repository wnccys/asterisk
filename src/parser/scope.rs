use std::{cell::RefCell, rc::Rc};

use crate::{objects::hash_table::HashTable, primitives::types::Modifier};

/// General scope handler.
///
#[derive(Debug)]
pub struct Scope {
    /// Represents all local variables, resolved dynamically at runtime, without a Constant Bytecode.
    ///
    /// (Var position on locals [consequently on Stack], Modifier)
    pub locals: HashTable<String, (usize, Modifier)>,
    pub local_count: usize,
}

/// Represent a block scope
/// 
impl Scope {
    /// Add new Local by hashing and inserting it
    /// 
    pub fn add_local(&mut self, lexeme: String, modifier: Modifier, total_locals: usize) {
        self.locals.insert(&lexeme, (total_locals, modifier));
        self.local_count += 1;
    }

    /// Return Local index to be used by stack if it exists
    /// 
    pub fn get_local(&self, lexeme: &String) -> Option<Rc<RefCell<(usize, Modifier)>>> {
        self.locals.get(lexeme)
    }
}

impl<'a> Default for Scope {
    fn default() -> Self {
        Scope {
            locals: HashTable::default(),
            local_count: 0,
        }
    }
}
#![deny(missing_docs)]

use std::collections::HashMap;

use super::variable::*;



/// Script's scope.
pub struct Scope<'p> {
    parent: Option<&'p mut Scope<'p>>,
    variables: HashMap<String, Variable>,
}



impl<'p> Scope<'p> {

    /// Creates a new scope
    pub fn new() -> Scope<'p> {
        Scope {
            parent: None,
            variables: HashMap::new(),
        }
    }
    /// Creates a new scope with a parent scope.
    pub fn extends(p: &'p mut Scope<'p>) -> Scope<'p> {
        Scope {
            parent: Some(p),
            variables: HashMap::new(),
        }
    }



    /// Gets the given variable.
    pub fn get_variable(&self, name: &String) -> Option<&Variable> {
        self.variables.get(name)
    }
    /// Gets the given variable.
    pub fn get_variable_mut(&mut self, name: &String) -> Option<&mut Variable> {
        self.variables.get_mut(name)
    }

}

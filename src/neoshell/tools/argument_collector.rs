#![deny(missing_docs)]

use std::collections::HashMap;

use crate::neoshell::parser::*;


/// Structure collecting argument for easier validation and access.
pub struct ArgumentCollector<'a> {
    /// Positional arguments.
    pub positionals: Vec<&'a AstArgument>,
    /// Switch arguments.
    pub switches: HashMap<String, Vec<&'a AstSwitch>>,
}


impl<'a> ArgumentCollector<'a> {

    /// Creates a new argument collector.
    pub fn new() -> ArgumentCollector<'a> {
        ArgumentCollector {
            positionals: Vec::new(),
            switches: HashMap::new(),
        }
    }



    /// Collects the argument from the given iterator.
    pub fn collect<I>(&mut self, iter: I)
    where
        I: Iterator<Item = &'a AstArgument>
    {
        for arg in iter {
            match &arg {
                AstArgument::Switch(s) => {
                    let mut name: &String;

                    match &s {
                        AstSwitch::On(n) => name = n,
                        AstSwitch::Off(n) => name = n,
                        AstSwitch::Option(n, _) => name = n,
                    }

                    self.switches.entry(name.clone())
                        .and_modify(|v| v.push(s))
                        .or_insert(vec![s]);
                },
                _ => self.positionals.push(arg),
            }
        }
    }

}

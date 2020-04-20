#![deny(missing_docs)]


use crate::neoshell::parser::ast::*;


/// A variable.
pub enum Variable {
    /// The variable is a value.
    Value(AstArgument),
    /// The variable is an instanced command.
    Command()
}



impl Variable {

    /// Gets the variable as a name.
    pub fn get_name(&self) -> Option<&AstName> {
        if let Variable::Value(v) = self {
            if let AstArgument::Name(n) = v {
                return Some(n);
            }
        }

        None
    }
    /// Gets the variable as an integer.
    pub fn get_integer(&self) -> Option<&i32> {
        if let Variable::Value(v) = self {
            if let AstArgument::Integer(i) = v {
                return Some(i);
            }
        }

        None
    }
    /// Gets the variable as a float.
    pub fn get_float(&self) -> Option<&f32> {
        if let Variable::Value(v) = self {
            if let AstArgument::Float(f) = v {
                return Some(f);
            }
        }

        None
    }
    /// Gets the variable as a string.
    pub fn get_string(&self) -> Option<&String> {
        if let Variable::Value(v) = self {
            if let AstArgument::String(s) = v {
                return Some(s);
            }
        }

        None
    }
    /// Gets the variable as a switch.
    pub fn get_switch(&self) -> Option<&AstSwitch> {
        if let Variable::Value(v) = self {
            if let AstArgument::Switch(s) = v {
                return Some(s);
            }
        }

        None
    }
    /// Gets the variable as a block.
    pub fn get_block(&self) -> Option<&AstBlock> {
        if let Variable::Value(v) = self {
            if let AstArgument::Block(b) = v {
                return Some(b);
            }
        }

        None
    }



    /// Gets the variable as a name.
    pub fn get_name_mut(&mut self) -> Option<&mut AstName> {
        if let Variable::Value(v) = self {
            if let AstArgument::Name(n) = v {
                return Some(n);
            }
        }

        None
    }
    /// Gets the variable as an integer.
    pub fn get_integer_mut(&mut self) -> Option<&mut i32> {
        if let Variable::Value(v) = self {
            if let AstArgument::Integer(i) = v {
                return Some(i);
            }
        }

        None
    }
    /// Gets the variable as a float.
    pub fn get_float_mut(&mut self) -> Option<&mut f32> {
        if let Variable::Value(v) = self {
            if let AstArgument::Float(f) = v {
                return Some(f);
            }
        }

        None
    }
    /// Gets the variable as a string.
    pub fn get_string_mut(&mut self) -> Option<&mut String> {
        if let Variable::Value(v) = self {
            if let AstArgument::String(s) = v {
                return Some(s);
            }
        }

        None
    }
    /// Gets the variable as a switch.
    pub fn get_switch_mut(&mut self) -> Option<&mut AstSwitch> {
        if let Variable::Value(v) = self {
            if let AstArgument::Switch(s) = v {
                return Some(s);
            }
        }

        None
    }
    /// Gets the variable as a block.
    pub fn get_block_mut(&mut self) -> Option<&mut AstBlock> {
        if let Variable::Value(v) = self {
            if let AstArgument::Block(b) = v {
                return Some(b);
            }
        }

        None
    }

}

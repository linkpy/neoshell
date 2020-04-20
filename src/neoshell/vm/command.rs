#![deny(missing_docs)]


use crate::neoshell::parser::*;
use super::vm::VM;
use super::scope::Scope;

use failure::Error;



/// Compile time command executor.
///
pub trait CompileTimeCommand {
    /// Executes the command.
    fn execute(&mut self, vm: &mut VM, scp: &mut Scope, cmd: &AstCommand) -> Option<Error>;
}

/// Macro command executor.
///
pub trait MacroCommand {
    /// Executes the command.
    fn execute(&self, vm: &VM, scp: &mut Scope, cmd: &AstCommand) -> Result<Vec<AstCommand>, Error>;
}

/// Runtime command executor.
///
pub trait RuntimeCommand {
    /// Executes the command.
    fn execute(&self, vm: &VM, scp: &mut Scope, cmd: &AstCommand) -> Result<AstArgument, Error>;
}

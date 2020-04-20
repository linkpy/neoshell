#![deny(missing_docs)]


use failure::Error;

use crate::neoshell::parser::*;

use super::command::*;
use super::scope::*;
use super::registery::*;


/// Virtual machine executing Neoshell code.
pub struct VM {
    registery: CommandRegistery,
}



impl VM {

    /// Creates a new VM object.
    pub fn new() -> VM {
        VM {
            registery: CommandRegistery::new()
        }
    }


    /// Gets the command registery;
    pub fn get_registery_mut(&mut self) -> &mut CommandRegistery {
        &mut self.registery
    }
    /// Gets the command registery;
    pub fn get_registery(&self) -> &CommandRegistery {
        &self.registery
    }


    /// Executes a compile time command.
    pub fn execute_ct_command(&mut self, scp: &mut Scope, cmd: &AstCommand) -> Option<Error> {
        if cmd.time != AstTime::CompileTime {
            panic!("The given command is not a compile time command.");
        }

        let cmd_name: &String;

        match &cmd.name {
            AstName::Placeholder => panic!("Can't execute placeholder commands."),
            AstName::Name(n) => cmd_name = n,
            AstName::Variable(_) => unimplemented!("Instanciated commands"),
        }

        let copt = self.registery.get_ct_command_move(cmd_name);

        match copt {
            None => panic!(format!("Compile time command '{}' not found.", cmd_name)),
            Some(mut c) => {
                let r = c.execute(self, scp, cmd);
                self.registery.register_ct_boxed(cmd_name, c);
                r
            }
        }
    }
    /// Executes a macro command.
    pub fn execute_macro(&self, scp: &mut Scope, cmd: &AstCommand) -> Result<Vec<AstCommand>, Error> {
        let cmd_name: &String;

        match &cmd.name {
            AstName::Placeholder => panic!("Can't execute placeholder commands."),
            AstName::Name(n) => cmd_name = n,
            AstName::Variable(_) => panic!("Can't uses variables as macros."),
        }

        let copt = self.registery.get_macro(cmd_name);

        match copt {
            None => panic!(format!("Macro '{}' not found.", cmd_name)),
            Some(c) => c.execute(self, scp, cmd)
        }
    }
    /// Executes a command.
    pub fn execute_command(&self, scp: &mut Scope, cmd: &AstCommand) -> Result<AstArgument, Error> {
        let cmd_name: &String;

        match &cmd.name {
            AstName::Placeholder => panic!("Can't execute placeholder commands."),
            AstName::Name(n) => cmd_name = n,
            AstName::Variable(_) => unimplemented!("Instanciated commands"),
        }

        let copt = self.registery.get_command(cmd_name);

        match copt {
            None => panic!(format!("Command '{}' not found.", cmd_name)),
            Some(c) => c.execute(self, scp, cmd)
        }
    }

}

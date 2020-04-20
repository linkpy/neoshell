

use std::collections::HashMap;

use super::command::*;


/// Registery of commands. All commands are registered in global scope.
///
pub struct CommandRegistery {
    ct_commands: HashMap<String, Box<dyn CompileTimeCommand>>,
    macros: HashMap<String, Box<dyn MacroCommand>>,
    commands: HashMap<String, Box<dyn RuntimeCommand>>,
}


impl CommandRegistery {

    /// Constructs a new registery with no commands.
    pub fn new() -> CommandRegistery {
        CommandRegistery {
            ct_commands: HashMap::new(),
            macros: HashMap::new(),
            commands: HashMap::new(),
        }
    }


    /// Registers a compile time command.
    pub fn register_ct(&mut self, name: &String, cmd: impl CompileTimeCommand + 'static) {
        self.ct_commands.insert(name.clone(), Box::new(cmd));
    }
    /// Registers a macro command.
    pub fn register_macro(&mut self, name: &String, cmd: impl MacroCommand + 'static) {
        self.macros.insert(name.clone(), Box::new(cmd));
    }
    /// Registers a runtime command.
    pub fn register(&mut self, name: &String, cmd: impl RuntimeCommand + 'static) {
        self.commands.insert(name.clone(), Box::new(cmd));
    }
    /// Registers a compile time command.
    pub fn register_ct_boxed(&mut self, name: &String, cmd: Box<dyn CompileTimeCommand>) {
        self.ct_commands.insert(name.clone(), cmd);
    }
    /// Registers a macro command.
    pub fn register_macro_boxed(&mut self, name: &String, cmd: Box<dyn MacroCommand>) {
        self.macros.insert(name.clone(), cmd);
    }
    /// Registers a runtime command.
    pub fn register_boxed(&mut self, name: &String, cmd: Box<dyn RuntimeCommand>) {
        self.commands.insert(name.clone(), cmd);
    }


    /// Gets the given compile time command.
    pub fn get_ct_command_mut(&mut self, name: &String) -> Option<&mut Box<dyn CompileTimeCommand>> {
        self.ct_commands.get_mut(name)
    }
    /// Gets the given macro command.
    pub fn get_macro_mut(&mut self, name: &String) -> Option<&mut Box<dyn MacroCommand>> {
        self.macros.get_mut(name)
    }
    /// Gets the given runtime command.
    pub fn get_command_mut(&mut self, name: &String) -> Option<&mut Box<dyn RuntimeCommand>> {
        self.commands.get_mut(name)
    }
    /// Gets the given compile time command.
    pub fn get_ct_command(&self, name: &String) -> Option<&Box<dyn CompileTimeCommand>> {
        self.ct_commands.get(name)
    }
    /// Gets the given macro command.
    pub fn get_macro(&self, name: &String) -> Option<&Box<dyn MacroCommand>> {
        self.macros.get(name)
    }
    /// Gets the given runtime command.
    pub fn get_command(&self, name: &String) -> Option<&Box<dyn RuntimeCommand>> {
        self.commands.get(name)
    }


    /// Gets the given compile time command.
    pub fn get_ct_command_move(&mut self, name: &String) -> Option<Box<dyn CompileTimeCommand>> {
        self.ct_commands.remove(name)
    }
    /// Gets the given macro command.
    pub fn get_macro_move(&mut self, name: &String) -> Option<Box<dyn MacroCommand>> {
        self.macros.remove(name)
    }
    /// Gets the given runtime command.
    pub fn get_command_move(&mut self, name: &String) -> Option<Box<dyn RuntimeCommand>> {
        self.commands.remove(name)
    }

}

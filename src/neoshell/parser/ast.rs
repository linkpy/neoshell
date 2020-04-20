#![deny(missing_docs)]



/// AST Command.
pub struct AstCommand {
    /// Execution time of the command.
    pub time: AstTime,
    /// Name of the command.
    pub name: AstName,
    /// Argument of the command.
    pub arguments: Vec<AstArgument>,
}

/// Command execution time.
#[derive(PartialEq)]
pub enum AstTime {
    /// Command executed at compile time.
    CompileTime,
    /// Command execute at compile time, whos result is inserted in the AST
    Macro,
    /// Command executed at runtime.
    Runtime,
}

/// Argument for a command.
pub enum AstArgument {
    /// No arguments.
    None,
    /// A name/word.
    Name(AstName),
    /// An integer literal.
    Integer(i32),
    /// A float literal.
    Float(f32),
    /// A string literal.
    String(String),
    /// A command switch.
    Switch(AstSwitch),
    /// A command block.
    Block(AstBlock),
}

/// A command switch.
pub enum AstSwitch {
    /// An enabling switch.
    On(String),
    /// A disabling switch.
    Off(String),
    /// An option switch, associated with a value
    Option(String, Box<AstArgument>)
}

/// A command block.
pub enum AstBlock {
    /// An evaluated block, whos result is passed to the command.
    Evaluated(Vec<AstCommand>),
    /// A static block, passed directly to the command as an argument.
    Argument(Vec<AstCommand>),
}

/// A name in the AST.
pub enum AstName {
    /// Placeholder name, used by macros.
    Placeholder,
    /// A regular name.
    Name(String),
    /// A variable.
    Variable(String),
}




impl AstCommand {

    /// Creates a new command without any arguments.
    ///
    pub fn new(time: AstTime, name: AstName) -> AstCommand {
        AstCommand {
            time: time,
            name: name,
            arguments: Vec::new(),
        }
    }
    /// Creates a new command from the given one, using other arguments.
    ///
    pub fn extends(cmd: AstCommand, arguments: Vec<AstArgument>) -> AstCommand {
        AstCommand {
            time: cmd.time,
            name: cmd.name,
            arguments: arguments,
        }
    }


    /// Dumps the AST.
    ///
    pub fn dump(&self, i: String) {
        print!("{}Command( ", i);

        match self.time {
            AstTime::CompileTime => println!("CompileTime )"),
            AstTime::Macro       => println!("Macro )"),
            AstTime::Runtime     => println!("Runtime )"),
        }

        println!("{}  Name :", &i);
        self.name.dump(format!("{}    ", i));

        println!("{}  Arguments :", &i);
        for a in self.arguments.iter() {
            a.dump(format!("{}    ", i));
        }
    }

}

impl AstArgument {

    /// Dumps the AST.
    ///
    pub fn dump(&self, i: String) {
        match self {
            AstArgument::None       => println!("{}None", i),
            AstArgument::Name(v)    => v.dump(i),
            AstArgument::Integer(v) => println!("{}Integer( {} )", i, v),
            AstArgument::Float(v)   => println!("{}Float( {} )", i, v),
            AstArgument::String(v)  => println!("{}String( {} )", i, v),
            AstArgument::Switch(v)  => v.dump(i),
            AstArgument::Block(v)   => v.dump(i),
        }
    }

}

impl AstSwitch {

    /// Dumps the AST.
    ///
    pub fn dump(&self, i: String) {
        match self {
            AstSwitch::On(n)        => println!("{}OnSwitch( {} )", i, n),
            AstSwitch::Off(n)       => println!("{}OffSwitch( {} )", i, n),
            AstSwitch::Option(n, v) => {
                println!("{}OptionSwitch( {} )", i, n);
                v.dump(format!("{}  ", i));
            }
        }
    }

}

impl AstBlock {

    /// Dumps the AST.
    ///
    pub fn dump(&self, i: String) {
        match self {
            AstBlock::Evaluated(v) => {
                println!("{}EvaluatedBlock", i);
                for c in v {
                    c.dump(format!("{}  ", i));
                }
            },
            AstBlock::Argument(v) => {
                println!("{}ArgumentBlock", i);
                for c in v {
                    c.dump(format!("{}  ", i));
                }
            }
        }
    }

}

impl AstName {

    /// Dumps the AST.
    ///
    pub fn dump(&self, i: String) {
        match self {
            AstName::Placeholder => println!("{}PlaceholderName", i),
            AstName::Name(v)     => println!("{}Name( {} )", i, v),
            AstName::Variable(v) => println!("{}Variable( {} )", i, v),
        }
    }

}


mod neoshell;

use neoshell::parser::*;
use neoshell::vm::*;

use failure::Error;


const SRC: &'static str = r#"

puts "hello world";
puts "this is from the script";
puts "wooooohoooooo !!!";

"#;

#[macro_use] extern crate failure;



struct PutsCommand { }
impl RuntimeCommand for PutsCommand {
    fn execute(&self, vm: &VM, scp: &mut Scope, cmd: &AstCommand) -> Result<AstArgument, Error>
    {
        assert_eq!(cmd.arguments.len(), 1);

        match &cmd.arguments[0] {
            AstArgument::String(s) => println!("{}", s),
            _ => panic!("invalid arg"),
        }

        Ok(AstArgument::None)
    }
}


fn main() {
    let ast = ns_parser::file(SRC);
    let mut vm = VM::new();
    vm.get_registery_mut().register(&"puts".to_owned(), PutsCommand{});
    let mut scp = Scope::new();

    match ast {
        Ok(v) => {
            for c in v {
                vm.execute_command(&mut scp, &c).expect("oof");
            }
        }
        Err(e) => println!("{}", e)
    }
}

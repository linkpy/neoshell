
use super::ast::*;


peg::parser!{
    pub grammar ns_parser() for str {
        pub rule file() -> Vec<AstCommand>
            = cl:(command()+) eof() { cl }

        pub rule command() -> AstCommand
            = _ c:a_command() _ al:(argument()*) _ ";" _ { AstCommand::extends(c, al) }

        rule a_command() -> AstCommand
            = c:ct_command()    { c }
            / c:mac_command()   { c }
            / c:rt_command()    { c }
        rule ct_command() -> AstCommand
            = "!" n:name()      { AstCommand::new(AstTime::CompileTime, n) }
        rule mac_command() -> AstCommand
            = n:name() "!"      { AstCommand::new(AstTime::Macro, n) }
        rule rt_command() -> AstCommand
            = n:name()          { AstCommand::new(AstTime::Runtime, n) }



        pub rule argument() -> AstArgument
            = _ a:an_argument() _ { a }

        rule an_argument() -> AstArgument
            = a:name()                              { AstArgument::Name(a) }
            / a:integer()                           { AstArgument::Integer(a) }
            / a:float()                             { AstArgument::Float(a) }
            / a:string()                            { AstArgument::String(a) }
            / a:switch()                            { AstArgument::Switch(a) }
            / a:block()                             { AstArgument::Block(a) }
        pub rule integer() -> i32
            = v:$(digit()+)                         { v.parse().unwrap() }
        pub rule float() -> f32
            = v:$(digit()+ "." digit()+)            { v.parse().unwrap() }

        pub rule string() -> String
            = "\"" v:$((!"\"" string_char())*) "\""  { v.to_string() }
        rule string_char()
            = "\\" (" " / "\t" / "\n" / "\r" / "\"")
            / !"\\" [_]

        pub rule switch() -> AstSwitch
            = s:on_switch()                   { s }
            / s:off_switch()                  { s }
            / s:option_switch()               { s }
        rule on_switch() -> AstSwitch
            = "+" n:identifier()              { AstSwitch::On(extract_name(n)) }
        rule off_switch() -> AstSwitch
            = "-" n:identifier()              { AstSwitch::Off(extract_name(n)) }
        rule option_switch() -> AstSwitch
            = "/" n:identifier() a:argument() { AstSwitch::Option(extract_name(n), Box::new(a)) }

        pub rule block() -> AstBlock
            = b:eval_block() { b }
            / b:arg_block()  { b }
        rule eval_block() -> AstBlock
            = "{" cl:(command()*) "}" { AstBlock::Evaluated(cl) }
        rule arg_block() -> AstBlock
            = "&{" cl:(command()*) "}" { AstBlock::Argument(cl) }



        pub rule name() -> AstName
            = n:identifier()  { n }
            / n:variable()    { n }
            / n:placeholder() { n }

        rule identifier() -> AstName
            = id:$(identifier_start() (identifier_continue() / "::")*) { AstName::Name(id.to_string()) }
        rule variable() -> AstName
            = "$" v:$(digit()+)                                        { AstName::Variable(v.to_string()) }
            / "$" id:$(identifier_start() identifier_continue()*)      { AstName::Variable(id.to_string()) }
        rule placeholder() -> AstName
            = "~"                                                      { AstName::Placeholder }



        rule identifier_start()
            = alpha() / "_"
        rule identifier_continue()
            = alnum() / "_" / "-"
        rule alnum()
            = alpha() / digit()
        rule alpha()
            = alpha_lower() / alpha_upper()
        rule alpha_lower()
            = ['a'..='z']
        rule alpha_upper()
            = ['A'..='Z']
        rule digit()
            = ['0'..='9']


        rule _()
            = (space() / comment())*
        rule space()
            = " " / "\n" / "\t" / "\r"
        rule comment()
            = "#" (!eol() [_])* eol()


        rule eol()
            = "\r\n" / "\n" / "\r"
        rule eof()
            = ![_]
    }
}



fn extract_name(n: AstName) -> String {
    match n {
        AstName::Name(v) => v,
        _                => unreachable!(),
    }
}

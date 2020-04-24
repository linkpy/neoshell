
use super::ast::*;


peg::parser!{
    pub grammar ns_parser() for str {


		rule pipe_input_description()
			= "|>" typename()
		rule pipe_output_description()
			= typename() "|>"



		// descriptor/command
		rule desc_command()
			= "'" _ identifier() _ desc_pipe_input()? _ desc_pipe_output()? _ desc_arg()* _ ";"

		// descriptor/command/pipe-input
		rule desc_pipe_input()
			= "|>" typename()

		// descriptor/command/pipe-output
		rule desc_pipe_output()
			= typename() "|>"

		// descriptor/command/argument
		rule desc_arg()
			= desc_arg_static()
			/ desc_mendatory_arg()
			/ desc_optional_arg()



		// descriptor/command/static-argument
		rule desc_arg_static()
			= "'" identifier() _
		// descriptor/command/mandatory-argument
		rule desc_mendatory_arg()
			= "<" _ desc_arg_without_def() _ ">" _
		// descriptor/command/optional-argument
		rule desc_optional_arg()
			= "[" _ desc_arg_with_def() ** semi_sep() _ "]" _



		rule desc_arg_with_def()
			= desc_arg_pos_with_def()
			/ desc_arg_pos_list_with_def()
			/ desc_arg_flag_with_def()
			/ desc_arg_opt_with_def()
			/ desc_arg_list_with_def()
			/ desc_arg_choice_with_def()
			/ desc_arg_nchoice_with_def()
			/ desc_arg_mchoice_with_def()
			/ desc_arg_nmchoice_with_def()

		rule desc_arg_without_def()
			= desc_arg_pos_without_def()
			/ desc_arg_pos_list_without_def()
			/ desc_arg_flag_without_def()
			/ desc_arg_opt_with_def()
			/ desc_arg_list_without_def()
			/ desc_arg_choice_without_def()
			/ desc_arg_nchoice_without_def()
			/ desc_arg_mchoice_without_def()
			/ desc_arg_nmchoice_without_def()



		// descriptor/argument/positional
		rule desc_arg_pos_with_def()
			= desc_arg_pos_without_def() _ some_value()
		rule desc_arg_pos_without_def()
			= identifier() _ typename()

		// descriptor/argument/positional-list
		rule desc_arg_pos_list_with_def()
			= desc_arg_pos_list_without_def() _ some_value()
		rule desc_arg_pos_list_without_def()
			= identifier() "..." _ typename()

		// descriptor/argument/flag
		rule desc_arg_flag_with_def()
			= desc_arg_flag_without_def() _ some_value() _ some_value()
		rule desc_arg_flag_without_def()
			= "/" identifier()

		// descriptor/argument/option
		rule desc_arg_opt_with_def()
			= desc_arg_opt_without_def() _ some_value()
		rule desc_arg_opt_without_def()
			= identifier() _ "=" _ typename()

		// descriptor/argument/list
		rule desc_arg_list_with_def()
			= desc_arg_list_without_def() _ some_value()
		rule desc_arg_list_without_def()
			= identifier() "[]" _ "=" _ typneame()

		// descriptor/argument/choice
		rule desc_arg_choice_with_def()
			= desc_arg_choice_without_def() _ some_value()
		rule desc_arg_choice_without_def()
			= identifier() _ "->" _ some_value() ** comma_sep()

		// descriptor/argument/named-choice
		rule desc_arg_nchoice_with_def()
			= desc_arg_nchoice_without_def() _ some_value()
		rule desc_arg_nchoice_without_def()
			= identifier() _ "->" _ some_name_pair_value() ** comma_sep()

		// descriptor/argument/multi-choice
		rule desc_arg_mchoice_with_def()
			= desc_arg_mchoice_without_def() _ some_value()
		rule desc_arg_mchoice_without_def()
			= identifier() _ "=>" _ some_value() ** comma_sep()

		// descriptor/argument/named-multi-choice
		rule desc_arg_nmchoice_with_def()
			= desc_arg_nmchoice_without_def() _ some_value()
		rule desc_arg_nmchoice_without_def()
			= identifier() _ "=>" _ some_name_pair_value() ** comma_sep()


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

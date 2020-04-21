#![deny(missing_docs)]

use crate::neoshell::parser::*;

use failure::*;


/// Switch usage trait. Defines how a switch behaves.
pub trait SwitchUsage<'a, T> {
	fn initial_value(&'a self) -> T;

	/// Collects the switch.
	fn collect(&'a mut self, name: &String, sw: &AstSwitch, v: T) -> Result<T, Error>;
}

/// Errors related to switch usages.
#[derive(Debug, Fail)]
pub enum SwitchUsageError {
	/// Invalid usage of a switch.
	#[fail(display = "Invalid usage for switch '{}' : {}", name, msg)]
	InvalidUsage{ name: String, msg: String }
}


/// Switch that enables something. Only supports the on switch : `+switch-name`.
pub struct EnablingSwitch<T> {
	initial: T,
	enabled: T,
}

/// Switch that disables something. Only supports the on switch : `-switch-name`.
pub struct DisablingSwitch<T> {
	initial: T,
	disabled: T,
}

/// Switch that can have three values. Only supports the on and off switches.
pub struct TernarySwitch<T> {
	initial: T,
	enabled: T,
	disabled: T,
}

pub struct ChoiceOptionSwitch<T> {
	initial: T,
	values: Vec<T>,
}

pub struct MultiChoiceOptionSwitch<T> {
	initial: T,
	values: Vec<T>,
}



impl SwitchUsageError {
	pub fn new_invalid_usage(n: String, m: &'static str) -> Error {
		Error::from(SwitchUsageError::InvalidUsage {
			name: n,
			msg: m.to_owned(),
		})
	}
}


impl<'a, T> SwitchUsage<'a, &'a T> for EnablingSwitch<T> {

	fn initial_value(&'a self) -> &'a T {
		return &self.initial;
	}

	fn collect(&'a mut self, name: &String, sw: &AstSwitch, _: &'a T) -> Result<&'a T, Error> {
		match sw {
			AstSwitch::On(_) => Ok(&self.enabled),
			_ => Err( SwitchUsageError::new_invalid_usage(name.clone(), "This switch only accepts on switch (+switch-name).") )
		}
	}

}

impl<'a, T> SwitchUsage<'a, &'a T> for DisablingSwitch<T> {

	fn initial_value(&'a self) -> &'a T {
		return &self.initial;
	}

	fn collect(&'a mut self, name: &String, sw: &AstSwitch, _: &'a T) -> Result<&'a T, Error> {
		match sw {
			AstSwitch::Off(_) => Ok(&self.disabled),
			_ => Err( SwitchUsageError::new_invalid_usage(name.clone(), "This switch only accepts off switchs (-switch-name).") )
		}
	}

}

impl<'a, T> SwitchUsage<'a, &'a T> for TernarySwitch<T> {

	fn initial_value(&'a self) -> &'a T {
		return &self.initial;
	}

	fn collect(&'a mut self, name: &String, sw: &AstSwitch, _: &'a T) -> Result<&'a T, Error> {
		match sw {
			AstSwitch::On(_) => Ok(&self.enabled),
			AstSwitch::Off(_) => Ok(&self.disabled),
			_ => Err( SwitchUsageError::new_invalid_usage(name.clone(), "This switch only accepts on or off switchs (+switch-name, -switch-name).") )
		}
	}

}

/*
Note about testing

I have spent wayyyyy too long trying to get this working by using Read
and Write traits to make it easily testable.

I have now given up - so the core methods are not tested.

Feel free to try get it working, I've realised I'm wasting my time and don't
actually care.

Good luck.
*/

use crate::prompt_config::PromptConfig;
use crate::prompt_error::PromptError;
use std::io::{stdin, stdout, Write};

/// Chomps new line from a string, returns the chomped part(s)
pub fn chomp(s: &mut String) -> String {
	let mut chomped: String = String::new();

	if let Some('\n') = s.chars().next_back() {
		s.pop();
		chomped += "\n";
	}
	if let Some('\r') = s.chars().next_back() {
		s.pop();
		chomped += "\r";
	}

	chomped
}

/// Structure containing all details of a prompt
pub struct Prompt {
	config: PromptConfig,
}

impl Prompt {
	pub fn new() -> Prompt {
		Prompt {
			config: PromptConfig::default(),
		}
	}

	// Helper constructors
	pub fn not_blank() -> Prompt {
		Prompt {
			config: PromptConfig::not_blank(),
		}
	}

	pub fn yn() -> Prompt {
		Prompt {
			config: PromptConfig::yn(),
		}
	}

	/// Set the default value
	pub fn default(mut self, default: &str) -> Prompt {
		self.config.default = Some(String::from(default));
		self
	}

	/// Set the choices
	pub fn choices(mut self, choices: Vec<&str>) -> Self {
		let mapped: Vec<String> = choices.iter().map(|&i| String::from(i)).collect();
		self.config.choices = Some(mapped);
		self
	}

	/// Set the validation for this prompt
	pub fn validate(
		mut self,
		func: impl Fn(String) -> Result<String, PromptError> + 'static,
	) -> Prompt {
		self.config.validator = Some(Box::new(func));
		self
	}

	/// Ask the question
	pub fn ask(&self, question: &str) -> Result<String, PromptError> {
		// If we have a custom validator then pass to the validate_loop()
		if let Some(validator) = &self.config.validator {
			return self.validate_loop(question, validator);
		}

		// If we have preset choices then call validate()
		if let Some(choices) = &self.config.choices {
			return self.validate_loop(question, |input: String| {
				let err_msg = format!("Valid options are: {}", choices.join(", "));
				match choices.iter().any(|i| i == &input) {
					true => Ok(input),
					false => Err(PromptError::ValidateError(err_msg)),
				}
			});
		}

		// Print question, take input
		print!("{}", self.format(question));
		let mut s: String = self.take_input()?;

		// Use default if we have one
		if s.is_empty() {
			if let Some(default) = &self.config.default {
				s = default.to_string();
			}
		}

		Ok(s)
	}

	/// Ask question and validate input
	/// Loop on validation failure
	fn validate_loop<F>(&self, question: &str, func: F) -> Result<String, PromptError>
	where
		F: Fn(String) -> Result<String, PromptError>,
	{
		let mut answer: Option<String> = None;
		let mut success = false;

		while !success {
			print!("{}", &self.format(question));
			let res = self.take_input()?;
			match func(res) {
				Ok(val) => {
					answer = Some(val);
					success = true;
				}
				Err(e) => {
					println!("{}", e)
				}
			}
		}

		match answer {
			Some(answer) => Ok(answer),
			// This should never be hit...
			None => Err(PromptError::InconcievableError()),
		}
	}

	/// Take input from STDIN
	fn take_input(&self) -> Result<String, PromptError> {
		let _ = stdout().flush();

		let mut s = String::new();
		match stdin().read_line(&mut s) {
			Ok(_) => chomp(&mut s),
			Err(e) => return Err(PromptError::InputError(e)),
		};

		Ok(s)
	}

	/// Format the question
	/// Displaying the default and options
	fn format(&self, question: &str) -> String {
		let mut our_question = String::from(question);
		let line_end = chomp(&mut our_question);
		our_question = String::from(our_question.trim());

		// If we have a default value then rejig the question to show that
		if let Some(choices) = &self.config.choices {
			our_question = format!("{} [{}]", our_question, choices.join(", "));
		} else if let Some(default) = &self.config.default {
			our_question = format!("{} [{}]", our_question, default);
		}

		format!("{}{} ", our_question, line_end)
	}
}

#[cfg(test)]
mod tests {
	use crate::Prompt;

	// Prompt::format
	#[test]
	fn format_plain() {
		let prompt = Prompt::new();
		assert_eq!(prompt.format("Dummy"), "Dummy ");
	}

	#[test]
	fn format_with_default() {
		let prompt = Prompt::new().default("def");
		assert_eq!(prompt.format("Dummy"), "Dummy [def] ");
	}

	#[test]
	fn format_with_choices() {
		let prompt = Prompt::new().choices(["a", "b", "c"].to_vec());
		assert_eq!(prompt.format("Dummy"), "Dummy [a, b, c] ");
	}

	#[test]
	fn format_with_choices_and_default() {
		let prompt = Prompt::new().choices(["a", "b", "c"].to_vec()).default("a");
		assert_eq!(prompt.format("Dummy"), "Dummy [a, b, c] ");
	}
}

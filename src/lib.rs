use std::collections::LinkedList;
use std::io::{stdin, stdout, Write};
use thiserror::Error;

// WORK IN PROGRESS
// TODO:
// * Add a PromptConfig struct (sets things like require non-blank, ignore case, etc...)
// * Add PromptList.all() method

/*
// Builder pattern:
let prompt_list = PromptList::new()
	.add(
		Prompt::new("What is your name?")
			.validate(|input: String| {
				if input.len() > 0 {
					return Ok(input);
				}
				return Err(PromptError::ValidateError(
					String::from("Cannot be blank")
				))
			})
	)
	.add(
		Prompt::new("Yes or no?")
			.choices([ "y", "Y", "n", "N" ].to_vec())
	)

let res = prompt_list.next().ask();

// Direct method
let res = Prompt::new("How is it going?").ask();
*/

// Error types for this module
#[derive(Error, Debug)]
pub enum PromptError {
	#[error("{0}")]
	ValidateError(String),

	#[error("Inconcievable! This should never happen!")]
	InconcievableError(),

	#[error(transparent)]
	InputError(#[from] std::io::Error),
}

/// Chomps new line from a string, returns the chomped part(s)
fn chomp(s: &mut String) -> String {
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
	question: String,
	choices: Option<Vec<String>>,
	default: Option<String>,
	validator: Option<Box<dyn Fn(String) -> Result<String, PromptError>>>,
}

impl Prompt {
	pub fn new(question: &str) -> Self {
		Prompt {
			question: String::from(question),
			choices: None,
			default: None,
			validator: None,
		}
	}

	/// Set the default value
	pub fn default(mut self, default: &str) -> Self {
		self.default = Some(String::from(default));
		self
	}

	/// Set the choices
	pub fn choices(mut self, choices: Vec<&str>) -> Self {
		let mapped: Vec<String> = choices.iter().map(|&i| String::from(i)).collect();
		self.choices = Some(mapped);
		self
	}

	/// Set the validation for this prompt
	pub fn validate(
		mut self,
		func: impl Fn(String) -> Result<String, PromptError> + 'static,
	) -> Self {
		self.validator = Some(Box::new(func));
		self
	}

	/// Ask the question
	pub fn ask(&self) -> Result<String, PromptError> {
		// If we have a custom validator then pass to the validate_loop()
		if let Some(validator) = &self.validator {
			return self.validate_loop(validator);
		}

		// If we have preset choices then call validate()
		if let Some(choices) = &self.choices {
			return self.validate_loop(|input: String| {
				let err_msg = format!("Valid options are: {}", choices.join(", "));
				match choices.iter().any(|i| i == &input) {
					true => Ok(input),
					false => Err(PromptError::ValidateError(err_msg)),
				}
			});
		}

		// Print question, take input
		print!("{}", &self.format());
		let mut s: String = self.take_input()?;

		// Use default if we have one
		if s.is_empty() {
			if let Some(default) = &self.default {
				s = default.to_string();
			}
		}

		Ok(s)
	}

	/// Ask question and validate input
	/// Loop on validation failure
	fn validate_loop<F>(&self, func: F) -> Result<String, PromptError>
	where
		F: Fn(String) -> Result<String, PromptError>,
	{
		let mut answer: Option<String> = None;
		let mut success = false;

		while !success {
			print!("{}", &self.format());
			let res = self.take_input()?;
			match func(res) {
				Ok(val) => {
					answer = Some(val);
					success = true;
				}
				Err(e) => println!("{}", e),
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
	fn format(&self) -> String {
		let mut our_question = String::from(&self.question);
		let line_end = chomp(&mut our_question);
		our_question = String::from(our_question.trim());

		// If we have a default value then rejig the question to show that
		if let Some(choices) = &self.choices {
			our_question = format!("{} [{}]", our_question, choices.join(", "));
		} else if let Some(default) = &self.default {
			our_question = format!("{} [{}]", our_question, default);
		}

		format!("{}{} ", our_question, line_end)
	}
}

/// Structure containing a queue of prompts
pub struct PromptList {
	prompts: LinkedList<Prompt>,
}

impl PromptList {
	pub fn new() -> PromptList {
		PromptList {
			prompts: LinkedList::new(),
		}
	}

	/// Adds a question to this prompt
	pub fn add(mut self, prompt: Prompt) -> Self {
		self.prompts.push_back(prompt);
		self
	}

	pub fn next(&mut self) -> Option<Prompt> {
		self.prompts.pop_front()
	}
}


#[cfg(test)]
mod tests {
	use crate::{Prompt, PromptList};

	// PromptList
	#[test]
	fn prompt_list_builder() {
		let prompt_list = PromptList::new()
			.add(Prompt::new("Dummy"))
			.add(Prompt::new("Dummy 2"));

		assert_eq!(prompt_list.prompts.len(), 2);
	}

	#[test]
	fn prompt_list_next() {
		let mut prompt_list = PromptList::new()
			.add(Prompt::new("Dummy"))
			.add(Prompt::new("Dummy 2"));

		assert!(prompt_list.next().is_some());
		assert!(prompt_list.next().is_some());
		assert!(prompt_list.next().is_none());
	}

	// Prompt::format
	#[test]
	fn format_plain() {
		let prompt = Prompt::new("Dummy");
		assert_eq!(prompt.format(), "Dummy ");
	}

	#[test]
	fn format_with_default() {
		let prompt = Prompt::new("Dummy").default("def");
		assert_eq!(prompt.format(), "Dummy [def] ");
	}

	#[test]
	fn format_with_choices() {
		let prompt = Prompt::new("Dummy").choices(["a", "b", "c"].to_vec());
		assert_eq!(prompt.format(), "Dummy [a, b, c] ");
	}

	#[test]
	fn format_with_choices_and_default() {
		let prompt = Prompt::new("Dummy").choices(["a", "b", "c"].to_vec()).default("a");
		assert_eq!(prompt.format(), "Dummy [a, b, c] ");
	}

	// Input
	// TODO!
}

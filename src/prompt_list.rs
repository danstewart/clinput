use crate::prompt::Prompt;
use std::collections::LinkedList;

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
}

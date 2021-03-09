use crate::prompt_error::PromptError;

pub struct PromptConfig {
	pub validator: Option<Box<dyn Fn(String) -> Result<String, PromptError>>>,
	pub choices: Option<Vec<String>>,
	pub default: Option<String>,
}

impl Default for PromptConfig {
	fn default() -> PromptConfig {
		PromptConfig {
			validator: None,
			choices: None,
			default: None,
		}
	}
}

// Some common use cases
impl PromptConfig {
	pub fn not_blank() -> PromptConfig {
		PromptConfig {
			choices: None,
			default: None,
			validator: Some(Box::new(|input: String| {
				if input.trim().is_empty() {
					return Err(PromptError::ValidateError(String::from("Cannot be blank")));
				}
				Ok(input)
			})),
		}
	}

	pub fn yn() -> PromptConfig {
		PromptConfig {
			choices: None,
			default: None,
			validator: Some(Box::new(|input: String| {
				if input.to_lowercase() != "n" && input.to_lowercase() != "y" {
					return Err(PromptError::ValidateError(String::from(
						"Valid options are: y, n",
					)));
				}
				Ok(input)
			})),
		}
	}
}

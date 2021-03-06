use crate::PromptError;

pub fn not_blank() -> impl Fn(String) -> Result<String, PromptError> {
	|input: String| {
		if input.is_empty() {
			return Err(PromptError::ValidateError(String::from("Cannot be blank")));
		}
		Ok(input)
	}
}

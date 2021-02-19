use thiserror::Error;

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

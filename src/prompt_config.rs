pub struct PromptConfig {
	pub no_loop: bool,
}

impl Default for PromptConfig {
	fn default() -> PromptConfig {
		PromptConfig { no_loop: false }
	}
}

WORK IN PROGRESS - DO NOT USE

```rust
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
```

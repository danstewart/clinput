# CLInput

A basic rust prompt crate.  


## How to use

#### Basic example
The most basic example is creating a new prompt and asking a question:  
```rust
// This will display "How is it going? " and wait for input via stdin
use clinput::{Prompt, PromptError};
let res: Result<String, PromptError> = Prompt::new().ask("How is it going?");
```

#### In depth
The `Prompt` struct also supports setting a default value, a list of valid choices and custom validator functions.  

```rust
use clinput::{Prompt, PromptError};

// This will display "What is your favourite colour? [red]" and wait for input via stdin
// If the input is empty then "red" will be used
let with_default = Prompt::new().default("red").ask("What is your favourite colour?");

// This will display "Choose a direction [up, down, left, right] " and wait for input via stdin
// If the input does not match one of the choices "Valid options are: up, down, left , right" will be 
// displayed and the prompt will loop
let from_list = Prompt::new().choices(["up", "down", "left", "right"]).ask("Choose a direction");

// This will display "Enter a password at least 8 chars in length "
// If the input is less than 8 chars it will display "Too short" and loop
let custom = Prompt::new()
	.validate(|input: String| {
		if input.length() < 8 {
			return Err(PromptError::ValidateError(String::from("Too short")));
		}
		Ok(input)
	})
	.ask("Enter a password at least 8 chars in length");

// These can all be combined
let from_list_with_default = Prompt::new()
	.default("Zues")
	.choices(["Zeus", "Aphrodite", "Hades", "Cronus", "Gaia", "Heracles", "Athena"].to_vec())
	.ask("Who is your favourite Greek god? ");
```

#### Common use case helpers
There are also some constructors to help with common use cases:  
```rust
use clinput::{Prompt, PromptError};

// This will display "What is your name? " and wait for input
// If the input is whitespace only it will display "Cannot be blank" and loop
let not_empty = Prompt::not_blank().ask("What is your name?");

// This will display "Continue? " and wait for input
// If the input is not in the list: y, n (case insensitive) it will display "Valid options are: y, n" and loop
let yes_or_no = Prompt::yn().ask("Continue?");
```

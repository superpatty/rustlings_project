
#Common Commands

- cargo new app
	- Creates a new rust project in the designated dir
- cargo build
	- Builds a debug version of the app in ./target/debug
- cargo build --release
	- Builds a release version of the app in ./target/release
- cargo run
	- Builds and runs the app
- cargo update
	- Updates any dependencies to the most recent non-BC version (0.8.3 --> 0.8.4, but NOT 0.8.3 --> 0.9.0)
- cargo check
	- Performs a syntax check on all the app code, but does not build the app

#Common Actions
- To add a dependency
	- add it to Cargo.toml in the dependencies section


#Fundamentals
- Variables default to immutable
	- To declare a variable as mutable
		- `let mut x = 5;`
- To explicitly cast a variable type
	- `let x: u32 = 23423;`
- Constants must have their type declared explicitly
	- Cannot be set via runtime, it but be a pre-defined value
	- ex: `const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;`
- Shadowing variables allows you to change the value of variable, but only in the specific scope
- Character type is denoted with single quotes (Unicode Scalar Value)
	- `let my_char = 'C';`
- String type is denoted with double quotes
	- `let my_sentence = "This is my sentence";`
- Compound Types
	- Tuple
		- Fixed length on creation
		- `let my_tup: (u32, f64, i32) = (34523, 82323.3382, -3432);`
		- `let (x, y, z) = my_tup;`
		- `println!("The value of y is: {}", y);`
		- 	OR
		- println!("The value of the second element is: {}", my_tup.1);
		- Empty Tuples is a `unit_type` and the value is a `unit_value`, expressed as `()`
	- Array
		- Fixed length on creation
		- All elements must be the same type
		- `let my_array = [1, 3, 10, 1];`
		- Vector is similar to an array, but have variable sizing
		- `let my_new_array: [u32, 4] = [1, 39, 9, 129834];`
		- Accessing array elements
		- `let my_second_element = my_new_array[0];`
- Functions
	- Defined with `fn` the function name, parentheses and curly brackets
		- `fn my_function() {}`
	- Parameter/argument in function call
		- Parameters must have type declaration
		- `fn my_function(x: i32) { println!("The value of x is: {}", x); }`
		- `fn print_labeled_measurement(value: i32, unit_label: char) { 
			println!("The measurement is {}{}", value, unit_label);
		}`
	- Expressions do not end in a semi-colon, if it does, it turns into a statement and does not return a value
	- ex: `let y = {
		let x = 3;
		x + 1
	};`
	- `y` evaluates to 4, if a semi-colon was added to the end of `x + 1`, `y` would not be 4
	- Return type of a function is declared with an ->
	- `fn five() -> i32 { 5 }`
	- 


//	Copyright 2013 Jeff Murray.

//	Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//	http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//	<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//	option. This file may not be copied, modified, or distributed
//	except according to those terms.

extern mod std;
extern mod core;
extern mod must;
use must::{Must};

// I moved the unit tests to must.rs but still use this to
//	generate and copy random keys when I need them

//	To compile: rustc must_test.rs -L .
//	to run on Linux: ./must_test

fn main() {

	io::println(Must::new_must().to_str());	
}

//	Copyright 2013 Jeff Murray.

//	Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//	http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//	<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//	option. This file may not be copied, modified, or distributed
//	except according to those terms.
 
 #[link(name = "lurker", vers = "1.0")];
 
//	rustc --lib lurker.rs -L .
//	rustc lurker.rs --test -o lurker-tests -L .
//	./lurker-tests

//  A b-tree has a leaf, and a lurker has a hole

trait Mound {

	fn find_hole(&self, val: char ) -> Result<Object, None>;
}

impl Mound for Object {

	fn find_hole(&self, val: char ) -> Result<Object, None>  {
		
		match self.paths.find( from_char( val ) ) {
			Some( obj ) => {
				obj
			}
			None => {
				None
			}
		}
	}
}

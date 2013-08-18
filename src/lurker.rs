//	Copyright 2013 Jeff Murray.

//	Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//	http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//	<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//	option. This file may not be copied, modified, or distributed
//	except according to those terms.
 
 #[link(name = "lurker", vers = "0.0")];
 
//	rustc --lib lurker.rs -L .
//	rustc lurker.rs --test -o lurker-tests -L .
//	./lurker-tests

//  A b-tree has a leaf, and a lurker has a hole in a colony.
//  http://en.wikipedia.org/wiki/File:Ant_Nest.jpg
//	The keys in this mound travel down a series of connected
//  holes, each representing the next value in the key, until
//  it becomes unique, where it stays.  This system is only 
//  efficient with highly random, alpha numeric keys, but, 
//	given that Must has them, this colony strategy, I hope, 
//	will offer a few advantages over over a b-tree for the 
//	master index.

//  The biggest benefit of the lurker approach is that balancing
//  usually will only involve moving a couple of keys to a couple
//  of new holes.  And deleting is just editing a hole, unless it is
//  the last one, and even then it is just one more edit.

//  Although this approach will likely require (I'm bad at math ;)) 
//	traversing 5 or 6 holes once a system gets a lot of keys, as
//	opposed to 3 or 4 leafs for a b-tree, I think that because of
//	the virtual elimination of node balancing, caching the first
//	two layers of holes will be trivial, making the lookups about
//	the same.  We will see.

//  most of the action is in fits/lurker_tunnel.rs, or will be when it is finished

extern mod std;
extern mod extra;
use extra::json::{ Object, Json };//,Null,ToJson
use std::hashmap::HashMap;
use std::str::from_char;

//struct Gallery;

trait Gallery {

	fn find_hole(&self, val: char ) -> Option<~Object>;
}

impl Gallery for ~HashMap<~str, Json> {

	fn find_hole(&self, val: char ) -> Option<~Object>  {
		
		match self.find( &from_char( val ) ) {
			Some( jsn ) => {
				match( copy *jsn ) {
					Object(  obj ) => {
						Some( copy obj )
					}
					_ => {
						None
					}
				}
			}
			None => {
				None
			}
		}
	}
}
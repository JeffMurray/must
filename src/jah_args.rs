//	Copyright 2013 Jeff Murray.

//	Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//	http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//	<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//	option. This file may not be copied, modified, or distributed
//	except according to those terms.
 
 #[link(name = "jah_args", vers = "1.0")];
 
//	rustc --lib jah_args.rs -L .
//	rustc jah_args.rs --test -o jah_args-tests -L .
//	./jah_args-tests

extern mod std;
extern mod core;
use std::json::{Object,String,Number,Json,List,Boolean, ToJson};//,Null,ToJson
use core::hashmap::linear::LinearMap;
use core::option::{ Some, None };


// See jah_spec.rs for a description of JahArgs
 
struct JahArgs {

	priv args: ~LinearMap<~str, Json>
}

enum GetArgErrorType {
	MissingKey,
	WrongDataType
}

impl JahArgs {

	pub fn new( args: ~LinearMap<~str, Json> ) -> JahArgs {
	
		JahArgs { args: args }
	}
	
	//	Gets the arg as a ~str

	pub fn get_str( &self, arg_name: ~str ) -> Result<~str, GetArgErrorType> {

		match self.get_json_val( arg_name ) {
			Some( value ) => {
				match value {
					String( s ) => {
						Ok( copy s )
					}
					_ => {
						Err( WrongDataType )
					}
				}
			}
			None => {
				Err( MissingKey )
			}
		}
	}
	
	//	Gets the arg as a map
	
	pub fn get_map( &self, arg_name: ~str ) -> Result<Object, GetArgErrorType> {
	
		match self.get_json_val( arg_name ) {
			Some( v ) => {
				match v {
					Object( o ) => {
						Ok( copy *o )
					}
					_ => {
						Err( WrongDataType )
					}
				}
			}
			None => {
				Err( MissingKey )
			}
		}
	}
	
	//	Gets the arg as a list
	
	pub fn get_list( &self, arg_name: ~str ) -> Result<List, GetArgErrorType> {
	
		match self.get_json_val( arg_name ) {
			Some( v ) => {
				match v {
					List( l ) => {
						Ok( copy l )
					}
					_ => {
						Err( WrongDataType )
					}
				}
			}
			None => {
				Err( MissingKey )
			}
		}
	}
	
	//	Gets the arg as a float
	
	pub fn get_float( &self, arg_name: ~str ) -> Result<float, GetArgErrorType> {
	
		match self.get_json_val( arg_name ) {
			Some( value ) => {
				match value {
					Number( num ) => {
						Ok( num )
					}
					_ => {
						Err( WrongDataType )
					}
				}
			}
			None => {
				Err( MissingKey )
			}
		}
	}
	
	//	Gets the arg as a bool
	
	pub fn get_bool( &self, arg_name: ~str ) -> Result<bool, GetArgErrorType> {
	
		match self.get_json_val( arg_name ) {
			Some( value ) => {
				match value {
					Boolean( b ) => {
						Ok( b )
					}
					_ => {
						Err( WrongDataType )
					}
				}
			}
			None => {
				Err( MissingKey )
			}
		}
	}
		
	pub fn has_arg( &self, key: &~str ) -> bool {
	
		self.args.contains_key(key) 
	}
	
	pub fn arg_count( &self) -> uint {
		
		self.args.len()
	}
	
	//	Gets the arg as a Json val
	
	pub fn get_json_val( &self, arg_name: ~str ) -> Option<Json> {
	
		match self.args.find( &arg_name ) {
			Some( val ) => {
				Some( copy *val )
			}
			None => {
				None
			}
		}
	}
	
	//	Returns a list of all the keys in the argument list
		
	pub fn arg_keys( &self ) -> ~[ ~str ] {
		
		let mut keys = ~[];
        for self.args.each |&( key, _ )| {
			keys.push( copy *key );
        }
        keys
	}
}

//	Implements to_str()

impl to_str::ToStr for JahArgs {

	fn to_str(&self) -> ~str {
	
		std::json::to_pretty_str(&(self.args.to_json()))		   	
	} 
}


#[test]
fn test_has_arg() {

	let mut map = ~LinearMap::new();
	map.insert( ~"test_arg", 1f.to_json() );
	let args = JahArgs::new( map );
	assert!( args.has_arg(&~"test_arg") && !args.has_arg(&~"missing_arg") );	
}

#[test]
fn test_arg_count() {

	let mut map = ~LinearMap::new();
	map.insert( ~"test_arg", 1f.to_json() );
	let args = JahArgs::new( map );
	assert!( args.arg_count() == 1u );	
}

#[test]
fn test_json_arg() {

	let mut map = ~LinearMap::new();
	map.insert( ~"test_arg", 1f.to_json() );
	let args = JahArgs::new( map );
	match args.get_json_val( ~"test_arg" ) {
		Some( _ ) => {
			assert!( true );
		}
		None => {
			fail!();
		}
	}
	
	match args.get_json_val( ~"test_missing_arg" ) {
		Some( _ ) => {
			fail!();
		}
		None => {
			assert!( true );
		}
	}
}

#[test]
fn test_arg_keys() {

	let mut map = ~LinearMap::new();
	map.insert( ~"test_arg_1", 1f.to_json() );
	map.insert( ~"test_arg_2", 2f.to_json() );
	let args = JahArgs::new( map );
	let arg_keys = args.arg_keys();
	assert!( arg_keys.len() == 2u );
	for arg_keys.each | key | {
		assert!( *key == ~"test_arg_1" || *key == ~"test_arg_2" );
	}	
}

#[test]
fn test_string_arg() {

	let mut map = ~LinearMap::new();
	let str = ~"hello world";
	map.insert( ~"test_str", str.to_json() );
	map.insert( ~"test_not_str", 100f.to_json() );
	let args = JahArgs::new( map );
	match args.get_str( ~"test_str" ) {
		Ok( val ) => {
			assert!( "hello world" == val );
		}
		Err( _ ) => {
			fail!();
		}
	}
	
	match args.get_str( ~"test_not_str" ) {
		Ok( _ ) => {
			fail!();
		}
		Err( err ) => {
			match err {
				WrongDataType => {
					assert!(true);
				}
				MissingKey => {
					fail!();
				}				
				
			}
		}
	}	
	match args.get_str( ~"test_not_there" ) {
		Ok( _ ) => {
			fail!();
		}
		Err( err ) => {
			match err {
				WrongDataType => {
					fail!();
				}
				MissingKey => {
					assert!(true);
				}				
			}
		}
	}	
}

#[test]
fn test_number_arg() {

	let mut map = ~LinearMap::new();
	let flt = 100f;
	map.insert( ~"test_float", flt.to_json() );
	map.insert( ~"test_not_float", true.to_json() );
	let args = JahArgs::new( map );
	match args.get_float( ~"test_float" ) {
		Ok( val ) => {
			assert!( 100f == val );
		}
		Err( _ ) => {
			fail!();
		}
	}
	match args.get_float( ~"test_not_float" ) {
		Ok( _ ) => {
			fail!();
		}
		Err( err ) => {
			match err {
				WrongDataType => {
					assert!(true);
				}
				MissingKey => {
					fail!();
				}				
				
			}
		}
	}	
	match args.get_float( ~"test_not_there" ) {
		Ok( _ ) => {
			fail!();
		}
		Err( err ) => {
			match err {
				WrongDataType => {
					fail!();
				}
				MissingKey => {
					assert!(true);
				}				
			}
		}
	}	
}

#[test]
fn test_object_arg() {

	let mut map = ~LinearMap::new();
	let mut obj = ~LinearMap::new();
	obj.insert( ~"test_key", true.to_json() );
	map.insert( ~"test_object", obj.to_json() );
	map.insert( ~"test_not_object", true.to_json() );
	let args = JahArgs::new( map );
	match args.get_map( ~"test_object" ) {
		Ok( val ) => {
			//make sure test_key is the same
			match JahArgs::new( copy ~val ).get_bool( ~"test_key" ) {
				Ok( bval ) => {
					assert!( bval );
				}
				Err( _ ) => {
					fail!();
				}
			}
		}
		Err( _ ) => {
			fail!();
		}
	}
	match args.get_map ( ~"test_not_object" ) {
		Ok( _ ) => {
			fail!();
		}
		Err( err ) => {
			match err {
				WrongDataType => {
					assert!(true);
				}
				MissingKey => {
					fail!();
				}				
				
			}
		}
	}	
	match args.get_map( ~"test_not_there" ) {
		Ok( _ ) => {
			fail!();
		}
		Err( err ) => {
			match err {
				WrongDataType => {
					fail!();
				}
				MissingKey => {
					assert!(true);
				}				
			}
		}
	}	
}

#[test]
fn test_boolean_arg() {

	let mut map = ~LinearMap::new();
	map.insert( ~"test_true", true.to_json() );
	map.insert( ~"test_false", false.to_json() );
	map.insert( ~"test_not_bool", 0f.to_json() );
	let args = JahArgs::new( map );
	match args.get_bool( ~"test_true" ) {
		Ok( val ) => {
			assert!( val );
		}
		Err( _ ) => {
			fail!();
		}
	}
	match args.get_bool( ~"test_false" ) {
		Ok( val ) => {
			assert!( !val );
		}
		Err( _ ) => {
			fail!();
		}
	}
	match args.get_bool( ~"test_not_bool" ) {
		Ok( _ ) => {
			fail!();
		}
		Err( err ) => {
			match err {
				WrongDataType => {
					assert!(true);
				}
				MissingKey => {
					fail!();
				}				
				
			}
		}
	}	
	match args.get_bool( ~"test_not_there" ) {
		Ok( _ ) => {
			fail!();
		}
		Err( err ) => {
			match err {
				WrongDataType => {
					fail!();
				}
				MissingKey => {
					assert!(true);
				}				
			}
		}
	}	
}

#[test]
fn test_list_arg() {

	let mut map = ~LinearMap::new();
	let mut list = ~[ 1f.to_json(), false.to_json() ];
	map.insert( ~"test_list", list.to_json() );
	map.insert( ~"test_not_list", true.to_json() );
	let args = JahArgs::new( map );
	match args.get_list( ~"test_list" ) {
		Ok( val ) => {
			match val[0] {
				Number( f ) => {
					assert!( f == 1f );
				}
				_ => {
					fail!();
				}
				
			}
		}
		Err( _ ) => {
			fail!();
		}
	}
	match args.get_list ( ~"test_not_list" ) {
		Ok( _ ) => {
			fail!();
		}
		Err( err ) => {
			match err {
				WrongDataType => {
					assert!(true);
				}
				MissingKey => {
					fail!();
				}				
				
			}
		}
	}	
	match args.get_list( ~"test_not_there" ) {
		Ok( _ ) => {
			fail!();
		}
		Err( err ) => {
			match err {
				WrongDataType => {
					fail!();
				}
				MissingKey => {
					assert!(true);
				}				
			}
		}
	}	
}
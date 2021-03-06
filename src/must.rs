//	Copyright 2013 Jeff Murray.

//	Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//	http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//	<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//	option. This file may not be copied, modified, or distributed
//	except according to those terms.

//	rustc --lib must.rs -L .
//	rustc must.rs --test -o must-tests -L .
//	./must-tests
//	See must_test.rs for another example of how to use a Must.

#[link(name = "must", vers = "0.0")];
extern mod std;
extern mod extra;
extern mod bootstrap;
extern mod jah_args;
use std::rand::{RngUtil, rng};
use std::to_str::{ToStr};
use extra::json::{ Json, Object, ToJson, String, Number };
use std::hashmap::HashMap;
use bootstrap::Bootstrap;
use jah_args::{ JahArgs, MissingKey, WrongDataType };
 
//	Must key goals:

//	-Be a unique, random, server assigned key that works well for drilling
//	through a b+ tree, and store a time stamp that differentiates versions
//	saved under the same key.
	
//	-Be quick and easy to work with, and powerful enough to 
//	confidentially assign would-wide unique keys to, in the words of Carl 
//	Sagan, "billions and billions" of documents.
	
//	-Be easy to convert to and from Json Objects

struct Must {

	//	See impl Must below for the meanings of these values

	key: ~str, 	
	sec: i64,
	nsec: i32
	
}

impl Must{

	//	Returns a new Must with a randomly-generated case-sensitive alphanumeric 
	//	key plus a clock time stamp adjusted to utc

	//	The time stamp data represents the Rust tm sec and nsec values at the time 
	//	of stamping, adjusted to utc

	//	This fn is used for making a key when adding adding a new document to
	//	the the file and b+ tree indexing system

	pub fn new() -> Must {
		let mut r = rng();
		Must::stamped( r.gen_str(16u))
	}
	
	//	Takes an existing key and returns a new Must with the same key but current 
	//	time stamp information.
	
	//	This stamp is used for distinguishing new versions of edited 
	//	documents since edited documents will keep the same alpha numeric 
	//	key as its ancestor and have a time stamp at the time of the saved 
	//	edit.  The document retains the same key, but new sec and nsec 
	//	values distinguish it from older versions in the b+ tree.
	
	pub fn stamped(must_key: ~str) -> Must {
	
		//	If the whole world uses utc for time stamps
		//	something good has to come ;)
		let t = extra::time::at_utc(extra::time::get_time()).to_timespec();
		Must { 
			key: copy must_key,
			sec: t.sec,
			nsec: t.nsec
		}
	}
	
	pub fn to_obj( &self ) -> ~Object {
		
		let mut obj = ~HashMap::new();
		obj.insert( ~"key", String( copy self.key ).to_json() );
		obj.insert( ~"sec", Number( self.sec.to_float() ).to_json() );
		obj.insert( ~"nsec", Number( self.nsec.to_float() ).to_json() );
		obj
	}
	
	//	Takes the must in json form and converts it to a Must
	
 	pub fn from_json( must: Json ) -> Result<Must, ~[~Object]> {
 	
 		match must {
 			Object( obj ) => {
 				Must::from_obj( &obj )
 			}
	 		_ => {
	 			Err( ~[Bootstrap::spec_rule_error(Bootstrap::arg_rule_key_arg_must_be_object(), ~"must", Bootstrap::spec_must_key(), ~"Q7wOoPiMNJ6xgqL1")] )
	 		}
 		}
	}
	
	//Takes an Object and converts it to a Must

	pub fn from_obj( obj: &~Object ) -> Result<Must, ~[~Object]> {
	
		let key = { match obj.get_str( ~"key" ) {
			Ok( key_val ) => { key_val }
			Err( err ) => {
				match err {
					MissingKey => {
						return Err( ~[ Bootstrap::spec_rule_error( Bootstrap::arg_rule_key_arg_must_exist(), ~"key",  Bootstrap::spec_must_key(), ~"ziVrdrQE6xeIjDLj" ) ] );
					}
					WrongDataType => {
						return Err( ~[ Bootstrap::spec_rule_error( Bootstrap::arg_rule_arg_must_be_string_key(), ~"key",  Bootstrap::spec_must_key(), ~"dtmIT0qQ2170Eohm" ) ] );
					}
				}	
			}
		}};
		let sec = { match obj.get_float( ~"sec" ) {
			Ok( sec_val ) => { sec_val }
			Err( err ) => {
				match err {
					MissingKey => {
						return Err( ~[ Bootstrap::spec_rule_error( Bootstrap::arg_rule_key_arg_must_exist(), ~"sec",  Bootstrap::spec_must_key(), ~"xSHex3PgO1VFrm5d" ) ] );
					}
					WrongDataType => {
						return Err( ~[ Bootstrap::spec_rule_error( Bootstrap::arg_rule_arg_key_arg_must_be_number(), ~"sec",  Bootstrap::spec_must_key(), ~"WDmdBc78ERtcJaZC" ) ] );
					}
				}	
			}
		}};
		let nsec = { match obj.get_float( ~"nsec" ) {
			Ok( nsec_val ) => { nsec_val }
			Err( err ) => {
				match err {
					MissingKey => {
						return Err( ~[ Bootstrap::spec_rule_error( Bootstrap::arg_rule_key_arg_must_exist(), ~"nsec",  Bootstrap::spec_must_key(), ~"2k5bXat4lBIhehIt" ) ] );
					}
					WrongDataType => {
						return Err( ~[ Bootstrap::spec_rule_error( Bootstrap::arg_rule_arg_key_arg_must_be_number(), ~"nsec",  Bootstrap::spec_must_key(), ~"nbON1bWP9U4NWFf7" ) ] );
					}
				}	
			}
		}};
		Ok( Must{ key: key, sec: sec.to_i64(), nsec: nsec.to_i32() } )
	}
}

impl Ord for Must {

	//	Less Than
	
	fn lt(&self, other: &Must) -> bool {
	
		if (self.key != other.key) {
			//	Key sorts ascending
			self.key < other.key
		} else{
			//	I am choosing to make the most current time be 
			//	the first on in the list of matching keys in the 
			//	b+ tree.  That is why the < operator is reversed 
			//	below
			if ( self.sec != other.sec ) {
				self.sec > other.sec
			} else {
				self.nsec > other.nsec
			}
		}
	}
		
	//	Less than or Equal
	
	fn le(&self, other: &Must) -> bool {
	
		!(*other).lt(&(*self)) 
	}
	
	//	Grater than or Equal

	fn ge(&self, other: &Must) -> bool { 
    	
		!(*self).lt(other) 
	}

	//	Grater Than

	fn gt(&self, other: &Must) -> bool { 
    	
		(*other).lt(&(*self))  	
	}
}

impl Eq for Must {

	//	EQual

	#[inline(always)]
	fn eq(&self, other: &Must) -> bool {

		eq_must(self, other) 
	}

	//	Not Equal
 
	#[inline(always)]
	fn ne(&self, other: &Must) -> bool {

		!eq_must(self, other) 
	}
}

//	Returns true if the key and time stamp is equal in
//	a and b

pub fn eq_must(a: &Must, b: &Must) -> bool{

	a.key == b.key && a.sec == b.sec && a.nsec == b.nsec
} 

impl ToStr for Must {

	//	Returns hyphen delimited string: key-sec-nsec
	fn to_str(&self) -> ~str {

		extra::json::to_pretty_str(&(self.to_json()))   	
	} 
}

impl ToJson for Must {

	//	Returns Json Object in the Must format
	fn to_json( &self ) -> Json {
	
		let mut must_spec = HashMap::new();
		must_spec.insert(~"key", self.key.to_json());
		must_spec.insert(~"sec", self.sec.to_json());
		must_spec.insert(~"nsec", self.nsec.to_json());
		Object(~must_spec)
	}
}

impl ToJson for @Must {

	//	Returns Json Object in the Must format
	fn to_json( &self ) -> Json {
    
		let mut must_spec = HashMap::new();
		must_spec.insert(~"key", self.key.to_json());
		must_spec.insert(~"sec", self.sec.to_json());
		must_spec.insert(~"nsec", self.nsec.to_json());
		Object(~must_spec)
	}
}

#[test]
fn test_Ord_Eq(){

	//	Make a new Must for adding a new document
	let must1 = Must::new();
	//	Make a new Must for a new version of an existing document
	let must2 = Must::stamped(copy must1.key);
	//	The newest documents should be less than the older one
	assert!( must2 < must1 &&  must2 <= must1 && must1 > must2 &&  must1 >= must2 );
	
	//	Make a new Must from existing information
	let must3 = copy must2;
	//	Make sure that a copied Must is equal to the Must that is copied.
	assert!(  must3 == must2 && must3 != must1 );
}

#[test]
fn test_to_str() {

	let must = Must::new();
	//	Make sure the key, sec and nsec values are always contained in to_str()
	assert!( must.to_str().contains( must.key ) && 
			must.to_str().contains( must.sec.to_str() ) &&   
			must.to_str().contains( must.nsec.to_str() ) 
		);
}

#[test]
fn test_from_json_and_to_json(){
	
	let m = Must::new();
	let j = m.to_json();
	match Must::from_json( j ) {
		Ok( val ) => {
			assert!( m == val );
		}
		Err( _ ) => {
			fail!();
		}
	}
}
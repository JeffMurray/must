/*	Copyright 2013 Jeff Murray.

	Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
	http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
	 <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
	option. This file may not be copied, modified, or distributed
	except according to those terms.
*/

#[link(name = "must", vers = "1.0")];
 extern mod std;
 extern mod core;
 use core::rand::RngUtil;
 use std::json ::{Json,Object,ToJson};
 use core::hashmap::linear::LinearMap;

/*	Must goals:

	Must be a unique random server assigned key that works will with b+ 
	tree to drill through it, and a time stamp that differentiates it 
	between saved versions of the same document.
	
	Must be quick and easy to work with, and powerful enough to 
	confidentially assign would-wide unique keys to billions of documents.
	
	Must be easy to convert to and from Json Objects
*/


/*	A must holds server issued key and revision information about a document.
*/
struct Must {

	/*	See impl Must below for the meanings of these values
	*/
	key: ~str, 	
	sec: i64,
	nsec: i32
	
}

impl Must{

	/*	Returns a new Must with a randomly-generated case-sensitive alphanumeric 
		key plus a clock time stamp adjusted to utc
	
		The time stamp data represents the sec and nsec values at the time 
		of stamping, adjusted to utc
	
		This fn is used for making a key when adding adding new document to
		the the file and b+ tree indexing system
	*/
	pub fn new_must() -> Must {
	
		Must::stamped_must_id(rand::Rng().gen_str(16u))
	}
	
	/*	Takes an existing key and returns a new Must with current time stamp
		information, adjusted to utc.
	
		This stamp is used for distingushing new versions of edited 
		documents since edited documents will keep the same alpha numeric 
		key as its ancestor and have a time stamp at the time of the saved 
		edit.  The document retains the same key, but new sec and nsec 
		values distinguish it from older versions in the b+ tree.
	*/
	pub fn stamped_must_id(must_key: ~str) -> Must {
	
		/*	If the whole world uses utc for time stamps
			something good has to come ;)
		*/		
		let t = std::time::at_utc(std::time::get_time()).to_timespec();
		
		Must { 
		
			key: copy must_key,
			sec: t.sec,
			nsec: t.nsec
		}
	}
	
	/*	Returns a new Must where all of the values are supplied
	*/	
	pub fn copied_must(must_key: ~str, sec: i64, nsec: i32) -> Must {
	
		Must { 
		
			key: copy must_key,
			sec: sec,
			nsec: nsec
		}
	}
}

impl Ord for Must {

	/*	less than
	*/
	fn lt(&self, other: &Must) -> bool {
	
		if (self.key != other.key) {
			
			//Key sorts ascending
			
			self.key < other.key
		}
		else{
		
			//I am choosing to make the most current time be the first on in the list
			//of matching keys in the b+ tree.  That is why the < operator is 
			//reversed below
			 
			 if ( self.sec != other.sec ) {
				
				self.sec > other.sec
			}
			else{
			
				self.nsec > other.nsec
			}
		}
	}
		
	/*	less than or equal
	*/
    fn le(&self, other: &Must) -> bool {
    
    	!(*other).lt(&(*self)) 
	}
	
	/*	grater than or equal
	*/	
    fn ge(&self, other: &Must) -> bool { 
    	
    	!(*self).lt(other) 
	}

	/*	grater than
	*/	
    fn gt(&self, other: &Must) -> bool { 
    	
    	(*other).lt(&(*self))  	
	}
}

impl Eq for Must {

	/*	equal
	*/
    #[inline(always)]
    fn eq(&self, other: &Must) -> bool {
    
        eq_must(self, other) 
    }

	/*	not equal
	*/    
    #[inline(always)]
    fn ne(&self, other: &Must) -> bool {
    
        !eq_must(self, other) 
    }
}

/*	Returns true if the key and time stamp is equal in
	a and b
*/
fn eq_must(a: &Must, b: &Must) -> bool{

	a.key == b.key && a.sec == b.sec && a.nsec == b.nsec
} 

impl to_str::ToStr for Must {

	/*	Returns hyphen delimited string: key-sec-nsec
	*/
    fn to_str(&self) -> ~str {
    
    	copy self.key + &"-"  + self.sec.to_str() + &"-" + self.nsec.to_str()   	
    } 
}

impl ToJson for Must {

	/*	Returns Json Object in the Must format
	*/
    fn to_json(&self) -> Json {
    
		let mut must_spec = LinearMap::new();

		must_spec.insert(~"key", self.key.to_json());
    	must_spec.insert(~"sec", self.sec.to_json());
    	must_spec.insert(~"nsec", self.nsec.to_json());
    	
    	Object(~must_spec)
	}
}

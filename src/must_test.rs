/*	Copyright 2013 Jeff Murray.

	Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
	http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
	<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
	option. This file may not be copied, modified, or distributed
	except according to those terms.
*/

extern mod std;
extern mod core;
mod must;
  
fn main() {

	io::println("testing must.rs");
	
	/*	I use the next line of code to print a new random Must
	*/
	io::println(must::Must::new_must().to_str());
	
	/*	Test the constructors and to_str function
	*/	
	test_to_str();
	
	/*	Test the comparitors and to_str function
	*/	
	test_Ord_Eq();	
}

fn test_Ord_Eq(){

	io::print("test_Ord_Eq: Order");
	
	/*	Make a new Must for adding a new document
	*/
	let must1 = must::Must::new_must();
	
	/*	Make a new Must for a new version of an existing document
	*/	
	let must2 = must::Must::stamped_must_id(copy must1.key);
		
	/*	The newest documents should be less than the older one
	*/
	if must2 < must1 &&  must2 <= must1 && must1 > must2 &&  must1 >= must2 {
	
		io::println("-passed");
	}
	else{
		
		io::println("-failed");
	}	
	
	io::print("test_Ord_Eq: Eq");	
	
	/*	Make a new Must from existing information
	*/	
	let must3 = must::Must::copied_must(copy must2.key, copy must2.sec, copy must2.nsec);
	
	/*	Make sure that a copied Must is equal to the Must that is copied.
	*/
	if must3 == must2 && must3 != must1 {
	
		io::println("-passed");
	}
	else{
		
		io::println("-failed");
	}		
}

fn test_to_str() {

	let must = must::Must::new_must();
	
	io::print("test_to_str");
	
	/*	Make sure the key, sec and nsec values are always contained in to_str()
	*/	
	if str::contains(must.to_str(), must.key) 
			&& str::contains(must.to_str(), must.sec.to_str())  
			&& str::contains(must.to_str(), must.nsec.to_str()) {
			
		io::println("-passed");
	}
	else{
	
		io::println("-failed");
	}
}
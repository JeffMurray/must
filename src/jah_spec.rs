//	Copyright 2013 Jeff Murray.

//	Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//	http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//	<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//	option. This file may not be copied, modified, or distributed
//	except according to those terms.

#[link(name = "jah_spec", vers = "1.0")];
 
//	rustc --lib jah_spec.rs -L .
//	rustc jah_spec.rs --test -o jah_spec-tests -L .
//	./jah_spec-tests

extern mod std;
extern mod extra;
extern mod jah_args;
extern mod bootstrap;
use extra::json::{ Object, Json, List, String, Number, Boolean, ToJson };
use std::hashmap::HashMap;
//use std::io::{println, print};
use jah_args::{ JahArgs, MissingKey, WrongDataType };
use bootstrap::{ Bootstrap };

//	jah is short for Json Argument Helper
 
// 	I am planning on spending a bit of time on jah_spec.rs because I want the Must document system 
// 	to have a "JSON-to-the-core" architecture.  CouchDB and Node.js both make compelling cases 
// 	that server side JavaScript engines and JSON can make powerful communication and real-time 
// 	index customization tools.

// 	By "JSON-to-the-core", I mean all communication in and out of the Must document system will
// 	be in Json values with documents not to exceed 1.4MB.  That means large documents can be stored
// 	by braking them up up into many documents who's keys are stored in a list in some reference 
//	document(s), or an index.
 	
//	I want to make argument specifications and error reporting user exposed through editable Json 
//	documents, so that others (hopefully) can maintain the documentation and error reporting in 
//	multiple languages and make tough decisions about high level Par and Fit argument specifications 
//	and level of specificity reported back to the user. 

// 	In addition to the obvious outward facing advantages for having a json interface, there are 
//	internal advantages as well. I plan to make a programmable argument relay (Par) that takes its
//	specifications in JahSpec documents.  Each Par...   par.rs, coming soon.   

// 	Through defining the argument specifications in the sunlight (user documents), and making jah_spec.rs 
//	to enforce the specifications at the relay level, the low level "functionally isolated transaction" 
//	(Fit) code can plug into the relay and trust that static specifications error reporting will be 
//	relayed correctly.


//	Holds keys to the specs that are expected, and what can be returned.

trait JahSpeced {
	fn spec_keys_in( &self ) -> ~[~str];
	fn spec_keys_out( &self ) -> ~[~str]; 
}

struct JahSpec {

	priv spec_args: ~JahArgs
}

struct ArgRules;

//	A JahSpec contains a JSON document (spec_map) that specifies what named arguments are 
//	allowed and any static checks that are to be enforced.

impl JahSpec {

	//	Create a new spec
	
	pub fn new( spec_args_: ~HashMap<~str,Json> ) -> ~JahSpec {
	
		~JahSpec{ spec_args: JahArgs::new(copy spec_args_) }
	}
	
	//	Returns the must key for argument spec in this Jah or returns
	//	a List of errors Identified by must key
	
	pub fn spec_key( &self ) -> ~str {
		
		match self.spec_args.get_str( ~"spec_key" ) {
			Ok( key_val ) => {
				key_val
			}
			Err( _ ) => {
				Bootstrap::spec_jah_spec_corrupt_key()
			}
		}
	}
	
	//	Checks that the supplied arguments are allowed, and that they conform to 
	//	conforms to the spec.  Also checks that any required args are present
	
	pub fn check_args( &self, args: ~JahArgs ) -> Result<bool, ~[~Object]> {
	
		match self.check_spec() {
			Ok( _ ) => {}
			Err( errs ) => {
				return Err( ~[ Bootstrap::reply_error_trace_info( ~"jah_spec.rs", ~"s0lFEONAYynSawUd" ) ] + errs ) 
			}
		}
		match self.get_allowed() {
    		Ok( alwd ) => {
				match self.check_rule_list( copy args, JahArgs::new( alwd ) ) {
					Ok(_) => {
						match self.check_required( copy args ) {
							Ok(_) => {
								Ok( true )
							}
							Err( errs ) => {
								Err( ~[ Bootstrap::reply_error_trace_info( ~"jah_spec.rs", ~"E4S4zOP8QKA6bm62" ) ]  + errs ) 
							}
						}
					}
					Err( errs ) => {
						Err( ~[ Bootstrap::reply_error_trace_info( ~"jah_spec.rs", ~"CvSaRhKZYrgqIl2q" ) ]  + errs ) 
					}
				}
    		}
    		Err( errs ) => {
    			Err( ~[ Bootstrap::reply_error_trace_info( ~"jah_spec.rs", ~"kmM0kE9Isb61If3j" ) ] + errs   ) 
			}
		}
	}
	
	priv fn errors_to_jah_args( errs: ~[~Object] ) -> ~[~JahArgs] {
	
		let mut args = ~[];
		for errs.iter().advance | err | {
			let map: ~HashMap<~str, Json> =  copy *err;
			args.push( JahArgs::new( map ) );
		}
		args
	}
	priv fn check_spec( &self ) -> Result<bool, ~[~Object]> {
		
		match self.spec_args.get_str( ~"spec_key" ) {
			Ok( _ ) => {}
			Err( err ) => {
				match err {
					MissingKey => {
						return Err( ~[ Bootstrap::spec_rule_error( Bootstrap::arg_rule_key_arg_must_exist(), ~"spec_key", Bootstrap::spec_jah_spec_key(), ~"a5q3rNiRtXeoO9Wj" ) ] )
					}
					WrongDataType => {
						return Err( ~[ Bootstrap::spec_rule_error( Bootstrap::arg_rule_arg_must_be_string_key(), ~"spec_key", Bootstrap::spec_jah_spec_key(), ~"EkhF0tz8VkQdmZL9" ) ] )
					}					
				}
			}
		}
		
		if self.spec_args.arg_count() != 2 {
			let keys = self.spec_args.arg_keys();
			for keys.iter().advance | key | {
				if !( key ==  &~"spec_key" || key == &~"allowed" ) {
					return Err( ~[ Bootstrap::spec_rule_error( Bootstrap::arg_rule_key_arg_is_not_allowed(), copy *key, Bootstrap::spec_jah_spec_key(), ~"yTI6O36SdlSKrlVV" ) ] )
				}
			}
		}
		Ok( true )
	}
	
	//	Loops through the allowed args and returns a list the arg names names 
	//	that are required
	
	pub fn get_required_keys(&self) -> Result<~[~str], ~[~Object]> {
	
		match self.get_allowed() {
    		Ok( alwd ) => {
    			let mut keys = ~[];
    			let ja = JahArgs::new( alwd );
    			let arg_keys = ja.arg_keys();
				for arg_keys.iter().advance | key | {
					match ja.get_list( copy *key ) {
						Ok( rule_list ) => {
							match JahSpec::list_has_rule( copy *rule_list, Bootstrap::arg_rule_key_arg_must_exist() ) {
								Ok( must_exist ) => {
									if must_exist {
										keys.push( copy *key );
									}
								}
								Err( errs ) => {
									return Err( errs + [ Bootstrap::reply_error_trace_info( ~"jah_spec.rs", ~"EsCOB18feSOPdIC0" ) ] ) 	
								}
							}
						},
						Err( err) => {
							match err {
								MissingKey => {
									//not possible because we are checking a confirmed list of parameters
								}
								WrongDataType => {
									return Err( ~[ Bootstrap::spec_rule_error( Bootstrap::arg_rule_key_arg_must_be_list(), copy *key, Bootstrap::spec_jah_spec_key(), ~"BWVlZZ1zI9hRBCaJ" ) ] )
								}
							}
						}						
					}
				}
				Ok( keys )		
    		}
    		Err( errs ) => {
    			Err( errs  + [ Bootstrap::reply_error_trace_info( ~"jah_spec.rs", ~"mwnGVUqBZIcqo0XX" ) ] )
			}
		}	
	}
	
	priv fn list_has_rule( rule_list: List, rule_key: ~str ) -> Result<bool, ~[~Object]> {
	
		for rule_list.iter().advance | rule | {
			match copy *rule {
				Object( rule_obj ) => {
					match ArgRules::get_rule_key( JahArgs::new( ~copy *rule_obj ) ) {
						Ok( this_rule_key ) => {
							if rule_key == this_rule_key {
								return Ok( true )
							}										
						},
						Err( errs ) => {
							return Err( errs  + [ Bootstrap::reply_error_trace_info( ~"jah_spec.rs", ~"CvSaRhKZYrgqIl2q" ) ] ) 		
						}
					}
				}
				_ => {
					return Err( ~[ Bootstrap::spec_rule_error( Bootstrap::arg_rule_key_arg_must_be_object(), copy rule_key, Bootstrap::spec_jah_spec_key(), ~"ALsKq4AAUB7E4hjR" ) ] )
				}
			}
		}
		Ok( false )		
	}
	
	// Finds all the allowed args that are also required makes sure they are in args
	
	priv fn check_required( &self, args: ~JahArgs ) -> Result<bool, ~[~Object]> {
		
		match self.get_required_keys() {
			Ok( req_args ) => {
				let mut errors = ~[];
				for req_args.iter().advance | req_key | {
					if !args.has_arg( req_key ) {
						errors.push( Bootstrap::spec_rule_error( Bootstrap::arg_rule_key_arg_must_exist(), copy *req_key, self.spec_key(), ~"InLa9WXyftFGkD0J" ) );
					}
				}
				if errors.len() == 0 {
					Ok( true )
				} else {
					Err( errors )
				}
			}
			Err (errs ) => {
				Err( errs )
			}
		}
	}
	
	//	Returns a the object with all the required argument keys
	
	priv fn get_allowed( &self ) -> Result<~Object, ~[~Object]> {
	
		match self.spec_args.get_map( ~"allowed" ) {
			Ok( obj ) => {
				Ok( copy obj )
	        }	
			Err( err ) =>  {
				match err {
					MissingKey => {
						Err( ~[ Bootstrap::spec_rule_error( Bootstrap::arg_rule_key_arg_must_exist(), ~"allowed", self.spec_key(), ~"4QRHmXsoKu1sWGVl" ) ] )
					}	
					WrongDataType => {
						Err( ~[ Bootstrap::spec_rule_error( Bootstrap::arg_rule_key_arg_must_be_list(), ~"allowed", self.spec_key(), ~"GfJyy8rO1hTvM0gs" ) ] )
					}
				}						
			}
		}
	}
	
	pub fn allowed_keys( &self ) -> Result<~[~str], ~[~Object]> {
		
		match self.get_allowed() {
			Ok( allowed ) => {
				let jah = JahArgs::new( allowed );
				Ok( jah.arg_keys() )
			}
			Err( errs ) => {
				Err( errs )
			}
		}
	}
	
	//	Loops through all of the rules for each allowed argument and check its rules in relation to its value
	
	priv fn check_rule_list( &self, args: ~JahArgs, allowed: ~JahArgs ) -> Result<bool, ~[~Object]> {
	
		let mut errors = ~[];
		let arg_keys = args.arg_keys();
		for arg_keys.iter().advance | key | {
	    	match allowed.get_list( copy *key ) {
				Ok( list ) => {
					match args.get_json_val( copy *key ) {
						Some(val) => { 
							let errs = JahSpec::check_rules( copy *key, copy *list, copy val, self.spec_key() );
							for errs.iter().advance | err_obj | {
								//we only get here if there are one or more
								//errors returned in the vector
								errors.push( copy *err_obj );
							}
						},
						None => {
							//it is silly to do something here because the key has been supplied from the read-only map we are checking.      											
						}
					}
				}
				Err( err ) => { 
					match err {
						MissingKey => {
							errors.push( Bootstrap::spec_rule_error(Bootstrap::arg_rule_key_arg_is_not_allowed(), copy *key,  self.spec_key(), ~"5uAMEPFBPxQVMTPB") )
						}
						WrongDataType => {
							errors.push( Bootstrap::spec_rule_error(Bootstrap::arg_rule_key_arg_must_be_list(), copy *key,  self.spec_key(), ~"XmPUcGmkS5wqXknu") )
						}
					}
				}
			}
		}
		if errors.len() > 0 {
			Err( errors )
		} else {
			Ok ( true )
		}
	}
	
	//	Loops through all assigned rules for a single arg
	//	and applyes them to the value
	priv fn check_rules(arg_key: ~str, rules: ~[ Json ], value: Json, spec_key: ~str ) -> ~[ ~Object ] {
	
		let mut errors: ~[ ~Object ] = ~[ ];
		for rules.iter().advance | rule | {
			let errs = JahSpec::check_rule(copy arg_key, copy *rule, copy value, copy spec_key );
			for errs.iter().advance | error | {
				errors.push( copy *error );
			}
		}
		errors
	}
		
	
	//	Checks that the rule is an Object, then sends it to do_rule
	//	along with the value to check
	
	priv fn check_rule( arg_key: ~str, rule: Json, value: Json, spec_key: ~str ) -> ~[~Object] {
	
		match rule {
			Object( ro ) => {
				match ArgRules::do_rule(arg_key, copy ro, value, copy spec_key ) {
					Ok(_) => { 
						~[] //arg_key passed, there is nothing to do
					}, 
					Err(err) => {
						~[ Bootstrap::reply_error_trace_info(~"jah_spec.rs", ~"Kerw1ihlUtNhYS5w") ] + err 
					}
				}
			}
			_ => {
				~[  Bootstrap::spec_rule_error( Bootstrap::arg_rule_key_arg_must_be_object(), copy arg_key, Bootstrap::spec_jah_spec_key(), ~"PhcnkCq9SzjjK3dc" )  ]
			}							
		}
	}
}

impl ArgRules {

	//	Checks a single rule against a single arg value returns OK
	//	if the value passes the check, otherwise Err with the errors
	
	pub fn do_rule( arg_key: ~str, rule: ~Object, val: Json, spec_key: ~str ) -> Result<bool,~[~Object]> {

		match ArgRules::get_rule_key( JahArgs::new( rule ) ) {
			Ok ( rule_key ) => {
				if rule_key == Bootstrap::arg_rule_key_arg_must_exist() {
					//arg_rule_required - this is checked in JahSpec.check_required
					Ok(true) 
				}
				else if rule_key == Bootstrap::arg_rule_arg_must_be_string_key() {
					ArgRules::arg_rule_must_be_string( copy arg_key, copy spec_key, val )
				}
				else if rule_key == Bootstrap::arg_rule_key_arg_must_be_object() {  
					ArgRules::arg_rule_must_be_object(copy arg_key, copy spec_key, val )
				} 
				else if rule_key == Bootstrap::arg_rule_arg_key_arg_must_be_number() {  
					ArgRules::arg_rule_must_be_number(copy arg_key, copy spec_key, val )
				}
				else if rule_key == Bootstrap::arg_rule_key_arg_must_be_list() {  
					ArgRules::arg_rule_must_be_list(copy arg_key, copy spec_key, val )
				} 
				else if rule_key == Bootstrap::arg_rule_key_arg_must_be_bool() {  
					ArgRules::arg_rule_must_be_bool(copy arg_key, copy spec_key, val )
				} else {
					Err( ~[ Bootstrap::spec_rule_error(Bootstrap::arg_rule_key_rule_must_be_implemented(), copy rule_key, spec_key, ~"OIbvlKzkmbwyYgBj") ] )
				}

			}
			Err( errs ) => {
				Err( errs + [ Bootstrap::reply_error_trace_info( ~"jah_spec.rs", ~"jFlfv8NZAe1UzYF4" ) ] )
			}
		}
	}
	//	Extracts the key that identifies the rule
	
	pub fn get_rule_key( rule: ~JahArgs ) -> Result<~str, ~[~Object]> {
	
		match rule.get_str( ~"rule_key" ) {
			Ok( rule_key ) => {
				Ok( rule_key )
			}
			Err( err ) => { 
				match err {
					MissingKey => {
						Err( ~[ Bootstrap::spec_rule_error(Bootstrap::arg_rule_key_arg_must_exist(), ~"rule_key",  Bootstrap::spec_rule_spec_key(), ~"spDzbsBn37HorqMZ") ] )
					}
					WrongDataType => {
						Err( ~[ Bootstrap::spec_rule_error(Bootstrap::arg_rule_arg_must_be_string_key(), ~"rule_key",  Bootstrap::spec_rule_spec_key(), ~"pxLJQ3RhHm5ls4YF") ] )
					}
				}
			}			
		} 	
	}
	
	//	Implements the rule that the argument must be an object
	
	fn arg_rule_must_be_object(arg_key: ~str, spec_key: ~str, val: Json) -> Result<bool,~[~Object]> {
		
		match val {
			Object(_) => {
				Ok( true ) 
			},
			_ => { 
				Err( ~[ Bootstrap::spec_rule_error(Bootstrap::arg_rule_key_arg_must_be_object(), copy arg_key, copy spec_key, ~"Qh6CpHMpGthIFStr") ] ) 
			}
		}
	}
	
	//	Implements the rule that the argument must be a number
	
	fn arg_rule_must_be_number(arg_key: ~str, spec_key: ~str, val: Json) -> Result<bool,~[~Object]> {
		
		match val {
			Number(_) => { 
				Ok( true ) 
			},
			_ => { 
				Err( ~[ Bootstrap::spec_rule_error(Bootstrap::arg_rule_arg_key_arg_must_be_number(), copy arg_key, copy spec_key, ~"ENt7F21pgLrisdgX") ] ) 
			}
		}
	}
	
	//	Implements the rule that the argument must be a string
	
	fn arg_rule_must_be_string(arg_key: ~str, spec_key: ~str, val: Json) -> Result<bool,~[~Object]> {
		
		match val {
			String(_) => { 
				Ok( true )
			},
			_ => { 
				Err( ~[ Bootstrap::spec_rule_error(Bootstrap::arg_rule_arg_must_be_string_key(), copy arg_key, copy spec_key, ~"bRRoirXHglnOzRza") ] ) 
			}
		}
	}
	
	//	Implements the rule that the argument must be a list
	
	fn arg_rule_must_be_list(arg_key: ~str, spec_key: ~str, val: Json) -> Result<bool,~[~Object]> {
		
		match val {
			List(_) => { 
				Ok( true )
			},
			_ => { 
				Err( ~[ Bootstrap::spec_rule_error(Bootstrap::arg_rule_key_arg_must_be_list(), copy arg_key, copy spec_key, ~"3quUP5sSihzrSikm") ] ) 
			}
		}
	}
	
	//	Implements the rule that the argument must be a boolean
	
	fn arg_rule_must_be_bool(arg_key: ~str, spec_key: ~str, val: Json) -> Result<bool,~[~Object]> {
		
		match val {
			Boolean(_) => { 
				Ok( true )
			},
			_ => { 
				Err( ~[ Bootstrap::spec_rule_error(Bootstrap::arg_rule_key_arg_must_be_bool(), copy arg_key, copy spec_key, ~"uBORM4dQ7ovO5own") ] ) 
			}
		}
	}
}

pub fn test_must_spec() {

	//
	let t = extra::time::at_utc( extra::time::get_time() ).to_timespec();
	let mut must_args = ~HashMap::new();
	let key = ~"bxHy5TuY3TsJzMdC";
	must_args.insert( ~"key", key.to_json() );
	must_args.insert( ~"sec", t.sec.to_json() );
	must_args.insert( ~"nsec", t.nsec.to_json() );	
	assert!( JahSpec::new( Bootstrap::spec_must() ).check_args( JahArgs::new( must_args ) ).is_ok() );
}

#[test]
pub fn test_missing_arg() {

	print( "test_missing arg" );
	let t = extra::time::at_utc( extra::time::get_time() ).to_timespec();
	let mut must_args = ~HashMap::new();
	let key = ~"bxHy5TuY3TsJzMdC";
	must_args.insert( ~"key", key.to_json() );
	must_args.insert( ~"sec", t.sec.to_json() );
		//param_rule_key_str_arg_must_exist()
	match JahSpec::new( Bootstrap::spec_must() ).check_args( JahArgs::new( must_args ) ) {
		Ok( _ ) => {
			fail!();
		}
		Err( errs ) => {
			for errs.iter().advance | err | {
				let jah = JahArgs::new( copy *err );
	 			match ArgRules::get_rule_key( jah ) {
	 				Ok( key ) => {
	 					if key == Bootstrap::arg_rule_key_arg_must_exist() {
	 						return;
	 					}
	 				}
	 				Err( _ ) => {}
				}
			}
			fail!();			
		}
	}
}

#[test]
pub fn test_extra_arg() {

	print( "test_extra_arg arg" );
	let t = extra::time::at_utc( extra::time::get_time() ).to_timespec();
	let mut must_args = ~HashMap::new();
	let key = ~"bxHy5TuY3TsJzMdC";
	must_args.insert( ~"key", key.to_json() );
	must_args.insert( ~"sec", t.sec.to_json() );
	must_args.insert( ~"nsec", t.nsec.to_json() );	
	must_args.insert( ~"extra_arg", t.nsec.to_json() );
	match JahSpec::new( Bootstrap::spec_must() ).check_args( JahArgs::new( must_args ) ) {
		Ok( _ ) => {
			fail!();
		}
		Err( errs ) => {
			for errs.iter().advance | err | {
				let jah = JahArgs::new( copy *err );
	 			match ArgRules::get_rule_key( jah ) {
	 				Ok( _ ) => {
	 					return;
	 				}
	 				Err( _ ) => {}
				}
			}
			fail!();			
		}	
	}
}

#[test]
pub fn test_number_rules() {

	//make a spec specifying the rules that apply to a number
	let mut allowed = ~HashMap::new();
 	allowed.insert(~"num_required_key",~[
 		Bootstrap::arg_rule_arg_must_exist().to_json(),
 		Bootstrap::arg_rule_num_must_be_number().to_json()
		]);
	allowed.insert(~"str_not_required_key",~[
 		Bootstrap::arg_rule_num_must_be_number().to_json()
		]);
		
 	let mut spec_map = ~HashMap::new();
 	
 	spec_map.insert( ~"spec_key", String( ~"Y79369vsP8sFfLss" ).to_json() );		
 	spec_map.insert( ~"allowed", allowed.to_json() ); 	
 	
 	//supply the minimum info to pass
 	let mut arg_map = ~HashMap::new();
 	arg_map.insert(~"num_required_key",1f.to_json());
 	let spec = JahSpec::new( spec_map );
 	assert!( spec.check_args( JahArgs::new( copy arg_map ) ).is_ok() );
 	
 	//make sure non-required args are still allowed
 	arg_map.insert( ~"str_not_required_key", 2f.to_json() );
 	assert!( spec.check_args( JahArgs::new( copy arg_map ) ).is_ok() );
 	
 	//make sure the required arg is required
 	arg_map.remove( &~"num_required_key" );
 	match spec.check_args( JahArgs::new( copy arg_map ) ) {
		Ok( _ ) => {
			fail!();
		}
		Err( errs ) => {
			let mut found = false;
			for errs.iter().advance | err | {
				let jah = JahArgs::new( copy *err );
	 			match ArgRules::get_rule_key( jah ) {
	 				Ok( key ) => {
	 					if key == Bootstrap::arg_rule_key_arg_must_exist() {
	 						found = true;
	 						break;
	 					}
	 				}
	 				Err( _ ) => {}
				}
			}
			if !found { fail!(); }		
		}
	}
	
	//check for wrong type
 	arg_map.remove( &~"str_not_required_key" );
 	arg_map.insert(~"str_not_required_key", true.to_json() );
 	match spec.check_args( JahArgs::new( copy arg_map ) ) {
		Ok( _ ) => {
			fail!();
		}
		Err( errs ) => {
			for errs.iter().advance | err | {
				let jah = JahArgs::new( copy *err );
	 			match ArgRules::get_rule_key( jah ) {
	 				Ok( key ) => {
	 					if key == Bootstrap::arg_rule_arg_key_arg_must_be_number() {
	 						return;
	 					}
	 				}
	 				Err( _ ) => {}
				}
			}
			fail!();			
		}
	}		
}	

#[test]
pub fn test_string_rules() {

	//make a fake spec specifying the rules that apply to a number
	let mut allowed = ~HashMap::new();
 	allowed.insert(~"str_required_key",~[
 		Bootstrap::arg_rule_arg_must_exist().to_json(),
 		Bootstrap::arg_rule_arg_must_be_string().to_json()
		]);
	allowed.insert(~"str_not_required_key",~[
 		Bootstrap::arg_rule_arg_must_be_string().to_json()
		]);
 	let mut spec_map = ~HashMap::new();
 	spec_map.insert( ~"spec_key", String( ~"Y79369vsP8sFfLss" ).to_json() );		
 	spec_map.insert( ~"allowed", allowed.to_json() ); 	
 	
 	//supply the minimum info to pass
 	let mut arg_map = ~HashMap::new();
 	arg_map.insert(~"str_required_key",String(~"test").to_json());
 	let spec = JahSpec::new( spec_map );
 	assert!( spec.check_args( JahArgs::new( copy arg_map ) ).is_ok() );
 	
 	//make sure non-required args are still allowed
 	arg_map.insert( ~"str_not_required_key", String(~"test").to_json() );
 	assert!( spec.check_args( JahArgs::new( copy arg_map ) ).is_ok() );
 	
 	//make sure the required arg is required and returns the right error
 	arg_map.remove( &~"str_required_key" );
 	match spec.check_args( JahArgs::new( copy arg_map ) ) {
		Ok( _ ) => {
			fail!();
		}
		Err( errs ) => {
			let mut found = false; 
			for errs.iter().advance | err | {
				let jah = JahArgs::new( copy *err );
	 			match ArgRules::get_rule_key( jah ) {
	 				Ok( key ) => {
	 					if key == Bootstrap::arg_rule_key_arg_must_exist() {
	 						found = true;
	 						break;
	 					}
	 				}
	 				Err( _ ) => {}
				}
			}
			if !found { fail!(); }		
		} 	
	}
	
	//check for wrong type and correct error
 	arg_map.remove( &~"str_not_required_key" );
 	arg_map.insert(~"str_not_required_key", true.to_json() );
 	match spec.check_args( JahArgs::new( copy arg_map ) ) {
		Ok( _ ) => {
			fail!();
		}
		Err( errs ) => {
			for errs.iter().advance | err | {
				let jah = JahArgs::new( copy *err );
	 			match ArgRules::get_rule_key( jah ) {
	 				Ok( key ) => {
	 					if key == Bootstrap::arg_rule_arg_must_be_string_key() {
	 						return;
	 					}
	 				}
	 				Err( _ ) => {}
				}
			}
			fail!();			
		} 	 	
	}		
}	

#[test]
pub fn test_list_rules() {

	//make a fake spec specifying the rules that apply to a number
	let mut allowed = ~HashMap::new();
 	allowed.insert(~"list_required_key",~[
 		Bootstrap::arg_rule_arg_must_exist().to_json(),
 		Bootstrap::arg_rule_arg_must_be_a_list().to_json()
		]);
	allowed.insert(~"list_not_required_key",~[
 		Bootstrap::arg_rule_arg_must_be_a_list().to_json()
		]);
 	let mut spec_map = ~HashMap::new();
 	spec_map.insert( ~"spec_key", String( ~"Y79369vsP8sFfLss" ).to_json() );		
 	spec_map.insert( ~"allowed", allowed.to_json() ); 	
 	
 	//supply the minimum info to pass
 	let mut arg_map = ~HashMap::new();
 	arg_map.insert( ~"list_required_key", List( ~[] ).to_json() );
 	let spec = JahSpec::new( spec_map );
 	assert!( spec.check_args( JahArgs::new( copy arg_map ) ).is_ok() );
 	
 	//make sure non-required args are still allowed
 	arg_map.insert( ~"list_not_required_key", List( ~[] ).to_json() );
 	assert!( spec.check_args( JahArgs::new( copy arg_map ) ).is_ok() );
 	
 	//make sure the required arg is required and returns the right error
 	arg_map.remove( &~"list_required_key" );
 	match spec.check_args( JahArgs::new( copy arg_map ) ) {
		Ok( _ ) => {
			fail!();
		}
		Err( errs ) => {
			let mut found = false;
			for errs.iter().advance | err | {
				let jah = JahArgs::new( copy *err );
	 			match ArgRules::get_rule_key( jah ) {
	 				Ok( key ) => {
	 					if key == Bootstrap::arg_rule_key_arg_must_exist() {
	 						found = true;
	 					}
	 				}
	 				Err( _ ) => {}
				}
			}
			if !found { fail!() };			
		} 	  	
	}
	
	//check for wrong type and correct error
 	arg_map.remove( &~"list_not_required_key" );
 	arg_map.insert(~"list_not_required_key", true.to_json() );
 	match spec.check_args( JahArgs::new( copy arg_map ) ) {
		Ok( _ ) => {
			fail!();
		}
		Err( errs ) => {
			for errs.iter().advance | err | {
				let jah = JahArgs::new( copy *err );
	 			match ArgRules::get_rule_key( jah ) {
	 				Ok( key ) => {
	 					if key == Bootstrap::arg_rule_key_arg_must_be_list() {
	 						return;
	 					}
	 				}
	 				Err( _ ) => {}
				}
			}
			fail!();			
		} 	  	
	}		
}

#[test]
pub fn test_object_rules() {

	//make a fake spec specifying the rules that apply to a number
	let mut allowed = ~HashMap::new();
 	allowed.insert(~"obj_required_key",~[
 		Bootstrap::arg_rule_arg_must_exist().to_json(),
 		Bootstrap::arg_rule_obj_must_be_object().to_json()
		]);
	allowed.insert(~"obj_not_required_key",~[
 		Bootstrap::arg_rule_obj_must_be_object().to_json()
		]);
 	let mut spec_map = ~HashMap::new();
 	spec_map.insert( ~"spec_key", String( ~"Y79369vsP8sFfLss" ).to_json() );		
 	spec_map.insert( ~"allowed", allowed.to_json() ); 	
 	
 	//supply the minimum info to pass
 	let mut arg_map = ~HashMap::new();
 	arg_map.insert( ~"obj_required_key", Object( ~HashMap::new() ).to_json() );
 	let spec = JahSpec::new( spec_map );
 	assert!( spec.check_args( JahArgs::new( copy arg_map ) ).is_ok() );
 	
 	//make sure non-required args are still allowed
 	arg_map.insert( ~"obj_not_required_key", Object( ~HashMap::new() ).to_json() );
 	assert!( spec.check_args( JahArgs::new( copy arg_map ) ).is_ok() );
 	
 	//make sure the required arg is required and returns the right error
 	arg_map.remove( &~"obj_required_key" );
 	match spec.check_args( JahArgs::new( copy arg_map ) ) {
		Ok( _ ) => {
			fail!();
		}
		Err( errs ) => {
			let mut found = false;
			for errs.iter().advance | err | {
				let jah = JahArgs::new( copy *err );
	 			match ArgRules::get_rule_key( jah ) {
	 				Ok( key ) => {
	 					if key == Bootstrap::arg_rule_key_arg_must_exist() {
	 						found = true;
	 					}
	 				}
	 				Err( _ ) => {}
				}
			}
			if !found { fail!(); }		
		} 	  	 	
	}
	
	//check for wrong type and correct error
 	arg_map.remove( &~"obj_not_required_key" );
 	arg_map.insert(~"obj_not_required_key", true.to_json() );
 	match spec.check_args( JahArgs::new( copy arg_map ) ) {
		Ok( _ ) => {
			fail!();
		}
		Err( errs ) => {
			for errs.iter().advance | err | {
				let jah = JahArgs::new( copy *err );
	 			match ArgRules::get_rule_key( jah ) {
	 				Ok( key ) => {
	 					if key == Bootstrap::arg_rule_key_arg_must_be_object() {
	 						return;
	 					}
	 				}
	 				Err( _ ) => {}
				}
			}
			fail!();			
		} 	
	}		
}

#[test]
pub fn test_bool_rules() {

	//make a fake spec specifying the rules that apply to a number
	let mut allowed = ~HashMap::new();
 	allowed.insert(~"bool_required_key",~[
 		Bootstrap::arg_rule_arg_must_exist().to_json(),
 		Bootstrap::arg_rule_arg_must_be_a_bool().to_json()
		]);
	allowed.insert(~"bool_not_required_key",~[
 		Bootstrap::arg_rule_arg_must_be_a_bool().to_json()
		]);
 	let mut spec_map = ~HashMap::new();
 	spec_map.insert( ~"spec_key", String( ~"Y79369vsP8sFfLss" ).to_json() );		
 	spec_map.insert( ~"allowed", allowed.to_json() ); 	
 	
 	//supply the minimum info to pass
 	let mut arg_map = ~HashMap::new();
 	arg_map.insert( ~"bool_required_key", true.to_json() );
 	let spec = JahSpec::new( spec_map );
 	
 	match spec.check_args( JahArgs::new( copy arg_map ) ) {
		Ok( _ ) => {}
		Err( errs ) => {
			for errs.iter().advance | err | {
				println( JahArgs::new( copy *err ).to_str() );
			}
			fail!();
		}
 	}
 	assert!( spec.check_args( JahArgs::new( copy arg_map ) ).is_ok() );
 	
 	//make sure non-required args are still allowed
 	arg_map.insert( ~"bool_not_required_key", false.to_json() );
 	assert!( spec.check_args( JahArgs::new( copy arg_map ) ).is_ok() );
 	
 	//make sure the required arg is required and returns the right error
 	arg_map.remove( &~"bool_required_key" );
 	match spec.check_args( JahArgs::new( copy arg_map ) ) {
		Ok( _ ) => {
			fail!();
		}
		Err( errs ) => {
			let mut found = false;
			for errs.iter().advance | err | {
				let jah = JahArgs::new( copy *err );
	 			match ArgRules::get_rule_key( jah ) {
	 				Ok( key ) => {
	 					if key == Bootstrap::arg_rule_key_arg_must_exist() {
	 						found = true;
	 						break;
	 					}
	 				}
	 				Err( _ ) => {}
				}
			}
			if !found { fail!(); }			
		}
	}
	
	//check for wrong type and correct error
 	arg_map.remove( &~"bool_not_required_key" );
 	arg_map.insert(~"bool_not_required_key", 1f.to_json() );
 	match spec.check_args( JahArgs::new( copy arg_map ) ) {
		Ok( _ ) => {
			fail!();
		}
		Err( errs ) => {
			for errs.iter().advance | err | {
				let jah = JahArgs::new( copy *err );
	 			match ArgRules::get_rule_key( jah ) {
	 				Ok( key ) => {
	 					if key == Bootstrap::arg_rule_key_arg_must_be_bool() {
	 						return;
	 					}
	 				}
	 				Err( _ ) => {}
				}
			}
			fail!();			
		}
	}
}

#[test]
pub fn zero_condition_spec() {

	let zero_spec = JahSpec::new( ~HashMap::new() );
	let zero_args = JahArgs::new( ~HashMap::new() );
	match zero_spec.check_args( zero_args ) {
		Ok( _ ) => {
			fail!();
		}
		Err( errs ) => {
			for errs.iter().advance | err | {
				let jah = JahArgs::new( copy *err );
	 			match ArgRules::get_rule_key( jah ) {
	 				Ok( key ) => {
	 					if key == Bootstrap::arg_rule_key_arg_must_exist() {
	 						return;
	 					}
	 				}
	 				Err( _ ) => {}
				}
			}
			fail!();			
		}
	}
}

#[test]
pub fn zero_condition_args() {

	let must_spec = JahSpec::new( Bootstrap::spec_must() );
	let zero_args = JahArgs::new( ~HashMap::new() );
	match must_spec.check_args( zero_args ) {
		Ok( _ ) => {
			fail!();
		}
		Err( errs ) => {
			for errs.iter().advance | err | {
				let jah = JahArgs::new( copy *err );
	 			match ArgRules::get_rule_key( jah ) {
	 				Ok( key ) => {
	 					if key == Bootstrap::arg_rule_key_arg_must_exist() {
	 						return;
	 					}
	 				}
	 				Err( _ ) => {}
				}
			}
			fail!();			
		}
	}
}
#[test]
pub fn zero_length_list_allowed_args() {

	//Hmm, the allowed list is empty, and the arg list is empty
	let mut map = ~HashMap::new();
	map.insert( ~"allowed", Object( ~HashMap::new() ) );
	map.insert( ~"spec_key", String( ~"KNexOJI1uttMf7qe" ) );
	let zero_list_allowed_spec = JahSpec::new( map );
	let zero_args = JahArgs::new( ~HashMap::new() );
	match zero_list_allowed_spec.check_args( zero_args ) {
		Ok( _ ) => {
			assert!( true ); //why not?
		}
		Err( _ ) => {
			fail!();		
		}
	}
	
	//The allowed list is empty and an arg is supplied?
	let mut map2 = ~HashMap::new();
	map2.insert( ~"little_ol_me", 1f.to_json() );
	let one_arg = JahArgs::new( map2 );
	match zero_list_allowed_spec.check_args( one_arg ) {
		Ok( _ ) => {
			fail!();
		}
		Err( errs ) => {
			for errs.iter().advance | err | {
				let jah = JahArgs::new( copy *err );
	 			match ArgRules::get_rule_key( jah ) {
	 				Ok( key ) => {
	 					if key == Bootstrap::arg_rule_key_arg_is_not_allowed() {
	 						return;
	 					}
	 				}
	 				Err( _ ) => {}
				}
			}
			fail!();			
		}
	}
}

#[test]
pub fn spec_missing_must_key() {

	let mut map = ~HashMap::new();
	map.insert( ~"allowed", List( ~[] ) );
	let missing_must_key_spec = JahSpec::new( map );
	let zero_args = JahArgs::new( ~HashMap::new() );
	match missing_must_key_spec.check_args( zero_args ) {
		Ok( _ ) => {
			fail!();
		}
		Err( errs ) => {
			for errs.iter().advance | err | {
				let jah = JahArgs::new( copy *err );
	 			match ArgRules::get_rule_key( jah ) {
	 				Ok( key ) => {
	 					if key == Bootstrap::arg_rule_key_arg_must_exist() {
	 						return;
	 					}
	 				}
	 				Err( _ ) => {}
				}
			}
			fail!();			
		}
	}
}


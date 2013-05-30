//	Copyright 2013 Jeff Murray.

//	Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//	http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//	<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//	option. This file may not be copied, modified, or distributed
//	except according to those terms.

//	rustc --lib bootstrap.rs -L .

#[link(name = "bootstrap", vers = "1.0")];
extern mod std;
extern mod core;
use std::json ::{Json,Object,ToJson,String};
use core::hashmap::linear::LinearMap;

//	Low Level JSON Specifications for JahSpecs, Rules and Errors that 
//	need to be used prior to the master b+tree being up and running **

//	For information about how JaSpecs and Rules are used, check out 
//	jah_spec.rs

//About Errors

//	I do not plan to report errors in any human language. Instead,
//	I will specify Must keys that will point to a Must document
//	that serves as the documentation source about the error.
//	When I get around to specifying the format for these error 
//	description documents, I will make sure to include multi-language
//	and user level sugar coating configurations.

struct Bootstrap;
impl Bootstrap {
 

	//	The jah_spec that JahSpec must conform to
	
	pub fn jah_spec_spec_key() -> ~str {
		
		~"VMldwIx01Q0ogNQb"
	}
	
	// returned if jah_spec cannot determine what the spec key is
	pub fn jah_corrupted_spec_key() -> ~str {
		
		~"MSzKdSSWEIP6WC5S"
	}	

 	pub fn jah_spec_spec() -> Object {
	 
	 	let mut spec = LinearMap::new();
	 	
	 	//	The must for the main source of information about the Jah specification
	 	
	 	spec.insert(~"spec_key",String(Bootstrap::jah_spec_spec_key()));
	 	let mut allowed = LinearMap::new();
	 		
	 	//	A list that specifies the rules to apply to the "allowed" argument.
	 	//	Every JahSpec has to have an arg called "allowed", which holds
	 	//	a list of rules
 			
	 	allowed.insert(~"allowed",~[
			//	The rule requiring this arg to be supplied in the specification
	 		Bootstrap::arg_rule_arg_must_exist().to_json(),
	 		//	The rule requiring this arg be an Object
	 		Bootstrap::arg_rule_obj_must_be_object().to_json()
 		]);
	 	spec.insert(~"allowed",	allowed.to_json());	 
	 	spec
	 }
	 
	 pub fn spec_must_must_key() -> ~str {
	 
	 	~"k0fA2inA45gmmZHV"
	 }
	 
	 pub fn spec_must() -> Object {
	 
	 	let mut allowed = LinearMap::new();
	 	//	A list that specifies the rules to apply to the "allowed" arg.
	 	//	Every JahSpec has to have an arg called "allowed", which holds
	 	//	a list of rules
 		allowed.insert(~"key",~[
	 		Bootstrap::arg_rule_arg_must_exist().to_json(),
	 		Bootstrap::arg_rule_arg_must_be_string().to_json()
 		]);
	 	allowed.insert(~"sec",~[
	 		Bootstrap::arg_rule_arg_must_exist().to_json(),
	 		Bootstrap::arg_rule_num_must_be_number().to_json()
 		]);
	 	allowed.insert(~"nsec",~[
	 		Bootstrap::arg_rule_arg_must_exist().to_json(),
	 		Bootstrap::arg_rule_num_must_be_number().to_json()
 		]);	
	 	let mut spec = LinearMap::new();
	 	//	The must for the main source of information about the Jah specification
	 	spec.insert( ~"spec_key", String( Bootstrap::spec_must_must_key() ).to_json() );		
	 	spec.insert( ~"allowed", allowed.to_json() ); 	
	 	spec
	 }
	 
	 
	 pub fn spec_rule_spec_key() -> ~str {
	 	
	 	~"g63UcB7rekfP4TlI"
	 }
	 
	 //	The jah_spec that spec_rules must conform to
	 pub fn spec_rule_spec() -> Object {
	 
	 	let mut allowed = LinearMap::new();
 		allowed.insert( ~"rule_key", ~[
	 		Bootstrap::arg_rule_arg_must_exist().to_json(),
	 		Bootstrap::arg_rule_arg_must_be_string().to_json()
 		]);
 		let mut spec = LinearMap::new();
	 	spec.insert( ~"spec_key", String( Bootstrap::spec_rule_spec_key() ).to_json() );
	 	spec.insert( ~"allowed", allowed.to_json() );
	 	spec
 	}
	 
	//	**Requests that can be sent before the document system is up and running **
	 
 	//	A request that the channel end communication
 	
	pub fn end_connection_key() -> ~str {
	 	
	 	~"mXnXQkmmB0GgltVM"
	}
	 
	pub fn end_connection() -> Object {
	 
	 	let mut order = LinearMap::new();
	 	order.insert( ~"spec_key", String( Bootstrap::end_connection_key() ).to_json() );
	 	order
	 }
	 
	 // ** Errors that can be generated before the document system is up and running **
	
	//	When an arg is sent that does not conform to the "allowed" arg list
	// 	this error document is reported.
	
	fn spec_rule_error_spec_key() -> ~str {
		
		~"gSNKN6Ey2JmDx70W"
	}
	
 	pub fn spec_rule_error(rule_key: ~str, arg_name: ~str, spec_key: ~str, line_must_key: ~str) -> Object { 
	 
		let mut rule = LinearMap::new();		
		//	The main source of information about rule document that reported on arg_name
		rule.insert( ~"rule_key", String( rule_key ).to_json() );
		rule.insert( ~"spec_key", spec_rule_error_spec_key().to_json
		//	The name of the supplied arg_name that is at issue
		rule.insert( ~"arg_name", String( arg_name ) );
		//	The main source of information about the specification that reported this error
		rule.insert( ~"err_spec_key", String( spec_key ).to_json() );
		//	The key that identifies the line of code that reported the error
		rule.insert( ~"line_must_key", String( line_must_key ).to_json() );
		rule
 	}
 	 		
 	//	When code identifies an error it adds information about the calling fn

	pub fn reply_error_trace_info(file_name: ~str, fn_name: ~str, line_must_key: ~str) -> Object { 
	 
		let mut rule = LinearMap::new();
		//	Trace info is another error type
		rule.insert( ~"err_must_key",String( ~"CDzmiOuZ8Vq7Ahuz" ).to_json() );
		//	The code file name containing the line of code referred to in the trace info
		rule.insert( ~"file_name", String( file_name ) );
		//	The function name that pertains to this trace info
		rule.insert( ~"fn_name", String( fn_name ) );
		//	A programmer assigns a unique, static, must to every line of code that creates this trace info
		rule.insert( ~"line_must_key", String( line_must_key ).to_json() );
		rule
	 }  
	
	 // ** Arg Rules **
 
	 pub fn arg_rule_key_arg_must_exist() -> ~str {
	 
	 	~"j60qzWM5fDYugKuh" 
	 }
	 
	 
	 pub fn arg_spec_key_not_known_to_par() -> ~str {
	 
	 	~"DZNl64Jyib2sQgde" 
	 }
	 
	 pub fn arg_spec_not_known_to_par() -> Object { 
	 
		let mut rule = LinearMap::new();
		rule.insert( ~"rule_key", Bootstrap::arg_spec_key_not_known_to_par().to_json() );
		rule
	 }
	 
	 pub fn arg_spec_key_not_known_to_fit() -> ~str {
	 
	 	~"DZNl64Jyib2sQgde" 
	 }
	 
	 //	This rule requires the arg to "exist" as opposed to only being "allowed"
	 pub fn arg_spec_not_known_to_fit() -> Object { 
	 
		let mut rule = LinearMap::new();
		rule.insert( ~"rule_key", Bootstrap::arg_spec_key_not_known_to_relay().to_json() );
		rule
	 }	 

	 //	This rule requires the arg to "exist" as opposed to only being "allowed"
	 pub fn arg_spec_arg_must_exist() -> Object { 
	 
		let mut rule = LinearMap::new();
		rule.insert( ~"spec_key", Bootstrap::arg_spec_key_arg_must_exist().to_json() );
		//	Note to self: These documents need further specification once the system is up and running
		rule
	 }
	 
	 pub fn arg_spec_key_arg_must_exist() -> ~str {
	 
	 	~"vTH21fYQkg6N6PBB" 
	 }
	 
	 //	This rule requires the arg to "exist" as opposed to only being "allowed"
	 pub fn arg_spec_arg_must_exist() -> Object { 
	 
		let mut rule = LinearMap::new();
		rule.insert( ~"spec_key", Bootstrap::arg_spec_key_arg_must_exist().to_json() );
		//	Note to self: These documents need further specification once the system is up and running
		rule
	 }
	 	  
	 //	This rule requires that the arg be a string and that the string match the 'value'
	 //	property

	 pub fn arg_rule_key_str_arg_must_equal() -> ~str {
	 
	 	~"OmQSjHS5lXaNO3fT" 
	 }
	 
	 pub fn arg_rule_str_arg_must_equal(value: ~str) -> Object { 
	 
		let mut rule = LinearMap::new();
		rule.insert( ~"rule_key", Bootstrap::arg_rule_key_str_arg_must_equal().to_json() );
		//	The string that the arg must be equal to
		rule.insert( ~"value", String( value ) );
		rule
	 }
	 
	 pub fn arg_rule_key_arg_key_must_be_string() -> ~str {
	 
	 	~"rWa4heRrNWkwabbB" 
	 }
	 
 	 pub fn arg_rule_arg_must_be_string() -> Object { 
	 
		let mut rule = LinearMap::new();
		rule.insert( ~"rule_key", Bootstrap::arg_rule_key_arg_key_must_be_string().to_json() );
		rule
	 }
 
 	 pub fn arg_rule_key_arg_must_be_object() -> ~str {
 	 
	 	~"VbXnEPrAXR7EFuqV" 
	 }
	 
 	 pub fn arg_rule_obj_must_be_object() -> Object { 
	 
		let mut rule = LinearMap::new();
		rule.insert( ~"rule_key", String( Bootstrap::arg_rule_key_arg_must_be_object() ).to_json() );
		rule
	 } 
	 
	 pub fn arg_rule_arg_key_arg_must_be_number() -> ~str {
	 
	 	~"SedD0iw7wRAqFNoT" 
	 }
	 
	 pub fn arg_rule_num_must_be_number() -> Object { 
	 
		let mut rule = LinearMap::new();
		rule.insert(~"rule_key", String( Bootstrap::arg_rule_arg_key_arg_must_be_number() ).to_json() ) ;
		rule
	 }	
	 	  
	 pub fn arg_rule_key_arg_is_not_allowed() -> ~str {
	 
	 	~"mHFrQhffSePzxCX8" 
	 }
	 
	 pub fn arg_rule_arg_key_is_not_allowed() -> Object { 
	 
		let mut rule = LinearMap::new();
		//	The must of the document pointing to the main source of information about trace info about arg_rule_num_must_be_allowed
		rule.insert(~"rule_key", String( Bootstrap::arg_rule_key_arg_is_not_allowed() ).to_json() ) ;
		rule
	 }	
	 
	 pub fn arg_rule_key_rule_must_be_implemented() -> ~str {
 		
 		~"jTA8LG31iLGuAo3e"
 	}
 	 
 	pub fn arg_rule_rule_must_be_implemented() -> Object { 
	 
		let mut rule = LinearMap::new();
		//	The must of the document pointing to the main source of information about trace info about arg_rule_num_must_be_allowed
		rule.insert(~"rule_key", String( Bootstrap::arg_rule_key_rule_must_be_implemented() ).to_json() ) ;
		rule
	}	
	 
 	pub fn arg_rule_key_arg_must_be_list() -> ~str {
 		
 		~"UbtWf9jG6pOCty32"
 	}
 	
 	pub fn arg_rule_arg_must_be_a_list() -> Object { 
	 
		let mut rule = LinearMap::new();
		//	The must of the document pointing to the main source of information about trace info about arg_rule_num_must_be_allowed
		rule.insert(~"rule_key", String( Bootstrap::arg_rule_key_arg_must_be_list() ).to_json() ) ;
		rule
	}	
	
 	pub fn arg_rule_key_arg_must_be_bool() -> ~str {
 		
 		~"QjupoyKaP6yp35um"
 	}
 	
 	pub fn arg_rule_arg_must_be_a_bool() -> Object { 
	 
		let mut rule = LinearMap::new();
		//	The must of the document pointing to the main source of information about trace info about arg_rule_num_must_be_allowed
		rule.insert(~"rule_key", String( Bootstrap::arg_rule_key_arg_must_be_bool() ).to_json() ) ;
		rule
	}	
}
//	Copyright 2013 Jeff Murray.

//	Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//	http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//	<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//	option. This file may not be copied, modified, or distributed
//	except according to those terms.

//	rustc --lib bootstrap.rs -L .

#[link(name = "bootstrap", vers = "0.0")];

extern mod std;
extern mod extra;
use extra::json ::{ Object, ToJson, String, List}; //, Json 
use std::hashmap::HashMap;

//  I'm leaving this with a long name like bootstrap to remind me how 
//	how badly I need to move these to a settings file :)
//	I'm considering naming the settings file BS, but maybe that is 
//	just the mood I am in.

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
 
	// looking forward to writing a lurker index
	// to retrieve these...for now I have: 
	// The master key of bootstrap specs
	pub fn find_spec( spec_key: ~str ) -> ~Object {
		
		match spec_key {
			~"VMldwIx01Q0ogNQb" => { Bootstrap::spec_jah_spec() } // see jah_spec.rs for a description of jah_specs
			~"MSzKdSSWEIP6WC5S" => { Bootstrap::spec_jah_spec_corrupt() } // return this when you would otherwise throw an error because of a bad spec 
			~"k0fA2inA45gmmZHV" => { Bootstrap::spec_must() } // holds the must key
			~"g63UcB7rekfP4TlI" => { Bootstrap::spec_rule_spec() }	
			~"gSNKN6Ey2JmDx70W" => { Bootstrap::spec_rule_error_spec() } //The spec for the for errors generated through enforcing specs.
			~"uJmQQbpKD9GrIAYl" => { Bootstrap::spec_fit_sys_err() }
			~"uHSQ7daYUXqUUPSo" => { Bootstrap::spec_add_doc() }
			~"CJeCZR6b9t6jj46S" => { Bootstrap::spec_edit_doc() }
			~"whORgvuF4eBf8vog" => { Bootstrap::spec_file_slice() }
			~"d6nLKNjnN05tJ2fl" => { Bootstrap::spec_find_slice_result() }
			~"cb2jMrSLSf72526W" => { Bootstrap::spec_append_slice() }
			~"TqXOr3DcqolwQavT" => { Bootstrap::spec_stored_doc() }
			~"H4rthuJ99hvwg8FZ" => { Bootstrap::fit_errs() }
			_ => { Bootstrap::spec_jah_spec_corrupt() }	
		}
	}
	
	// ************** Parts ****************
	pub fn file_append_slice_key() -> ~str { ~"S68yWotrIh06IdE8" }  //FileAppendSlice
	pub fn err_fit_key() -> ~str { ~"Zbh4OJ4uE1R1Kkfr" }  // ErrFit
	pub fn doc_slice_prep_key() -> ~str { ~"6Ssa58eFrC5Fpmys" }  // DocSlicePrep
	pub fn file_get_slice_key() -> ~str { ~"GwldCnkeG6FvjMiL" }  // FileGetSlice
	
	//	**Requests that can be sent before the document system is up and running **
	 	 
	 // ** Errors that can be generated before the document system is up and running **
	
	//	When an arg is sent that does not conform to the "allowed" arg list
	// 	this error document is reported.
		
	pub fn fit_sys_err( args: ~Object, sys_text: ~str, fit_key: ~str, file_name: ~str, line_key: ~str ) -> ~Object {
	
		let mut err = ~HashMap::new();		
		//	The main source of information about rule document that reported on arg_name
		err.insert( ~"sys_text", sys_text.to_json() );
		err.insert( ~"spec_key", Bootstrap::spec_fit_sys_err_key().to_json() );
		//	The name of the supplied arg_name that is at issue
		err.insert( ~"fit_key", fit_key.to_json() );
		//	The key that identifies the line of code that reported the error
		err.insert( ~"line_key", line_key.to_json() );
		err.insert( ~"file_name", file_name.to_json() );
		err.insert( ~"args", args.to_json() );
		err
	}
	
	// This reports a violation of some spec rule.  Note to future self: that is why it is not with the specs :)
 	pub fn spec_rule_error(rule_key: ~str, arg_name: ~str, spec_key: ~str, line_key: ~str) -> ~Object { 
	 
		let mut err = ~HashMap::new();		
		//	The main source of information about rule document that reported on arg_name
		err.insert( ~"rule_key", String( rule_key ).to_json() );
		err.insert( ~"spec_key", spec_key.to_json() );
		//	The name of the supplied arg_name that is at issue
		err.insert( ~"arg_name", String( arg_name ) );
		//	The main source of information about the specification that reported this error
		err.insert( ~"err_spec_key", String( spec_key ).to_json() );
		//	The key that identifies the line of code that reported the error
		err.insert( ~"line_key", String( line_key ).to_json() );
		err
 	}
 	
 	pub fn logic_error(rule_key: ~str, arg_name: ~str, line_key: ~str, file_name: ~str) -> ~Object { 
	 
		let mut err = ~HashMap::new();		
		//	The main source of information about rule document that reported on arg_name
		err.insert( ~"rule_key", String( rule_key ).to_json() );
		err.insert( ~"spec_key", String( ~"bEEA7c4Yp9Xl3pX1" ) );
		//	The name of the supplied arg_name that is at issue
		err.insert( ~"arg_name", String( arg_name ) );
		//	The key that identifies the line of code that reported the error
		err.insert( ~"line_key", String( line_key ).to_json() );
		//	The code file name containing the line of code referred to in the trace info
		err.insert( ~"file_name", String( file_name ) );
		err
 	} 	 		
 	
 	//	When code identifies an error it adds information about the calling fn
	pub fn reply_error_trace_info_key() -> ~str { 
	
		~"l79xpsPDlugK29zC"
	}
	
	pub fn reply_error_trace_info(file_name: ~str, line_key: ~str) -> ~Object { 
	 
		let mut err = ~HashMap::new();
		//	Trace info is another error type
		err.insert( ~"err_key",String( ~"CDzmiOuZ8Vq7Ahuz" ).to_json() );
		//	The code file name containing the line of code referred to in the trace info
		err.insert( ~"file_name", String( file_name ) );
		err.insert( ~"spec_key", String( Bootstrap::reply_error_trace_info_key() ) );
		//	A programmer assigns a unique, static, must key to every line of code that creates this trace info
		err.insert( ~"line_key", String( line_key ).to_json() );
		err
	 }  
	 
	// Fit Rule Keys
	pub fn slice_len_cannot_be_zero() -> ~str {
		
		~"ylmS0MTyl7abwJ5f"		
	}	
	
	pub fn named_attachment_is_missing() -> ~str {
		
		~"O2HbOCBd8z0yUZYh"		
	}	
		
	// Part Rule keys
	
	pub fn part_does_not_exist() -> ~str {
		
		~"3S5UX55Q84zKqx5o"		
	}
	
	// ** Spec Rule keys **
 
	pub fn arg_rule_key_arg_must_exist() -> ~str {
 
		~"j60qzWM5fDYugKuh" 
	}
	 
	//	This rule requires the arg to "exist" as opposed to only being "allowed"
	pub fn arg_rule_arg_must_exist() -> ~Object { 
	 
		let mut rule = ~HashMap::new();
		rule.insert( ~"rule_key", Bootstrap::arg_rule_key_arg_must_exist().to_json() );
		rule
	}
	 
	pub fn arg_spec_key_not_known_to_par() -> ~str {
	
	 
	 	~"DZNl64Jyib2sQgde" 
	}
	 
	pub fn arg_spec_not_known_to_par() -> ~Object { 
	 
		let mut rule = ~HashMap::new();
		rule.insert( ~"rule_key", Bootstrap::arg_spec_key_not_known_to_par().to_json() );
		rule
	}
	 
	pub fn arg_spec_key_not_known_to_fit() -> ~str {
	 
	 	~"DZNl64Jyib2sQgde" 
	}
	 
	//	thrown if the fit does not understand the arguments
	pub fn arg_spec_not_known_to_fit() -> ~Object { 
	 
		let mut rule = ~HashMap::new();
		rule.insert( ~"rule_key", Bootstrap::arg_spec_key_not_known_to_fit().to_json() );
		rule
	}	 
	 
	pub fn arg_spec_key_arg_must_exist() -> ~str {
	 
	 	~"vTH21fYQkg6N6PBB" 
	}
	 
	//	This rule requires the arg to "exist" as opposed to only being "allowed"
	pub fn arg_spec_arg_must_exist() -> ~Object { 
	 
		let mut rule = ~HashMap::new();
		rule.insert( ~"rule_key", Bootstrap::arg_spec_key_arg_must_exist().to_json() );
		rule
	}
	 	  
	//	This rule requires that the arg be a string and that the string match the 'value'
	//	property

	pub fn arg_rule_key_str_arg_must_equal() -> ~str {
	 
	 	~"OmQSjHS5lXaNO3fT" 
	}
	 
	pub fn arg_rule_str_arg_must_equal(value: ~str) -> ~Object { 
	 
		let mut rule = ~HashMap::new();
		rule.insert( ~"rule_key", Bootstrap::arg_rule_key_str_arg_must_equal().to_json() );
		//	The string that the arg must be equal to
		rule.insert( ~"value", String( value ) );
		rule
	}
	 
	pub fn arg_rule_arg_must_be_string_key() -> ~str {
	 
	 	~"rWa4heRrNWkwabbB" 
	}
	 
 	pub fn arg_rule_arg_must_be_string() -> ~Object { 
	 
		let mut rule = ~HashMap::new();
		rule.insert( ~"rule_key", Bootstrap::arg_rule_arg_must_be_string_key().to_json() );
		rule
	}
 
 	pub fn arg_rule_key_arg_must_be_object() -> ~str {
 	 
	 	~"VbXnEPrAXR7EFuqV" 
	}
	 
 	pub fn arg_rule_obj_must_be_object() -> ~Object { 
	 
		let mut rule = ~HashMap::new();
		rule.insert( ~"rule_key", String( Bootstrap::arg_rule_key_arg_must_be_object() ).to_json() );
		rule
	} 
	 
	pub fn arg_rule_arg_key_arg_must_be_number() -> ~str {
	 
	 	~"SedD0iw7wRAqFNoT" 
	}
	 
	pub fn arg_rule_num_must_be_number() -> ~Object { 
	 
		let mut rule = ~HashMap::new();
		rule.insert( ~"rule_key", String( Bootstrap::arg_rule_arg_key_arg_must_be_number() ).to_json() ) ;
		rule
	}	
	 	  
	pub fn arg_rule_key_arg_is_not_allowed() -> ~str {
	 
	 	~"mHFrQhffSePzxCX8" 
	}
	 
	pub fn arg_rule_arg_key_is_not_allowed() -> ~Object { 
	 
		let mut rule = ~HashMap::new();
		//	The must of the document pointing to the main source of information about trace info about arg_rule_num_must_be_allowed
		rule.insert( ~"rule_key", String( Bootstrap::arg_rule_key_arg_is_not_allowed() ).to_json() ) ;
		rule
	}	
	 
	pub fn arg_rule_key_rule_must_be_implemented() -> ~str {
 		
 		~"jTA8LG31iLGuAo3e"
 	}
 	 
 	pub fn arg_rule_rule_must_be_implemented() -> ~Object { 
	 
		let mut rule = ~HashMap::new();
		rule.insert( ~"rule_key", String( Bootstrap::arg_rule_key_rule_must_be_implemented() ).to_json() ) ;
		rule
	}	
	 
 	pub fn arg_rule_key_arg_must_be_list() -> ~str {
 		
 		~"UbtWf9jG6pOCty32"
 	}
 	
 	pub fn arg_rule_arg_must_be_a_list() -> ~Object { 
	 
		let mut rule = ~HashMap::new();
		rule.insert( ~"rule_key", String( Bootstrap::arg_rule_key_arg_must_be_list() ).to_json() ) ;
		rule
	}	
	
 	pub fn arg_rule_key_arg_must_be_bool() -> ~str {
 		
 		~"QjupoyKaP6yp35um"
 	}
 	
 	pub fn arg_rule_arg_must_be_a_bool() -> ~Object { 
	 
		let mut rule = ~HashMap::new();
		rule.insert( ~"rule_key", String( Bootstrap::arg_rule_key_arg_must_be_bool() ).to_json() ) ;
		rule
	}
		
	//  *** specs ***
	
	pub fn spec_fit_sys_err_key() -> ~str {
	
		~"uJmQQbpKD9GrIAYl"
	}
	
	//	The jah_spec that spec_rules must conform to
	
	pub fn spec_fit_sys_err() -> ~Object {
		 
	 	let mut allowed = ~HashMap::new();
		allowed.insert( ~"sys_text", ~[  // This text is reserved for native rust error messages
	 		Bootstrap::arg_rule_arg_must_exist().to_json(),
	 		Bootstrap::arg_rule_arg_must_be_string().to_json()
		]);
		allowed.insert( ~"spec_key", ~[
	 		Bootstrap::arg_rule_arg_must_exist().to_json(),
	 		Bootstrap::arg_rule_arg_must_be_string().to_json()
		]);
		allowed.insert( ~"fit_key", ~[
	 		Bootstrap::arg_rule_arg_must_exist().to_json(),
	 		Bootstrap::arg_rule_arg_must_be_string().to_json()
		]);
		allowed.insert( ~"line_key", ~[
	 		Bootstrap::arg_rule_arg_must_exist().to_json(),
	 		Bootstrap::arg_rule_arg_must_be_string().to_json()
		]);
		allowed.insert( ~"file_name", ~[
	 		Bootstrap::arg_rule_arg_must_exist().to_json(),
	 		Bootstrap::arg_rule_arg_must_be_string().to_json()
		]);	 
		allowed.insert(~"args",~[]); // can put other info here if helpful
		
		let mut spec = ~HashMap::new();
	 	spec.insert( ~"spec_key", String( Bootstrap::spec_fit_sys_err_key() ).to_json() );
	 	spec.insert( ~"allowed", allowed.to_json() );
	 	spec
	}	
	
	fn spec_hole_key() -> ~str {
		~"f0ALGyiyVzMmy3z8"
	}
	fn spec_next_hole_location_key() -> ~str {
		~"YvS6YKAm697XtRmG"
	}
		
	fn spec_doc_location_key() -> ~str {
		~"rcq0cffIOqyhQrcl"
	}	
	
	fn spec_next_hole_location() -> ~Object {
	
	 	let mut allowed = ~HashMap::new();
		allowed.insert( ~"next_hole", ~[  
	 		Bootstrap::arg_rule_arg_must_exist().to_json(),
	 		Bootstrap::arg_rule_obj_must_be_object().to_json()
		]);
		allowed.insert( ~"spec_key", ~[
	 		Bootstrap::arg_rule_arg_must_exist().to_json(),
	 		Bootstrap::arg_rule_arg_must_be_string().to_json()
		]);
		
		let mut spec = ~HashMap::new();
	 	spec.insert( ~"spec_key", String( Bootstrap::spec_next_hole_location_key() ).to_json() );
	 	spec.insert( ~"allowed", allowed.to_json() );
	 	spec		
	}

	fn spec_doc_location() -> ~Object {
	
	 	let mut allowed = ~HashMap::new();
		allowed.insert( ~"next_hole", ~[  
	 		Bootstrap::arg_rule_arg_must_exist().to_json(),
	 		Bootstrap::arg_rule_obj_must_be_object().to_json()
		]);
		allowed.insert( ~"spec_key", ~[
	 		Bootstrap::arg_rule_arg_must_exist().to_json(),
	 		Bootstrap::arg_rule_arg_must_be_string().to_json()
		]);
		
		let mut spec = ~HashMap::new();
	 	spec.insert( ~"spec_key", String( Bootstrap::spec_doc_location_key() ).to_json() );
	 	spec.insert( ~"allowed", allowed.to_json() );
	 	spec		
	}
		
	fn spec_hole_location() -> ~str {
		~"YvS6YKAm697XtRmG"
	}
		
	fn spec_rule_error_spec_key() -> ~str {
		
		~"gSNKN6Ey2JmDx70W"
	}
	
	fn spec_rule_error_spec() -> ~Object {

	 	let mut allowed = ~HashMap::new();
 		allowed.insert( ~"rule_key", ~[
	 		Bootstrap::arg_rule_arg_must_exist().to_json(),
	 		Bootstrap::arg_rule_arg_must_be_string().to_json()
 		]);
 		allowed.insert( ~"spec_key", ~[
	 		Bootstrap::arg_rule_arg_must_exist().to_json(),
	 		Bootstrap::arg_rule_arg_must_be_string().to_json()
 		]);
 		allowed.insert( ~"arg_name", ~[
	 		Bootstrap::arg_rule_arg_must_exist().to_json(),
	 		Bootstrap::arg_rule_arg_must_be_string().to_json()
 		]);
 		allowed.insert( ~"rule_key", ~[
	 		Bootstrap::arg_rule_arg_must_exist().to_json(),
	 		Bootstrap::arg_rule_arg_must_be_string().to_json()
 		]);
 		allowed.insert( ~"line_key", ~[
	 		Bootstrap::arg_rule_arg_must_exist().to_json(),
	 		Bootstrap::arg_rule_arg_must_be_string().to_json()
 		]);
 		allowed.insert( ~"file_name", ~[
	 		Bootstrap::arg_rule_arg_must_exist().to_json(),
	 		Bootstrap::arg_rule_arg_must_be_string().to_json()
 		]);	 		
 		let mut spec = ~HashMap::new();
	 	spec.insert( ~"spec_key", String( Bootstrap::spec_rule_error_spec_key() ).to_json() );
	 	spec.insert( ~"allowed", allowed.to_json() );
	 	spec

	}
	
	// returned if jah_spec cannot determine what the spec key is
	pub fn spec_jah_spec_corrupt_key() -> ~str {
		
		~"MSzKdSSWEIP6WC5S"
	}	

	pub fn spec_jah_spec_corrupt() -> ~Object {
	
	 	let mut spec = ~HashMap::new();
	 	spec.insert(~"spec_key",String(Bootstrap::spec_jah_spec_corrupt_key()));
	 	let mut allowed = ~HashMap::new();
 		allowed.insert( ~"spec_key", ~[
	 		Bootstrap::arg_rule_arg_must_exist().to_json(),
	 		Bootstrap::arg_rule_arg_must_be_string().to_json()
 		]);
	 	spec.insert(~"allowed", allowed.to_json() ); 
	 	spec		
	}
	
		//	The jah_spec that JahSpec must conform to
	
	pub fn spec_jah_spec_key() -> ~str {
		
		~"VMldwIx01Q0ogNQb" 
	}
		
 	pub fn spec_jah_spec() -> ~Object {
	 
	 	let mut spec = ~HashMap::new();
	 	//	The key for the main source of information about the Jah specification
	 	spec.insert(~"spec_key",String(Bootstrap::spec_jah_spec_key()));
	 	let mut allowed = ~HashMap::new();
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
	 
 	 pub fn spec_must_key() -> ~str {
	 
	 	~"k0fA2inA45gmmZHV"
	 }
	 
	 pub fn spec_must() -> ~Object {
	 
	 	let mut allowed = ~HashMap::new();
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
	 	let mut spec = ~HashMap::new();
	 	//	The must for the main source of information about the Jah specification
	 	spec.insert( ~"spec_key", String( Bootstrap::spec_must_key() ).to_json() );		
	 	spec.insert( ~"allowed", allowed.to_json() ); 	
	 	spec
	 }
	 
	 pub fn spec_rule_spec_key() -> ~str {
	 	
	 	~"g63UcB7rekfP4TlI"
	 }
	 
	 //	The jah_spec that spec_rules must conform to
	 pub fn spec_rule_spec() -> ~Object {
	 
	 	let mut allowed = ~HashMap::new();
 		allowed.insert( ~"rule_key", ~[
	 		Bootstrap::arg_rule_arg_must_exist().to_json(),
	 		Bootstrap::arg_rule_arg_must_be_string().to_json()
 		]);
 		let mut spec = ~HashMap::new();
	 	spec.insert( ~"spec_key", String( Bootstrap::spec_rule_spec_key() ).to_json() );
	 	spec.insert( ~"allowed", allowed.to_json() );
	 	spec
 	}	
 	
 	pub fn spec_add_doc_key() -> ~str {
 	
 		~"uHSQ7daYUXqUUPSo"
 	}
 	
 	pub fn spec_add_doc() -> ~Object {
 	
 		let mut allowed = ~HashMap::new();
		allowed.insert( ~"user", List( ~[Bootstrap::arg_rule_arg_must_be_string().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json() ] ) );
		allowed.insert( ~"acct", List( ~[Bootstrap::arg_rule_arg_must_be_string().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json()] ) );
		allowed.insert( ~"doc", List( ~[Bootstrap::arg_rule_obj_must_be_object().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json()] ) );
		allowed.insert( ~"spec_key", List( ~[Bootstrap::arg_rule_arg_must_be_string().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json()] ) );
		let mut spec = ~HashMap::new();
		spec.insert( ~"allowed", Object(allowed).to_json() );
		spec.insert( ~"spec_key", String( Bootstrap::spec_add_doc_key() ).to_json() );
		spec
 	}

 	pub fn spec_stored_doc_key() -> ~str {
 	
 		~"TqXOr3DcqolwQavT"
 	}
 	
 	pub fn spec_stored_doc() -> ~Object {
 	
 		let mut allowed = ~HashMap::new();
		allowed.insert( ~"user", List( ~[Bootstrap::arg_rule_arg_must_be_string().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json() ] ) );
		allowed.insert( ~"acct", List( ~[Bootstrap::arg_rule_arg_must_be_string().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json()] ) );
		allowed.insert( ~"doc", List( ~[Bootstrap::arg_rule_obj_must_be_object().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json()] ) );
		allowed.insert( ~"must", List( ~[Bootstrap::arg_rule_obj_must_be_object().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json()] ) );
		allowed.insert( ~"spec_key", List( ~[Bootstrap::arg_rule_arg_must_be_string().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json()] ) );
		let mut spec = ~HashMap::new();
		spec.insert( ~"allowed", Object(allowed).to_json() );
		spec.insert( ~"spec_key", String( Bootstrap::spec_add_doc_key() ).to_json() );
		spec
 	}
 	
 	pub fn spec_edit_doc_key() -> ~str {
 	
 		~"CJeCZR6b9t6jj46S"
 	}
 	
 	pub fn spec_edit_doc() -> ~Object {
 	
 		let mut allowed = ~HashMap::new();
		allowed.insert( ~"user", List( ~[Bootstrap::arg_rule_arg_must_be_string().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json() ] ) );
		allowed.insert( ~"acct", List( ~[Bootstrap::arg_rule_arg_must_be_string().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json()] ) );
		allowed.insert( ~"must", List( ~[Bootstrap::arg_rule_obj_must_be_object().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json()] ) );
		allowed.insert( ~"doc", List( ~[Bootstrap::arg_rule_obj_must_be_object().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json()] ) );
		allowed.insert( ~"spec_key", List( ~[Bootstrap::arg_rule_arg_must_be_string().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json()] ) );
		let mut spec = ~HashMap::new();
		spec.insert( ~"allowed", Object(allowed).to_json() );
		spec.insert( ~"spec_key", String( Bootstrap::spec_edit_doc_key() ).to_json() );
		spec
 	}

 	pub fn spec_file_append_result_key() -> ~str {
 	
 		~"MNoxG1jj5wAUJr2I"
 	}

 	pub fn spec_file_append_result() -> ~Object {
 	
 		let mut allowed = ~HashMap::new();
		allowed.insert( ~"slice", List( ~[Bootstrap::arg_rule_obj_must_be_object().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json() ] ) );
		allowed.insert( ~"extra", List( ~[Bootstrap::arg_rule_obj_must_be_object().to_json() ] ) );
		let mut spec = ~HashMap::new();
		spec.insert( ~"allowed", Object(allowed).to_json() );
		spec.insert( ~"spec_key", String(Bootstrap::spec_file_append_result_key()).to_json() );
		spec
 	}
	 	 	
 	pub fn spec_file_slice_key() -> ~str {
 	
 		~"whORgvuF4eBf8vog"
 	}

 	pub fn spec_file_slice() -> ~Object {
 	
 		let mut allowed = ~HashMap::new();
		allowed.insert( ~"pos", List( ~[Bootstrap::arg_rule_num_must_be_number().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json() ] ) );
		allowed.insert( ~"len", List( ~[Bootstrap::arg_rule_num_must_be_number().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json() ] ) );
		allowed.insert( ~"fn", List( ~[Bootstrap::arg_rule_num_must_be_number().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json() ] ) );		
		allowed.insert( ~"spec_key", List( ~[Bootstrap::arg_rule_arg_must_be_string().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json()] ) );
		let mut spec = ~HashMap::new();
		spec.insert( ~"allowed", Object(allowed).to_json() );
		spec.insert( ~"spec_key", String(Bootstrap::spec_file_slice_key()).to_json() );
		spec
 	}									
 	
 	pub fn spec_find_slice_result_key() -> ~str {
 	
 		~"d6nLKNjnN05tJ2fl"
 	}
 	
 	pub fn spec_find_slice_result() -> ~Object {
 	
 		let mut allowed = ~HashMap::new();
		allowed.insert( ~"attach", List( ~[Bootstrap::arg_rule_arg_must_be_string().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json() ] ) );
		allowed.insert( ~"extra", List( ~[Bootstrap::arg_rule_obj_must_be_object().to_json() ] ) );  // used to pass meaning and state between fits
		allowed.insert( ~"spec_key", List( ~[Bootstrap::arg_rule_arg_must_be_string().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json()] ) );
		let mut spec = ~HashMap::new();
		spec.insert( ~"allowed", Object(allowed).to_json() );
		spec.insert( ~"spec_key", String(Bootstrap::spec_find_slice_result_key()).to_json() );
		spec
 	}
 	
 	pub fn spec_append_slice_key() -> ~str {
	 	
	 	~"cb2jMrSLSf72526W"
	}
		
 	pub fn spec_append_slice() -> ~Object {
 	
 		let mut allowed = ~HashMap::new();
		allowed.insert( ~"attach", List( ~[Bootstrap::arg_rule_arg_must_be_string().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json() ] ) );
		allowed.insert( ~"spec_key", List( ~[Bootstrap::arg_rule_arg_must_be_string().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json() ] ) );
		allowed.insert( ~"extra", List( ~[Bootstrap::arg_rule_obj_must_be_object().to_json() ] ) );  // used to pass meaning and state between fits
		let mut spec = ~HashMap::new();
		spec.insert( ~"allowed", allowed.to_json() );
		spec.insert( ~"spec_key", String(Bootstrap::spec_append_slice_key()).to_json() );
		spec
	}
	
 	pub fn fit_errs_key() -> ~str {
	 	
	 	~"H4rthuJ99hvwg8FZ"
	}
		
 	pub fn fit_errs() -> ~Object {
 	
 		let mut allowed = ~HashMap::new();
		allowed.insert( ~"errs", List( ~[Bootstrap::arg_rule_arg_must_be_a_list().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json() ] ) );
		allowed.insert( ~"spec_key", List( ~[Bootstrap::arg_rule_arg_must_be_string().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json() ] ) );
		let mut spec = ~HashMap::new();
		spec.insert( ~"allowed", allowed.to_json() );
		spec.insert( ~"spec_key", String( Bootstrap::fit_errs_key() ).to_json() );
		spec
	}	
	
	// A gallery holds holds holds either a tunnel ore a chamber for each key that it holds.
 	pub fn lurker_gallery_spec_key() -> ~str {
	 	
	 	~"uf7heb1lYvudl8GV"
	}
		
 	pub fn lurker_gallery_spec() -> ~Object {
 	
 		let mut allowed = ~HashMap::new();
		allowed.insert( ~"keys", List( ~[Bootstrap::arg_rule_obj_must_be_object().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json() ] ) );
		allowed.insert( ~"spec_key", List( ~[Bootstrap::arg_rule_arg_must_be_string().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json() ] ) );
		let mut spec = ~HashMap::new();
		spec.insert( ~"allowed", allowed.to_json() );
		spec.insert( ~"spec_key", String( Bootstrap::lurker_gallery_spec_key() ).to_json() );
		spec
	}	
	
	// A tunnel leads to the next gallery
 	pub fn lurker_tunnel_spec_key() -> ~str {
	 	
	 	~"lh7BzwQUT7C5qkcK"
	}
		
 	pub fn lurker_tunnel_spec() -> ~Object {
 	
 		let mut allowed = ~HashMap::new();
		allowed.insert( ~"slice", List( ~[Bootstrap::arg_rule_obj_must_be_object().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json() ] ) );
		allowed.insert( ~"spec_key", List( ~[Bootstrap::arg_rule_arg_must_be_string().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json() ] ) );
		let mut spec = ~HashMap::new();
		spec.insert( ~"allowed", allowed.to_json() );
		spec.insert( ~"spec_key", String( Bootstrap::lurker_tunnel_spec_key() ).to_json() );
		spec
	}	
	
	
	// A chamber is where lurker keys live
 	pub fn lurker_chamber_spec_key() -> ~str {
	 	
	 	~"CUdcs4n0WDLkcLMj"
	}
		
 	pub fn lurker_chamber_spec() -> ~Object {
 	
 		let mut allowed = ~HashMap::new();
		allowed.insert( ~"last_key", List( ~[Bootstrap::arg_rule_obj_must_be_object().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json() ] ) );
		allowed.insert( ~"last_slice", List( ~[Bootstrap::arg_rule_obj_must_be_object().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json() ] ) );
		allowed.insert( ~"rev_count", List( ~[Bootstrap::arg_rule_num_must_be_number().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json() ] ) );
		allowed.insert( ~"rev_slice", List( ~[Bootstrap::arg_rule_obj_must_be_object().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json() ] ) );
		allowed.insert( ~"spec_key", List( ~[Bootstrap::arg_rule_arg_must_be_string().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json() ] ) );
		let mut spec = ~HashMap::new();
		spec.insert( ~"allowed", allowed.to_json() );
		spec.insert( ~"spec_key", String( Bootstrap::lurker_chamber_spec_key() ).to_json() );
		spec
	}
	// A chamber is where lurker keys live
 	pub fn lurker_revision_horde_spec_key() -> ~str {
	 	
	 	~"HgcjZSW4l1V7Cuu1"
	}
		
 	pub fn lurker_revision_horde_spec() -> ~Object {
 	
 		let mut allowed = ~HashMap::new();
		allowed.insert( ~"revs", List( ~[Bootstrap::arg_rule_arg_must_be_a_list().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json() ] ) );
		allowed.insert( ~"spec_key", List( ~[Bootstrap::arg_rule_arg_must_be_string().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json() ] ) );
		let mut spec = ~HashMap::new();
		spec.insert( ~"allowed", allowed.to_json() );
		spec.insert( ~"spec_key", String( Bootstrap::lurker_revision_horde_spec_key() ).to_json() );
		spec
	}
	
	pub fn lurk_store_key() -> ~str {
		
		~"PS15PUGe7Pjlt4Iq"
	}
	
 	pub fn lurk_store_key_spec() -> ~Object {
 	
 		let mut allowed = ~HashMap::new();
		allowed.insert( ~"key", List( ~[Bootstrap::arg_rule_obj_must_be_object().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json() ] ) );
		allowed.insert( ~"key_pos", List( ~[Bootstrap::arg_rule_num_must_be_number().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json() ] ) );
		allowed.insert( ~"doc_slice", List( ~[Bootstrap::arg_rule_obj_must_be_object().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json() ] ) );
		allowed.insert( ~"spec_key", List( ~[Bootstrap::arg_rule_arg_must_be_string().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json() ] ) );
		let mut spec = ~HashMap::new();
		spec.insert( ~"allowed", allowed.to_json() );
		spec.insert( ~"spec_key", String( Bootstrap::lurk_store_key() ).to_json() );
		spec
	}
	
	pub fn lurk_find_key() -> ~str {
		
		~"Y5RE8zBsk7SZjAZk"
	}
	
 	pub fn lurk_find_key_spec() -> ~Object {
 	
 		let mut allowed = ~HashMap::new();
		allowed.insert( ~"key", List( ~[Bootstrap::arg_rule_obj_must_be_object().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json() ] ) );
		allowed.insert( ~"key_pos", List( ~[Bootstrap::arg_rule_num_must_be_number().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json() ] ) );
		allowed.insert( ~"spec_key", List( ~[Bootstrap::arg_rule_arg_must_be_string().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json() ] ) );
		let mut spec = ~HashMap::new();
		spec.insert( ~"allowed", allowed.to_json() );
		spec.insert( ~"spec_key", String( Bootstrap::lurk_find_key() ).to_json() );
		spec
	}
}

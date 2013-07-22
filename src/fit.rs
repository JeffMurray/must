//	Copyright 2013 Jeff Murray.

//	Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//	http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//	<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//	option. This file may not be copied, modified, or distributed
//	except according to those terms.
 
 #[link(name = "fit", vers = "0.0")];
 
//	rustc --lib fit.rs -L .
//	rustc fit.rs --test -o fit-tests -L .
//	./fit-tests

extern mod std;
extern mod extra;
extern mod bootstrap;
use extra::json::{ Object, ToJson, String, List };
use std::comm::{ ChanOne };
use std::to_str::ToStr;
use std::hashmap::HashMap;
use bootstrap::{ Bootstrap };

//	Functionally Isolated Task (Fit)
// See Par.rs for more info about what a Fit is

//  what gets passed around from fit to fit
struct FitArgs {
	doc: ~Object,
	attach: ~[u8], //  Args can only contain 0 or 1 attachment, for multiple attachments use multiple calls
	state: ~Object //  A way for a Fit to get state information back with a result
}

impl FitArgs {

	pub fn from_doc( doc: ~Object ) -> FitArgs {
	
		FitArgs{ doc: doc, attach: ~[], state: ~HashMap::new() }
	}	
	
	pub fn from_doc_with_attach( doc: ~Object, attach: ~[u8] ) -> FitArgs {
	
		FitArgs{ doc: doc, attach: attach, state: ~HashMap::new() }
	}	
}

struct FitErrs {
	errs: ~[~Object]
}

enum ParFitComm { 
	// take some args, do some work, send the results home
	DoFit( ~FitArgs, ChanOne<FitComm> ), // ( args, home_chan ) 
	ParFitCommEndChan
}

enum FitComm { // designed to be used in a oneshot
	FitOk( ~FitArgs ), // (  args )
	FitErr( ~FitErrs ), // (  errors )
	FitSysErr( ~FitErrs ) // ( errors ) resource fail message from Rust that breaks this fit
}

trait Parfitable {
	pub fn new( config: ~Object ) -> ~Self;
	pub fn connect( &self ) -> Result<Chan<ParFitComm>, ~FitErrs> ;
	pub fn fit_key( &self ) -> ~str;
}

impl FitErrs {

	pub fn from_object( err: ~Object ) -> ~FitErrs {
	
		~FitErrs { errs: ~[err] }	
	}
	
	pub fn from_objects( errs: ~[~Object] ) -> ~FitErrs {
	
		~FitErrs { errs: errs }	
	}
	
	pub fn prepend_errs( &self,  errs: ~[~Object] ) -> ~FitErrs {

		~FitErrs { errs: errs + self.errs }
	}

	pub fn prepend_err( &self,  err: ~Object ) -> ~FitErrs {

		~FitErrs { errs: ~[err] + self.errs }
	}
	
	pub fn to_args( &self )-> ~Object {
		
		let mut err = ~HashMap::new();		
		//	The main source of information about rule document that reported on arg_name
		err.insert( ~"spec_key", String( Bootstrap::fit_errs_key() ).to_json() );
		let mut errs = ~[];
		for self.errs.iter().advance | err | {
			errs.push( err.to_json() );
		}
		err.insert( ~"errs", List(errs).to_json() );
		err
	}
}

impl ToStr for FitErrs {
	
	fn to_str( &self ) -> ~str {
		
		let mut pretty = ~"";
		for self.errs.iter().advance | err | {
			pretty =  pretty + extra::json::to_pretty_str(&((*err).to_json()));
		}
		pretty
	}
}

//	Copyright 2013 Jeff Murray.

//	Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//	http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//	<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//	option. This file may not be copied, modified, or distributed
//	except according to those terms.
 
 #[link(name = "fit", vers = "1.0")];
 
//	rustc --lib fit.rs -L .
//	rustc fit.rs --test -o fit-tests -L .
//	./fit-tests

extern mod std;
extern mod extra;
use extra::json::{ Object, ToJson };
use std::comm::{ ChanOne };
use std::to_str::ToStr;

//	Functionally Isolated Task (Fit)
// See Par.rs for more info about what a Fit is

//  what gets passed around from fit to fit
struct FitArgs {
	doc: ~Object,
	attach: ~[u8] //  Args can only contain 0 or 1 attachment, for multiple attachments use multiple calls
}

impl FitArgs {

	pub fn from_doc( doc: ~Object ) -> FitArgs {
	
		FitArgs{ doc: doc, attach: ~[] }
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

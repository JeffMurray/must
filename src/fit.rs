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
use extra::json::{ Object };
use std::comm::{ ChanOne };

//	Functionally Isolated Task (Fit)
// See Par.rs for more info about what a Fit is

enum ParFitComm { 
	// take some args, do some work, send the results home
	DoFit( ~Object, ChanOne<FitComm> ), // ( args, home_chan ) 
	ParFitCommEndChan
}

enum FitComm { // designed to be used in a oneshot
	FitOk( ~Object ), // (  args )
	FitErr( ~Object ), // (  errors )
	FitSysErr( ~Object ) // ( errors ) resource fail message from Rust that breaks this fit
}

trait Parfitable {
	pub fn new( config: ~Object ) -> ~Self;
	pub fn connect( &self ) -> Result<Chan<ParFitComm>, ~Object> ;
	pub fn fit_key( &self ) -> ~str;
}

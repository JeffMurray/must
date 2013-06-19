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
extern mod core;
use std::json::{ Object };
use core::owned::{ Eq };

//	Functionally Isolated Task (Fit)

// See Par.rs for more info about what a Fit is

#[deriving(Eq)]
enum ParFitComm { 
	DoFit( ~str, ~Object ), // ( t_key, args )
	ParFitCommEndChan
}
#[deriving(Eq)]
enum FitComm { 
	FitOk( ~str, ~Object ), // ( t_key, args )
	FitErr( ~str, ~Object ), // ( t_key, errors )
	FitTryFail( ~str ),  // ( t_key )
	FitSysErr( ~Object )  // resource fail message from Rust that breaks this fit
}

trait ParFitable {
	fn connect( &self ) -> ( Port<FitComm>, Chan<ParFitComm> );
	fn fit_key( &self ) -> ~str;
}


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
extern mod jah_args; 
use std::time::Timespec;
use std::comm::DuplexStream;
use jah_args::{ JahArgs };

//	Functionally Isolated Task (Fit)

enum ParFitComm {
	DoFit( ~str, JahArgs ),
	ParFitCommEndChan
}

enum FitComm {
	FitOk( ~str, Timespec, JahArgs ),
	FitErr( ~str, Timespec, JahArgs ),
	FitTryFail( ~str, Timespec )
}

trait ParFitable {
	fn connect( &self, par_plex: DuplexStream<FitComm, ParFitComm> );
	fn key(&self) -> ~str;
	fn spawnable( &self ) -> bool;
}
//	Copyright 2013 Jeff Murray.

//	Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//	http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//	<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//	option. This file may not be copied, modified, or distributed
//	except according to those terms.

#[link(name = "err_fit", vers = "1.0")];

//	rustc --lib err_fit.rs -L .
//	rustc err_fit.rs --test -o err_fit-tests -L .
//	./err_fit-tests

extern mod std;
extern mod core;
extern mod fit;
extern mod bootstrap;
extern mod jah_spec;
extern mod jah_args;
use core::comm::{ stream, Port, Chan };  //, SharedChan
use std::json::{ Object, ToJson, String };//,Number,List, PrettyEncoder
use bootstrap::{ Bootstrap };
use core::hashmap::linear::LinearMap;
use fit::{ ParFitable, ParFitComm, DoFit, ParFitCommEndChan, FitOk};  //, FitSysErr, FitErr, FitComm, FitTryFail 
use jah_spec::{ JahSpeced }; 
use jah_args::{ JahArgs };

//Implements an append only JSON writer that takes a streamable json map and 
//calculates and writes some system JSONAppendReply variables to an accounting doc 
struct ErrFit {
	priv settings: ~Object
};
	
impl ParFitable for ErrFit {

	fn new( settings: ~Object ) -> ~ErrFit {
		settings = settings;
	}
	
	fn connect( &self ) -> Result<Chan<ParFitComm>, ~Object> {
	
		let ( in_port, in_chan ) = stream();
		self.go( in_port );
		Ok( in_chan )
	}
	
	fn fit_key( &self ) -> ~str {
	
		~"EHR6DFySwtSHzlb2" //unique randomly-generated id identifying the code implementing 
							//the fit.  Hopefully there will be associated documentation in 
							//the Must Document System
	}
	
}

impl JahSpeced for ErrFit {
	
	fn specs_in( &self ) -> ~Object {
	
		let mut allowed = ~LinearMap::new();
		// just pass the Object returned from FitComm, it can be sorted out on the index later on
		allowed.insert( ~"err_args", ~[Bootstrap::arg_rule_obj_must_be_object().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json()] );
		allowed.insert( ~"spec_key", ~[Bootstrap::arg_rule_arg_must_be_string().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json()] );
		let mut spec = ~LinearMap::new();
		spec.insert( ~"allowed", allowed.to_json() );
		spec.insert( ~"spec_key", String(~"qJzMxt6eQV2CgKbr").to_json() );
		let mut specs  = ~LinearMap::new();
		specs.insert( ~"qJzMxt6eQV2CgKbr", Object( spec ) );
		specs
	}
	
	fn specs_out( &self ) -> ~Object {
	
		let mut allowed = ~LinearMap::new();
		allowed.insert( ~"spec_key", ~[Bootstrap::arg_rule_arg_must_be_string().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json()] );
		let mut spec = ~LinearMap::new();
		spec.insert( ~"allowed", allowed.to_json() );
		spec.insert( ~"spec_key", String(~"er5OWig71VG9oNjK").to_json() );
		let mut specs  = ~LinearMap::new();
		specs.insert( ~"er5OWig71VG9oNjK", Object( spec ) );
		specs
	}
}

impl ErrFit {

	fn new() -> ~ErrFit {
		~ErrFit
	}

	//The Par always call go() using spawn because go does not exit  
	//unless the channel sends a request with is_stop_order set to true

	priv fn go ( &self, par_port: Port<ParFitComm> ) {

		do spawn {	
			loop {
				match par_port.recv() {
		  			DoFit( args, home_chan ) => {
		  				io::println( JahArgs::new( args ).to_str() );
		  				let mut r_args = ~LinearMap::new();
		  				r_args.insert(  ~"spec_key", String(~"er5OWig71VG9oNjK").to_json() );
		 				home_chan.send( FitOk( copy r_args ) );
		  			}
					ParFitCommEndChan => {
						break;
					}
				}
			}
		}
	}
}

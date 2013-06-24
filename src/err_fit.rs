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
struct ErrFit;
	
impl ParFitable for ErrFit {

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

/*
#[test]
fn test_write_and_read() {
	let fit = ~FileAppendJSON{ 
		file_path: ~"test.json",
		file_num: 0
		};
		
	let ( port, chan ) = fit.connect();
	let mut doc = ~LinearMap::new();
	doc.insert( ~"message",String( ~"하세요!" ) );
	let mut args = ~LinearMap::new();
	args.insert( ~"user", String( ~"va4wUFbMV78R1AfB" ) );
	args.insert( ~"acct", String( ~"ofWU4ApC809sgbHJ" ) );
	args.insert( ~"must", Must::new_must().to_json() );	
	args.insert( ~"doc", doc.to_json() );
	args.insert( ~"spec_key", String(~"uHSQ7daYUXqUUPSo").to_json() );

	chan.send( DoFit( ~"p5M8DIzbKWYhKR5W", copy args ) );
	let slice_len = JahArgs::new( copy args ).to_str().len();
	let rval = {
		match port.recv() {
			FitOk( t_key, rval ) => {
				assert!( t_key == ~"p5M8DIzbKWYhKR5W" );
				rval
			}
			FitSysErr( err ) => {
				io::println( JahArgs::new( err ).to_str() );
				chan.send( ParFitCommEndChan );
				fail!();
			}
			FitErr( t_key, err ) => {
				io::println( JahArgs::new( err ).to_str() );
				chan.send( ParFitCommEndChan );
				fail!();
			}		
			_ => { chan.send( ParFitCommEndChan ); fail!(); }
		}};
	let ospecs = copy *fit.specs_out();
	let ospec = copy *ospecs.find(&~"ma2snwuG8VPGxY8z").get();
	match ospec {
		Object( out_spec ) => {
			let jah = JahArgs::new( rval );
			assert!( JahSpec::new( out_spec ).check_args( copy jah ).is_ok() );
			let slice = JahArgs::new( ~jah.get_map( ~"slice" ).get() );
			let len: uint = slice.get_float( ~"len" ).get().to_uint();
			assert!( len == slice_len );
		}
		_ => { fail!(); }
	}
	chan.send( ParFitCommEndChan );
}
*/
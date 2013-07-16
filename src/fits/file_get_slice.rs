//	Copyright 2013 Jeff Murray.

//	Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//	http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//	<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//	option. This file may not be copied, modified, or distributed
//	except according to those terms.

#[link(name = "file_get_slice", vers = "1.0")];

//	rustc --lib fits/file_get_slice.rs -L . -L fits
//	rustc fits/file_get_slice.rs --test -o fits/file_get_slice-tests -L . -L fits
//	./fits/file_get_slice-tests

extern mod std;
extern mod extra;
extern mod fit;
extern mod bootstrap;
extern mod jah_spec;
extern mod jah_args;
extern mod must;
extern mod file_append_json;
use std::io::{ SeekSet };
use std::comm::{ SharedChan, stream, Port, Chan, ChanOne, oneshot, recv_one };
use extra::json::{ Object, ToJson, String }; 
use bootstrap::{ Bootstrap };
use std::hashmap::HashMap;
use fit::{ Parfitable, ParFitComm, DoFit, ParFitCommEndChan, FitOk, FitErr, FitSysErr, FitErrs, FitArgs };  
use jah_spec::{ JahSpeced, JahSpec }; 
use jah_args::{ JahArgs };
use must::{ Must };
use file_append_json::{ FileAppendJSON };
//	FileGetSlice receives a doc that identifies a slice of the file to retrieve and returns it as a binary attachment. 
//	The Fit then calculates and sends slice info or errors through a oneshot it received with the args  

pub struct FileGetSlice {
	file_args: ~Object
}
	
impl Parfitable for FileGetSlice {

	pub fn new( settings: ~Object ) -> ~FileGetSlice {
	
		~FileGetSlice { file_args: settings }
	}
	
	pub fn connect( &self ) -> Result<Chan<ParFitComm>, ~FitErrs> {
	
		let ( in_port, in_chan ) = stream();
		match self.go( in_port ) {
			Ok( _ ) => { Ok( in_chan ) }
			Err( errs ) => { Err( errs ) }
		}
	}
	
	pub fn fit_key( &self ) -> ~str {
	
		~"tGMl7e89oQlKKQVu" 
	}
}

impl JahSpeced for FileGetSlice {
	
	fn spec_keys_in( &self ) -> ~[~str] {
	
		~[Bootstrap::spec_file_slice_key()]
	}
	
	fn spec_keys_out( &self ) -> ~[~str] {
	
		~[Bootstrap::spec_find_slice_result_key()]
	}
}

impl FileGetSlice {

	priv fn spawn_and_read( home: SharedChan<ChanOne<( ~str, ~str, ParFitComm ) >>) {
	
		do spawn {
			let ( p, c ) = oneshot();
			home.send( c );
			let ( fit_key, path_str, parfit_comm ): ( ~str, ~str, ParFitComm ) =  p.recv();
			match parfit_comm {
				DoFit( fit_args, home_chan ) => {
					let path = Path( path_str );
					let file_reader_rslt = std::io::file_reader( &path );
					if file_reader_rslt.is_err() {
		  				home_chan.send( FitSysErr( FitErrs::from_object( Bootstrap::fit_sys_err( fit_args.doc , copy file_reader_rslt.get_err(), copy fit_key, ~"file_get_slice.rs", ~"jlSoLMf7JAOKMF6A" ) ) ) );			  				
					} else {
						let file_reader = file_reader_rslt.get();
						let jah = JahArgs::new( fit_args.doc );
						//Fits are guaranteed that the jah_spec for the incoming doc have already been checked
						//to insure the args exist and their data type is correct, so I am going straight for the val 
						let pos = jah.get_float(~"pos").get().to_int();
						let len = jah.get_float(~"len").get().to_uint();
						file_reader.seek( pos, SeekSet );
						let mut args = ~HashMap::new();
						args.insert( ~"attach", String(~"file_slice" ).to_json() );
						args.insert( ~"spec_key", String( Bootstrap::spec_find_slice_result_key() ).to_json() );
						file_reader.seek( pos, SeekSet );
						let mut file_slice = std::vec::from_elem(len, 0_u8);
						file_reader.read( file_slice, len );
						home_chan.send( FitOk( ~FitArgs{ doc: args, attach: file_slice } ) );
					}		
				} ParFitCommEndChan => {} // the calling function does not send ParFitCommEndChan and this is a private function
			}
		}
	}
	
	//Implements a spawned read

	priv fn go ( &self, in_port: Port<ParFitComm> ) -> Result<bool, ~FitErrs> {

    	let fit_key = self.fit_key();
    	let ( file_path, _ ) = { 
    		match self.get_startup_args() {
	    		Ok( ( file_path, file_num ) ) => {
	    			( file_path, file_num )
	    		}
	    		Err( fit_sys_errs ) => { 
	    			return Err( fit_sys_errs.prepend_err( Bootstrap::reply_error_trace_info( ~"file_append_json.rs", ~"eUZCAcGIlfzXEsJi" ) ) );
	    		}
    		}};
		//I open reader at the beginning while I can still deny the communication channel ;)
		let file_reader_rslt = std::io::file_reader( &Path( copy file_path ) );
		if file_reader_rslt.is_err() {
	  		return Err( FitErrs::from_object( Bootstrap::fit_sys_err( ~HashMap::new(), copy file_reader_rslt.get_err(), copy fit_key, ~"file_append_json.rs", ~"YXoR14QfuczXLyeh" ) ) );			  				
		}
    	//At the point of writing this, I do not fully understand the pro's and con's
    	//related to spawning and opening a new reader every time a slice is read.
    	//I will likely make a sequential reader as well, and compare them.
		do spawn {	
			let path = Path( copy file_path );
			let file_reader_rslt = std::io::file_reader( &path );
			if file_reader_rslt.is_err() {
				match in_port.recv() {
		  			DoFit( _ , home_chan ) => {
		  				home_chan.send( FitErr( FitErrs::from_object( Bootstrap::fit_sys_err( ~HashMap::new() , copy file_reader_rslt.get_err(), copy fit_key, ~"file_get_slice.rs", ~"5kQeNVLDkteS1c2w" ) ) ) );			  				
		  			} _ => {}
		  		}
			} else {
				loop {
					let ( sp, sc ) = stream();
					let sc = SharedChan::new( sc );
					let parfit_comm = in_port.recv();
					match parfit_comm {
						ParFitCommEndChan => {
							break;
						},
						_ => {
							FileGetSlice::spawn_and_read( sc.clone() );
							sp.recv().send(( copy fit_key, copy file_path, parfit_comm ));
			  			}
					}	
				}
			}
		}
		Ok( true )
	}
	
	priv fn arg_out( &self ) -> ~Object {
	
		let mut allowed = ~HashMap::new();
		allowed.insert( ~"path", ~[Bootstrap::arg_rule_arg_must_be_string().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json()] );
		allowed.insert( ~"num", ~[Bootstrap::arg_rule_num_must_be_number().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json()] );
		allowed.insert( ~"spec_key", ~[Bootstrap::arg_rule_arg_must_be_string().to_json(), Bootstrap::arg_rule_arg_must_exist().to_json()] );
		let mut spec = ~HashMap::new();
		spec.insert( ~"allowed", allowed.to_json() );
		spec.insert( ~"spec_key", String(~"5W6emlWjT77xoGOH").to_json() );
		spec
	}
	
	fn get_startup_args( &self ) -> Result<( ~str, uint ), ~FitErrs > {
	
		let args = JahArgs::new( copy self.file_args );
		let spec = JahSpec::new( self.arg_out() );
		match spec.check_args( copy args ) {
			Ok( _ ) => { }
			Err( errs ) => {
				return Err( FitErrs::from_objects( ~[Bootstrap::reply_error_trace_info(~"file_get_slice.rs", ~"rx9vMuM19wlGvMm2" )]  + errs ) );
			}
		}
		// Since args has passed a spec check, I am pretty confident using .get()		
		let file_path = copy args.get_str( ~"path" ).get();
		let file_num = args.get_float( ~"num" ).get().to_uint();
		Ok( ( file_path, file_num ) )
	}
}

#[test]
fn test_write_and_read() {
	let slice_args = {
		let fit = ~FileAppendJSON{ 
			file_args: {
				let mut startup_args = ~HashMap::new();
				startup_args.insert( ~"path", String(~"test.json").to_json() );
				startup_args.insert( ~"num", 1u.to_json() );
				startup_args.insert( ~"spec_key", String(~"5W6emlWjT77xoGOH").to_json() );
				startup_args
			}};
		let fit_chan = { 
			match fit.connect() {
				Ok( chan ) => {
					chan
				}
				Err( fit_errs ) => {
					println( fit_errs.to_str() );
					fail!();
				}
			}};
		let mut doc = ~HashMap::new();
		doc.insert( ~"message",String( ~"하세요!" ) );
		let mut args = ~HashMap::new();
		args.insert( ~"user", String( ~"va4wUFbMV78R1AfB" ) );
		args.insert( ~"acct", String( ~"ofWU4ApC809sgbHJ" ) );
		args.insert( ~"must", Must::new().to_json() );	
		args.insert( ~"doc", doc.to_json() );
		args.insert( ~"spec_key", String(~"uHSQ7daYUXqUUPSo").to_json() );
		let rval = {
			match { let ( p, c ) = oneshot();
					fit_chan.send( DoFit( ~FitArgs::from_doc( copy args ), c ) );
					recv_one( p )
			} {
				FitOk( rval ) => {
					fit_chan.send ( ParFitCommEndChan );
					rval
				}
				FitSysErr( fit_errs ) => {
					println( fit_errs.to_str() );
					fail!();
				}
				FitErr( fit_errs ) => {
					println( fit_errs.to_str() );
					fail!();
				}
			}};
		let jah = JahArgs::new( copy rval.doc );
		assert!( JahSpec::new( Bootstrap::find_spec( Bootstrap::spec_file_slice_key() ) ).check_args( copy jah ).is_ok() );
		let len: uint = jah.get_float( ~"len" ).get().to_uint();
		assert!( len == JahArgs::new( args ).to_str().len() );
		rval
	};
	
	
	let fit = ~FileGetSlice{ 
		file_args: {
			let mut startup_args = ~HashMap::new();
			startup_args.insert( ~"path", String(~"test.json").to_json() );
			startup_args.insert( ~"num", 1u.to_json() );
			startup_args.insert( ~"spec_key", String(~"5W6emlWjT77xoGOH").to_json() );
			startup_args
		}};
	let fit_chan = { 
		match fit.connect() {
			Ok( chan ) => {
				chan
			}
			Err( fit_errs ) => {
				println( fit_errs.to_str() );
				fail!();
			}
		}};
	
	let rval = { 
		match { let ( p, c ) = oneshot();
			fit_chan.send( DoFit( copy slice_args, c ) );
			recv_one( p )
		} {
			FitOk( rval ) => {
				fit_chan.send ( ParFitCommEndChan );
				rval
			}
			FitSysErr( fit_errs ) => {
				println( fit_errs.to_str() );
				fail!();
			}
			FitErr( fit_errs ) => {
				println( fit_errs.to_str() );
				fail!();
			}
		}};	
	
	let jah = JahArgs::new( copy rval.doc );
	assert!( JahSpec::new( Bootstrap::find_spec( Bootstrap::spec_find_slice_result_key() ) ).check_args( copy jah ).is_ok() );
	assert!( jah.get_str( ~"attach" ).get() == ~"file_slice" );
	let jah_slice = JahArgs::new( slice_args.doc );
	assert!( rval.attach.len() == jah_slice.get_float(~"len").get().to_uint() );
	
	match extra::json::from_str( rval.attach.to_str() ).get() {
		Object( val ) => {
			assert!( JahSpec::new( Bootstrap::find_spec( Bootstrap::spec_doc_key() ) ).check_args( JahArgs::new( val) ).is_ok() );
		} _ => {}
	}
}
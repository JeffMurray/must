//	Copyright 2013 Jeff Murray.

//	Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//	http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//	<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//	option. This file may not be copied, modified, or distributed
//	except according to those terms.

#[link(name = "file_append_json", vers = "1.0")];

//	rustc --lib fits/file_append_json.rs -L .
//	rustc fits/file_append_json.rs --test -o fits/file_append_json-tests -L . -L fits
//	./fits/file_append_json-tests

extern mod std;
extern mod extra;
extern mod fit;
extern mod bootstrap;
extern mod jah_spec;
extern mod jah_args;
extern mod must;
use std::io::{ SeekEnd };
use std::comm::{ stream, Port, Chan, oneshot, recv_one }; // oneshot and recv_one are used in unit tests
use extra::json::{ Object, ToJson, PrettyEncoder, String };//,Number, Json , List
use bootstrap::{ Bootstrap };
use extra::serialize::Encodable;
use std::io::{ Create, Append };
use std::hashmap::HashMap;
use fit::{ Parfitable, ParFitComm, DoFit, ParFitCommEndChan, FitOk, FitErr, FitSysErr, FitArgs, FitErrs}; //, FitComm, FitTryFail 
use jah_spec::{ JahSpeced, JahSpec }; 
use jah_args::{ JahArgs };
use must::{ Must }; // used in unit tests

//	FileAppendJSON receives a document through ParFitComm and appends it to the end of the file described in
//	self.file_args. 
//	The Fit then calculates and sends slice info or errors through a oneshot it received with the args  

pub struct FileAppendJSON {
	file_args: ~Object
}
	
impl Parfitable for FileAppendJSON {

	pub fn new( settings: ~Object ) -> ~FileAppendJSON {
	
		~FileAppendJSON { file_args: settings }
	}
	
	pub fn connect( &self ) -> Result<Chan<ParFitComm>, ~FitErrs> {
	
		let ( in_port, in_chan ) = stream();
		match self.go( in_port ) {
			Ok( _ ) => { Ok( in_chan ) }
			Err( errs ) => { Err( errs ) }
		}
	}
	
	pub fn fit_key( &self ) -> ~str {
	
		~"S68yWotrIh06IdE8" //unique randomly-generated id identifying the code implementing 
							//the fit.  Hopefully there will be associated documentation in 
							//the Must Document System
	}
}

impl JahSpeced for FileAppendJSON {
	
	fn spec_keys_in( &self ) -> ~[~str] {
	
		~[~"uHSQ7daYUXqUUPSo"]
	}
	
	fn spec_keys_out( &self ) -> ~[~str] {
	
		~[~"ma2snwuG8VPGxY8z"]
	}
}

impl FileAppendJSON {
	
	//Implements a sequential writer for single Must controlled file

	priv fn go ( &self, in_port: Port<ParFitComm> ) -> Result<bool, ~FitErrs> {

    	//I do not understand the pro's and con's
    	//related to opening a new writer for every append in Rust.
    	//I'm guessing opening with with every write could get expensive
    	//given that writes could be heavy. 
    			
    	let fit_key = self.fit_key();
    	let ( file_path, file_num ) = { match self.get_startup_args() {
    		Ok( ( file_path, file_num ) ) => {
    			( file_path, file_num )
    		}
    		Err( fit_sys_errs ) => { 
    			return Err( FitErrs::from_objects( ~[Bootstrap::reply_error_trace_info( ~"file_append_json.rs", ~"eUZCAcGIlfzXEsJi" )] + fit_sys_errs ) );
    		}
    		}};
    	println( file_path );
    	let path = Path( file_path );
    	let spec = JahSpec::new( Bootstrap::find_spec( ~"uHSQ7daYUXqUUPSo" ) );
		if spec.spec_key() !=  ~"uHSQ7daYUXqUUPSo" {
			return Err( FitErrs::from_object( Bootstrap::fit_sys_err( copy self.file_args, ~"Missing expected key uHSQ7daYUXqUUPSo", copy fit_key, ~"file_append_json.rs", ~"wi8D6MEqdXkORYtX") ) );
		}				
		do spawn {	
			let append_writer_rslt = std::io::mk_file_writer( &path, &[Create, Append] );
			let file_reader_rslt = std::io::file_reader( &path );
			if append_writer_rslt.is_err() {
				match in_port.recv() {  //send the error to the first thing that communicates
		  			DoFit( args, home_chan ) => {
		  				home_chan.send( FitSysErr( FitErrs::from_object( Bootstrap::fit_sys_err( args.doc, copy append_writer_rslt.get_err(), copy fit_key, ~"file_append_json.rs", ~"aP5FFu7dV0xNr4MZ" ) ) ) );			  				
		  			} _ => {}
		  		}
			} else if file_reader_rslt.is_err() {
				match in_port.recv() {
		  			DoFit( args, home_chan ) => {
		  				home_chan.send( FitSysErr(  FitErrs::from_object( Bootstrap::fit_sys_err( args.doc, copy file_reader_rslt.get_err(), copy fit_key, ~"file_append_json.rs", ~"mKdumoT12u9UsAQg" ) ) ) );			  				
		  			} _ => {}
		  		}
			} else {
				let append_writer =  append_writer_rslt.get();
				let file_reader = file_reader_rslt.get();
				loop {
					//This loop assumes it is the only writer for this file
					//its managing implementations should insure this.
					//We will not spawn, except when sending replies to insure 
					//appends to this file are sequential.
					match in_port.recv() {
						ParFitCommEndChan => {
							break;
						},
			  			DoFit( args, home_chan ) => {
							match spec.check_args( JahArgs::new( copy args.doc ) ) {
								Ok( _ ) => { 
									
									//get current the ending position of the file
									file_reader.seek( 0, SeekEnd );
									let start_pos = file_reader.tell();
									
									//write the doc with all of the args
									let mut encoder = PrettyEncoder(append_writer);
				        			args.doc.encode(&mut encoder);						
									append_writer.flush();
									
									//get the new ending position of the file
									file_reader.seek( 0, SeekEnd );
									
									//calculate the slice info that will get stored with the documents
									//master index
							        let mut slice = ~HashMap::new();
			        				slice.insert( ~"pos", start_pos.to_json() );
								    slice.insert( ~"len", ( file_reader.tell() - start_pos ).to_json() );
								    slice.insert( ~"fn", file_num.to_json() );
									slice.encode(&mut encoder);
									
									//write the slice info to the file for redundancy purposes
									append_writer.flush();
									
									//put the return args together and send them home
									let mut r_args = ~HashMap::new();
									r_args.insert( ~"slice", slice.to_json() );
									r_args.insert( ~"spec_key", (~"WZody857ygg3YF1x").to_json() );
									home_chan.send( FitOk( ~FitArgs::from_doc( r_args ) ) );
								},
								Err( errs ) => {
									home_chan.send( FitErr( FitErrs::from_objects( ~[Bootstrap::reply_error_trace_info( ~"file_append_json.rs", ~"hiLXpCZ3nbya2Oea" )] + errs ) ) );
								}
							}
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
	
	fn get_startup_args( &self ) -> Result<( ~str, uint ), ~[~Object] > {
	
		let args = JahArgs::new( copy self.file_args );
		let spec = JahSpec::new( self.arg_out() );
		match spec.check_args( copy args ) {
			Ok( _ ) => { }
			Err( errs ) => {
				return Err( ~[Bootstrap::reply_error_trace_info(~"file_append_json.rs", ~"rx9vMuM19wlGvMm2" )] + errs );
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
	let rval = match { let ( p, c ) = oneshot();
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
		}};
	let jah = JahArgs::new( rval.doc );
	assert!( JahSpec::new( Bootstrap::find_spec( ~"ma2snwuG8VPGxY8z" ) ).check_args( copy jah ).is_ok() );
	let slice = JahArgs::new( jah.get_map( ~"slice" ).get() );
	let len: uint = slice.get_float( ~"len" ).get().to_uint();
	assert!( len == JahArgs::new( args ).to_str().len() );
}

	
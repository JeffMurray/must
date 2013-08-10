//	Copyright 2013 Jeff Murray.

//	Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//	http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//	<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//	option. This file may not be copied, modified, or distributed
//	except according to those terms.

#[link(name = "file_append_slice", vers = "0.0")];

//	rustc --lib fits/file_append_slice.rs -L .
//	rustc fits/file_append_slice.rs --test -o fits/file_append_slice-tests -L . -L fits
//	./fits/file_append_slice-tests

extern mod std;
extern mod extra;
extern mod fit;
extern mod bootstrap;
extern mod jah_spec;
extern mod jah_args;
extern mod must;
use std::io::{ SeekEnd, BytesWriter };
use std::comm::{ stream, Port, Chan, oneshot }; // oneshot is used in unit tests
use extra::json::{ Object, ToJson, PrettyEncoder, String };//,Number, Json , List
use bootstrap::{ Bootstrap };
use extra::serialize::Encodable;
use std::io::{ Create, Append };
use std::hashmap::HashMap;
use fit::{ Parfitable, ParFitComm, DoFit, ParFitCommEndChan, FitOk, FitErr, FitSysErr, FitArgs, FitErrs}; //, FitComm, FitTryFail 
use jah_spec::{ JahSpeced, JahSpec }; 
use jah_args::{ JahArgs };
use must::{ Must }; // used in unit tests

//	FileAppendSlice receives a binary slice through ParFitComm and appends it to the end of the file described in
//	self.file_args. 
//	The Fit then calculates and sends slice info or errors through a oneshot it received with the args  

pub struct FileAppendSlice {
	file_args: ~Object
}
	
impl Parfitable for FileAppendSlice {

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

impl JahSpeced for FileAppendSlice {
	
	fn spec_keys_in( &self ) -> ~[~str] {
	
		~[Bootstrap::spec_append_slice_key()]
	}
	
	fn spec_keys_out( &self ) -> ~[~str] {
	
		~[Bootstrap::spec_file_slice_key()]
	}
}

impl FileAppendSlice {
	
	//Implements a sequential append only binary writer for single Must controlled file

	priv fn go ( &self, in_port: Port<ParFitComm> ) -> Result<bool, ~FitErrs> {

    	//I do not understand the pro's and con's
    	//related to opening a new writer for every append in Rust.
    	//I'm guessing opening with with every write could get expensive
    	//given that writes could be heavy. 
    			
    	let fit_key = self.fit_key();
    	let ( file_path, file_num ) = { 
    		match self.get_startup_args() {
	    		Ok( ( file_path, file_num ) ) => {
	    			( file_path, file_num )
	    		}
	    		Err( fit_sys_errs ) => { 
	    			return Err( FitErrs::from_objects( ~[Bootstrap::reply_error_trace_info( ~"file_append_slice.rs", ~"eUZCAcGIlfzXEsJi" )] + fit_sys_errs ) );
	    		}
    		}};
    	let path = Path( file_path );
    	let spec = Bootstrap::find_spec( Bootstrap::spec_add_doc_key() );
		if JahSpec::spec_key(&spec) != Bootstrap::spec_add_doc_key()  {
			return Err( FitErrs::from_object( Bootstrap::fit_sys_err( copy self.file_args, ~"Missing expected key uHSQ7daYUXqUUPSo", copy fit_key, ~"file_append_slice.rs", ~"cSCDVSNDFpLOSwDz") ) );
		}
		{	//checking file opening abilities before establishing a spawned channel
			let append_writer_rslt = std::io::mk_file_writer( &path, &[Create, Append] );
			let file_reader_rslt = std::io::file_reader( &path );
			if append_writer_rslt.is_err() {
				return Err( FitErrs::from_object( Bootstrap::fit_sys_err( copy self.file_args , copy append_writer_rslt.get_err(), copy fit_key, ~"file_append_slice.rs", ~"zc9sQbV5cvyhYOUD" ) ) );			  				
			} else if file_reader_rslt.is_err() {
				return  Err(  FitErrs::from_object( Bootstrap::fit_sys_err( copy self.file_args, copy file_reader_rslt.get_err(), copy fit_key, ~"file_append_slice.rs", ~"tUMNwzzD2qFXXomQ" ) ) );			  				
			}
		}
		do spawn {	
			let append_writer_rslt = std::io::mk_file_writer( &path, &[Create, Append] );
			let file_reader_rslt = std::io::file_reader( &path );
			if append_writer_rslt.is_err() {
				match in_port.try_recv().expect("file_append_slice 1") {  //send the error to the first thing that communicates
		  			DoFit( args, _, home_chan ) => {
		  				home_chan.send( FitSysErr( FitErrs::from_object( Bootstrap::fit_sys_err( args.doc, copy append_writer_rslt.get_err(), copy fit_key, ~"file_append_slice.rs", ~"aP5FFu7dV0xNr4MZ" ) ) ) );			  				
		  			} _ => {}
		  		}
			} else if file_reader_rslt.is_err() {
				match in_port.try_recv().expect("file_append_slice 2") {
		  			DoFit( args, _, home_chan ) => {
		  				home_chan.send( FitSysErr(  FitErrs::from_object( Bootstrap::fit_sys_err( args.doc, copy file_reader_rslt.get_err(), copy fit_key, ~"file_append_slice.rs", ~"Ov1duvNzsrX9syZb" ) ) ) );			  				
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
					match in_port.try_recv().expect("file_append_slice 3") {
						ParFitCommEndChan => {
							break;
						},
			  			DoFit( args, _, home_chan ) => {
			  				if args.attach.len() == 0 {
		  						home_chan.send( FitErr ( FitErrs::from_object( Bootstrap::logic_error( Bootstrap::slice_len_cannot_be_zero(), ~"attach", ~"e42iDEm1ulsqawrf", ~"file_append_slice.rs") ) ) );
		  					} else {
			  					//No need to check args because all fits have their args checked 
				  				//according to spec prior to getting called and doc_slice_prep has already sliced the document
								//get current the ending position of the file
								file_reader.seek( 0, SeekEnd );
								let start_pos = file_reader.tell();
								
								append_writer.write( args.attach );
								append_writer.flush();
								
								//calculate the slice info that will get stored with the documents
								//master index
						        let mut slice = ~HashMap::new();
		        				slice.insert( ~"pos", start_pos.to_json() );
							    slice.insert( ~"len", args.attach.len().to_json() );
							    slice.insert( ~"fn", file_num.to_json() );
								//put the return args together and send them home
								//let mut r_args = ~HashMap::new();
								slice.insert( ~"spec_key", (Bootstrap::spec_file_slice_key()).to_json() );
								home_chan.send( FitOk( ~FitArgs::from_doc( slice ) ) );
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
	
		match JahSpec::check_args( &self.arg_out(), &self.file_args ) {
			Ok( _ ) => { }
			Err( errs ) => {
				return Err( ~[Bootstrap::reply_error_trace_info(~"file_append_slice.rs", ~"rx9vMuM19wlGvMm2" )] + errs );
			}
		}
		// Since args has passed a spec check, I am pretty confident using .get()		
		let file_path = self.file_args.get_str( ~"path" ).get();
		let file_num = self.file_args.get_float( ~"num" ).get().to_uint();
		Ok( ( file_path, file_num ) )
	}
}

#[test]
fn test_write_and_read() {
	let fit = ~FileAppendSlice{ 
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
	args.insert( ~"spec_key", String( Bootstrap::spec_stored_doc_key() ).to_json() );
	let bw = @BytesWriter::new();
	let mut encoder = PrettyEncoder( bw as @Writer );
	args.to_json().encode( &mut encoder );				
	bw.flush();	
	let mut r_doc = ~HashMap::new();
	r_doc.insert( ~"attach", String(~"doc").to_json() );
	r_doc.insert( ~"spec_key", String( Bootstrap::spec_append_slice_key() ).to_json() );	
	let rval = {
		match { let ( p, c ) = oneshot();
			fit_chan.send( DoFit( ~FitArgs::from_doc_with_attach( doc, copy *bw.bytes ), ~Must::new(), c ) );
			p.recv()
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
	assert!( JahSpec::check_args( &Bootstrap::find_spec( Bootstrap::spec_file_slice_key() ),  &rval.doc ).is_ok() );
	let len = rval.doc.get_float( ~"len" ).get().to_uint();
	assert!( len == bw.bytes.len() );
}

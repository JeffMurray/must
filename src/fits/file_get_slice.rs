//	Copyright 2013 Jeff Murray.

//	Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//	http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//	<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//	option. This file may not be copied, modified, or distributed
//	except according to those terms.

#[link(name = "file_get_slice", vers = "0.0")];

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
extern mod file_append_slice;
use extra::serialize::Encodable;
use file_append_slice::{ FileAppendSlice };
use std::io::{ SeekSet, BytesWriter };
use std::comm::{ SharedChan, stream, Port, Chan, ChanOne, oneshot };
use extra::json::{ Object, ToJson, String, PrettyEncoder }; 
use bootstrap::{ Bootstrap };
use std::hashmap::HashMap;
use fit::{ Parfitable, ParFitComm, DoFit, ParFitCommEndChan, FitOk, FitErr, FitSysErr, FitErrs, FitArgs };  
use jah_spec::{ JahSpeced, JahSpec }; 
use jah_args::{ JahArgs };
use must::{ Must };
//	FileGetSlice receives a doc that identifies a slice of the file to retrieve and returns it as a binary attachment. 
//	The Fit then calculates and sends slice info or errors through a oneshot it received with the args  

pub struct FileGetSlice {
	file_args: ~Object
}
	
impl Parfitable for FileGetSlice {

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
			let ( fit_key, path_str, parfit_comm ): ( ~str, ~str, ParFitComm ) =  p.try_recv().expect("file_get_slice 1");
			match parfit_comm {
				DoFit( fit_args, _, home_chan ) => {
					let path = Path( path_str );
					let file_reader_rslt = std::io::file_reader( &path );
					if file_reader_rslt.is_err() {
		  				home_chan.send( FitSysErr( FitErrs::from_object( Bootstrap::fit_sys_err( fit_args.doc , copy file_reader_rslt.get_err(), copy fit_key, ~"file_get_slice.rs", ~"jlSoLMf7JAOKMF6A" ) ) ) );			  				
					} else {
						let file_reader = file_reader_rslt.get();
						//Fits are guaranteed that the jah_spec for the incoming doc have already been checked
						//to insure the args exist and their data type is correct, so I am going straight for the val 
						let pos = fit_args.doc.get_float(~"pos").get().to_int();
						let len = fit_args.doc.get_float(~"len").get().to_uint();
						file_reader.seek( pos, SeekSet );
						let mut args = ~HashMap::new();
						args.insert( ~"attach", String(~"file_slice" ).to_json() );
						args.insert( ~"spec_key", String( Bootstrap::spec_find_slice_result_key() ).to_json() );
						file_reader.seek( pos, SeekSet );
						let mut file_slice = std::vec::from_elem(len, 0_u8);
						file_reader.read( file_slice, len );
						home_chan.send( FitOk( ~FitArgs::from_doc_with_attach( args, file_slice ) ) );
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
	    			return Err( fit_sys_errs.prepend_err( Bootstrap::reply_error_trace_info( ~"file_get_slice.rs", ~"Pj3l7Xw3UHVRjiqv" ) ) );
	    		}
    		}};
		//I open reader at the beginning while I can still deny the communication channel ;)
		let file_reader_rslt = std::io::file_reader( &Path( copy file_path ) );
		if file_reader_rslt.is_err() {
	  		return Err( FitErrs::from_object( Bootstrap::fit_sys_err( copy self.file_args, copy file_reader_rslt.get_err(), copy fit_key, ~"file_get_slice.rs", ~"YXoR14QfuczXLyeh" ) ) );			  				
		}
    	//At the point of writing this, I do not fully understand the pro's and con's
    	//related to spawning and opening a new reader every time a slice is read.
    	//I will likely make a sequential reader as well, and compare them.
		do spawn {	
			let path = Path( copy file_path );
			let file_reader_rslt = std::io::file_reader( &path );
			if file_reader_rslt.is_err() {
				match in_port.try_recv().expect("file_get_slice 2") {
		  			DoFit( _, _, home_chan ) => {
		  				home_chan.send( FitErr( FitErrs::from_object( Bootstrap::fit_sys_err( ~HashMap::new() , copy file_reader_rslt.get_err(), copy fit_key, ~"file_get_slice.rs", ~"5kQeNVLDkteS1c2w" ) ) ) );			  				
		  			} _ => {}
		  		}
			} else {
				loop {
					let ( sp, sc ) = stream();
					let sc = SharedChan::new( sc );
					let parfit_comm = in_port.try_recv().expect("file_get_slice 3");
					match parfit_comm {
						ParFitCommEndChan => {
							break;
						},
						_ => {
							FileGetSlice::spawn_and_read( sc.clone() );
							sp.try_recv().expect("file_get_slice 4").send(( copy fit_key, copy file_path, parfit_comm ));
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

		match JahSpec::check_args( &self.arg_out(), &self.file_args ) {
			Ok( _ ) => { }
			Err( errs ) => {
				return Err( FitErrs::from_objects( ~[Bootstrap::reply_error_trace_info(~"file_get_slice.rs", ~"rx9vMuM19wlGvMm2" )]  + errs ) );
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
	let slice_args = {
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
		let mut r_doc = ~HashMap::new();
		r_doc.insert( ~"attach", String(~"doc").to_json() );
		r_doc.insert( ~"spec_key", String( Bootstrap::spec_append_slice_key() ).to_json() );
		let bw = @BytesWriter::new();
		let mut encoder = PrettyEncoder( bw as @Writer );
		args.to_json().encode( &mut encoder );				
		bw.flush();						
		let rval = {
			match { let ( p, c ) = oneshot();
				fit_chan.send( DoFit( ~FitArgs::from_doc_with_attach( r_doc, copy *bw.bytes ), ~Must::new(), c ) );
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
		assert!( JahSpec::check_args( &Bootstrap::find_spec( Bootstrap::spec_file_slice_key()), &rval.doc ).is_ok() );
		let len: uint = rval.doc.get_float( ~"len" ).get().to_uint();
		assert!( len == args.to_pretty_str().len() );
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
			fit_chan.send( DoFit( copy slice_args, ~Must::new(), c ) );
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
	assert!( JahSpec::check_args( &Bootstrap::find_spec( Bootstrap::spec_find_slice_result_key() ), &rval.doc ).is_ok() );
	assert!( rval.doc.get_str( ~"attach" ).get() == ~"file_slice" );
	assert!( rval.attach.len() == slice_args.doc.get_float(~"len").get().to_uint() );
	
	match extra::json::from_str( rval.attach.to_str() ).get() {
		Object( val ) => {
			assert!( JahSpec::check_args( &Bootstrap::find_spec( Bootstrap::spec_stored_doc_key() ), &val ).is_ok() );
		} _ => {}
	}
}
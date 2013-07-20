//	Copyright 2013 Jeff Murray.

//	Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//	http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//	<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//	option. This file may not be copied, modified, or distributed
//	except according to those terms.

#[link(name = "doc_slice_prep", vers = "1.0")];

//	rustc --lib fits/doc_slice_prep.rs -L .
//	rustc fits/doc_slice_prep.rs --test -o fits/doc_slice_prep-tests -L . -L fits
//	./fits/doc_slice_prep-tests

extern mod std;
extern mod extra;
extern mod fit;
extern mod bootstrap;
extern mod jah_spec;
extern mod jah_args;
extern mod must;
extern mod par;
use std::io::{ BytesWriter, Writer };
use extra::serialize::Encodable;
use std::str::{ from_bytes_owned };
use par::{ Par, ParTrans, ParCommEndChan }; // Used in unit tests
use std::comm::{ stream, Port, Chan, ChanOne, SharedChan, oneshot }; // oneshot and recv_one are used in unit tests
use extra::json::{ Object, ToJson, String, PrettyEncoder }; //, to_pretty_str, Json
use bootstrap::{ Bootstrap };
//use extra::serialize::Encodable;
use std::hashmap::HashMap;
use fit::{ Parfitable, ParFitComm, DoFit, ParFitCommEndChan, FitOk, FitErr, FitArgs, FitErrs, FitSysErr }; // FitSysErr is used in unit test
use jah_spec::{ JahSpeced, JahSpec }; 
use jah_args::{ JahArgs };
use must::{ Must }; // used in unit tests

//	DocSlicePrep receives a document through ParFitComm and preps it for adding or editing


pub struct DocSlicePrep {
	settings: ~Object
}
	
impl Parfitable for DocSlicePrep {

	pub fn new( settings: ~Object ) -> ~DocSlicePrep {
	
		~DocSlicePrep { settings: settings }
	}
	
	pub fn connect( &self ) -> Result<Chan<ParFitComm>, ~FitErrs> {
	
		let ( in_port, in_chan ) = stream();
		match self.go( in_port ) {
			Ok( _ ) => { Ok( in_chan ) }
			Err( errs ) => { Err( errs ) }
		}
	}
	
	pub fn fit_key( &self ) -> ~str {
	
		~"OKfLlCA6dQbw9tD8"
	}
}

impl JahSpeced for DocSlicePrep {
	
	fn spec_keys_in( &self ) -> ~[~str] {
	
		~[Bootstrap::spec_add_doc_key(), Bootstrap::spec_edit_doc_key()]
	}
	
	fn spec_keys_out( &self ) -> ~[~str] {
	
		~[Bootstrap::spec_file_slice_key()]
	}
}

impl DocSlicePrep {

	priv fn spawn_and_read( home: SharedChan<ChanOne<ParFitComm>>) {
	
		do spawn {
			let ( p, c ) = oneshot();
			home.send( c );
			let parfit_comm : ParFitComm =  p.recv();
			match parfit_comm {
				DoFit( fit_args, home_chan ) => {
					let mut doc = fit_args.doc;
					//we are guaranteed here to have the first layer of properties checked
					//so I am calling get()
					let new_must = {
						match doc.get_str(~"spec_key").get() {
							~"uHSQ7daYUXqUUPSo" => { //add
								Ok( Must::new() )
							}
							~"CJeCZR6b9t6jj46S" => { //edit
								//Only the first layer of properties are checked by the transcriptor at the moment
								//so I am checking the must spec
								match JahSpec::check_args( &Bootstrap::find_spec( Bootstrap::spec_must_key() ), &doc ) {
									Err( errs ) => {
										Err( FitErrs::from_objects( errs ) )
									}
									Ok( _ ) => {
										doc.remove(&~"must");
										//With a checked spec...
										Ok( Must::stamped( doc.get_str( ~"key" ).get() ) )								
									}
								}
							}
							_ => {
								Err( FitErrs::from_object( Bootstrap::logic_error(Bootstrap::arg_spec_key_not_known_to_fit(), doc.get_str(~"spec_key").get(), ~"aZWkaywgi34NMiDk", ~"doc_slice_prep.rs") ) )
							}
						}};
					match new_must {
						Ok( must ) => {
							doc.insert(~"must", must.to_json() );
							doc.remove( &~"spec_key" );
							doc.insert( ~"spec_key", String( Bootstrap::spec_stored_doc_key() ).to_json() );
							let mut r_doc = ~HashMap::new();
							r_doc.insert( ~"attach", String(~"doc").to_json() );
							r_doc.insert( ~"spec_key", String( Bootstrap::spec_append_slice_key() ).to_json() );
							let bw = @BytesWriter::new();
							let mut encoder = PrettyEncoder( bw as @Writer );
							doc.to_json().encode( &mut encoder );				
							bw.flush();			 		
							home_chan.send( FitOk( ~FitArgs{ doc: r_doc, attach: copy *bw.bytes } ) );							
						}
						Err( errs ) => {
							home_chan.send( FitErr( errs ) );
						}

					}
				} ParFitCommEndChan => {} // the calling function does not send ParFitCommEndChan and this is a private function
			}
		}		
	}

	priv fn go ( &self, in_port: Port<ParFitComm> ) -> Result<bool, ~FitErrs> {

    	//let fit_key = self.fit_key();
		do spawn {	
			loop {
				let ( sp, sc ) = stream();
				let sc = SharedChan::new( sc );
				let parfit_comm = in_port.recv();
				match parfit_comm {
					ParFitCommEndChan => {
						break;
					},
					_ => {
						DocSlicePrep::spawn_and_read( sc.clone() );
						sp.recv().send( parfit_comm );
		  			}
				}	
			}
		}
		Ok( true )
	}
}

#[test]
fn append_doc() {
	
	
	let par_chan = {
		let par = ~Par::new( 20u );
		let doc_slice_prep = ~DocSlicePrep{ settings: ~HashMap::new() };
		match doc_slice_prep.connect() {
			Ok( prep_conn ) => {
				match par.connect( prep_conn ) {
					Ok( par_chan ) => {
						par_chan
					}
					_ => { fail!(); }
				}
			}
			_ => { fail!(); }
		}};
	{	//Wrong spec key
		let mut doc = ~HashMap::new();
		doc.insert( ~"message",String( ~"hello from DocSlicePrep" ) );
		let mut args = ~HashMap::new();
		args.insert( ~"user", String( ~"va4wUFbMV78R1AfB" ) );
		args.insert( ~"acct", String( ~"ofWU4ApC809sgbHJ" ) );
		args.insert( ~"must", Must::new().to_json() );	
		args.insert( ~"doc", doc.to_json() );
		args.insert( ~"spec_key", String( Bootstrap::spec_jah_spec_corrupt_key() ).to_json() );
		let ( p, c ) = oneshot();
		//check the wrong spec_key
		par_chan.send( ParTrans( ~FitArgs::from_doc( copy args ), c ) );
		
		let outcome = p.recv();
		
		match outcome.outcome {
			FitOk( _ ) => { fail!(); } 
			FitErr( _ ) => {}
			FitSysErr( _ ) => { fail!(); }
		}
	}
	{	//Add doc
		let mut doc = ~HashMap::new();
		doc.insert( ~"message",String( ~"hello from DocSlicePrep" ) );
		let mut args = ~HashMap::new();
		args.insert( ~"user", String( ~"va4wUFbMV78R1AfB" ) );
		args.insert( ~"acct", String( ~"ofWU4ApC809sgbHJ" ) );
		//args.insert( ~"must", Must::new().to_json() );	
		args.insert( ~"doc", doc.to_json() );
		args.insert( ~"spec_key", String( Bootstrap::spec_add_doc_key() ).to_json() );
		let ( p, c ) = oneshot();
		par_chan.send( ParTrans( ~FitArgs::from_doc( copy args ), c ) );
		let outcome = p.recv();
		match outcome.outcome {
			FitOk( fit_args ) => {
				match extra::json::from_str( from_bytes_owned( fit_args.attach ) ) {
					Ok( val ) => {
						match val {
							Object( doc ) => {
								match JahSpec::check_args( &Bootstrap::find_spec( Bootstrap::spec_stored_doc_key() ), &doc ) {
									Err( errs ) => {
										for errs.iter().advance | err | {
											println( err.to_pretty_str() );
										}
										
										fail!();
									}
									Ok( _ ) => {}
								}
							}
							_ => { fail!(); }	
						}
					}
					Err( err ) => {
						println( copy *err.msg );
					}
				}
			}  
			FitErr( _ ) => {fail!();}
			FitSysErr( _ ) => { fail!(); }
		}	
	}	
		
	let ( p, c ) = oneshot();
	par_chan.send( ParCommEndChan( c ) );
	p.recv();
}


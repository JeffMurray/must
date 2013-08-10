//	Copyright 2013 Jeff Murray.

//	Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//	http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//	<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//	option. This file may not be copied, modified, or distributed
//	except according to those terms.

#[link(name = "stately_tester", vers = "0.0")];

//	rustc --lib fits/stately_tester.rs -L .

extern mod std;
extern mod extra;
extern mod fit;
extern mod jah_spec;
extern mod jah_args;
extern mod stately;
extern mod must;
extern mod bootstrap;
use bootstrap::{ Bootstrap };
use must::{ Must };
use stately::{ Comeback, LostToErr, LoopOutComm, ComebackIfOk};
use std::comm::{ stream, Port, Chan };  //, SharedChan
use extra::json::{ Object, ToJson, String };//,Number,List, PrettyEncoder
use std::hashmap::HashMap;
use fit::{ Parfitable, ParFitComm, DoFit, ParFitCommEndChan, FitOk, FitArgs, FitErrs}; 
use jah_spec::{ JahSpeced }; 
use jah_args::{ JahArgs };
use std::comm::{ oneshot, SharedChan };

struct StatelyTester {
	stately_chan: SharedChan<LoopOutComm>
}
	
impl Parfitable for StatelyTester {

	pub fn connect( &self ) -> Result<Chan<ParFitComm>, ~FitErrs> {
	
		let ( in_port, in_chan ) = stream();
		self.go( in_port );
		Ok( in_chan )
	}
	
	pub fn fit_key( &self ) -> ~str {
	
		~"AL9SJolxGQyklS4A"
	}
}

impl JahSpeced for StatelyTester {
	
	fn spec_keys_in( &self ) -> ~[~str] {
	
		~[~"qJzMxt6eQV2CgKbr"]
	}
	
	fn spec_keys_out( &self ) ->  ~[~str] {
	
		~[~"er5OWig71VG9oNjK"]
	}
}

impl StatelyTester {

	//The Par always call go() using spawn because go does not exit  
	//unless the channel sends a request with is_stop_order set to true

	priv fn go ( &self, par_port: Port<ParFitComm> ) {

		let stately_chan = self.stately_chan.clone();
		do spawn {	
			loop {
				match par_port.recv() {
		  			DoFit( _, t_key, home_chan ) => {
		  				//does the same basic test regardless of the incoming args
		  				let state_ok = Must::new();		
		  				let ( p1, c1 ) = oneshot();
		  				stately_chan.send( ComebackIfOk( ~FitArgs::from_doc( state_ok.to_obj() ), ~FitArgs::from_doc( Bootstrap::reply_error_trace_info( ~"stately_tester.rs", ~"ChhEUlXgiJKHDzI0" ) ), ~"DROOg7Vt2GXiVl00", *copy t_key, c1 ) );  // ( state, args, strand_key, t_key, comeback_chan )
		  				match p1.try_recv().expect( "stately_tester.rs Nvq6D43Qq8JCG4hS" ) {
  							Comeback( state, args ) => { // ( state, args )
  								match Must::from_obj( &state.doc ) {
  									Ok( st ) => {
  										assert!( st == state_ok );
  									}
  									Err( _ ) => {
  										fail!();
  									}
  								}
  								match args.doc.get_str( ~"spec_key" ) {
  									Ok( key ) => {
  										assert!( key == ~"er5OWig71VG9oNjK" );  // this is the spec_key that the ErrFit returns.										
  									}
  									Err( _ ) => {
  										fail!();
  									}
  								}
  							}
							LostToErr( _ ) => {
								fail!();
							}
		  				}
		  				println( "sending Error in stately_tester.rs as a test for LostToErr" );	  				
						let mut args = Bootstrap::reply_error_trace_info( ~"stately_tester.rs", ~"LABaRJT4NwQka5BN" );
						let state_err = Must::new();
		  				args.insert( ~"dude", String( ~"dude" ).to_json() ); // sending an argument that is not in the spec
						let ( p2, c2 ) = oneshot();
		  				stately_chan.send( ComebackIfOk( ~FitArgs::from_doc( state_err.to_obj() ), ~FitArgs::from_doc( args ), ~"DROOg7Vt2GXiVl00", *copy t_key, c2 ) ); // ( state, args, strand_key, t_key, comeback_chan )
		  				match p2.try_recv().expect( "stately_tester.rs JQCobR337OOhVwzi" ) {
  							Comeback( _, _ ) => { // ( state, args )
  								fail!();
  							}
							LostToErr( state ) => {
  								match Must::from_obj( &state.doc ) {
  									Ok( st ) => {
  										assert!( st == state_err );
  									}
  									Err( _ ) => {
  										fail!();
  									}
  								}							
							}
		  				}
		  				let state_add_ok = Must::new();	
						let mut doc = ~HashMap::new();
						doc.insert( ~"message",String( ~"testing add document using stately." ) );
						let mut args = ~HashMap::new();
						args.insert( ~"user", String( ~"va4wUFbMV78R1AfB" ) );
						args.insert( ~"acct", String( ~"ofWU4ApC809sgbHJ" ) );
						//args.insert( ~"must", Must::new().to_json() );	
						args.insert( ~"doc", doc.to_json() );
						args.insert( ~"spec_key", String( Bootstrap::spec_add_doc_key() ).to_json() );
						let ( p3, c3 ) = oneshot();
						stately_chan.send( ComebackIfOk( ~FitArgs::from_doc( state_add_ok.to_obj() ), ~FitArgs::from_doc( args ), Bootstrap::must_start_strand(), *copy t_key, c3 ) ); // ( state, args, strand_key, t_key, comeback_chan )
						match p3.try_recv().expect( "stately_tester.rs OScAaqxLEtRouSzj" ) {
  							Comeback( state, args ) => { // ( state, args )
  								match Must::from_obj( &state.doc ) {
  									Ok( st ) => {
  										assert!( st == state_add_ok );
  									}
  									Err( _ ) => {
  										fail!();
  									}
  								}
  								match args.doc.get_str( ~"spec_key" ) {
  									Ok( key ) => {
  										assert!( key == Bootstrap::spec_file_slice_key() ); 										
  									}
  									Err( _ ) => {
  										fail!();
  									}
  								}
  							}
							LostToErr( _ ) => {
								//println( state.doc.to_str() );
								fail!();
							}
		  				}		
		  				let mut r_args = ~HashMap::new();
		  				r_args.insert(  ~"spec_key", String(~"er5OWig71VG9oNjK").to_json() );
		 				home_chan.send( FitOk( ~FitArgs::from_doc( copy r_args ) ) );
		  			}
					ParFitCommEndChan => {
						break;
					}
				}
			}
		}
	}
}
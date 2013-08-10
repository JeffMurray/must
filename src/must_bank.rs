//	Copyright 2013 Jeff Murray.

//	Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//	http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//	<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//	option. This file may not be copied, modified, or distributed
//	except according to those terms.
 
 #[link(name = "must_bank", vers = "0.0")];  
 
//	rustc --lib must_bank.rs -L . -L fits
//	rustc must_bank.rs --test -o must_bank-tests -L . -L fits
//	./must_bank-tests

extern mod std;
extern mod extra;
extern mod fit;
extern mod fits;
extern mod bootstrap;
extern mod must;
extern mod parts;
extern mod transcriptor;
extern mod stately;
//extern mod jah_args;
//use jah_args::{ JahArgs };
use stately::{ StateServ, LoopOutComm, StatelyRelease };
use fits::{ Fits };
use transcriptor::{ Transcriptor };
use fit::{ FitErrs, FitArgs };
use parts::{ ParTs, ParTInComm, ParTInAdminComm, AddParT, ParTsRelease };
use must::{ Must };
use extra::json::{ Object, String, ToJson }; 
use bootstrap::{ Bootstrap };
use std::comm::{ oneshot, ChanOne, stream, SharedChan };
use std::hashmap::HashMap;
use std::task::{ spawn, yield };
use std::io::{ stdin, ReaderUtil };

struct MustBank;

enum MustBankInComm {
	MBTranscript( ~str, ~Object, ChanOne<Result< ~Object, ~FitErrs >> ), // strand_key, args, Result( trans_key info ) or Err info 
	MBRelease( ChanOne<()> ) //( ack_chan )
}

impl MustBank {
	
	pub fn connect() -> Result<Chan<MustBankInComm>, ~FitErrs> {
	
		let (port, chan ) = stream();
		MustBank::loop_in_spawn( port );
		Ok( chan )
	}
	
	priv fn loop_in_spawn( port: Port< MustBankInComm > )  {
	
		do spawn {
		
			let ( parts_chan, parts_admin_chan ) = {
				match ParTs::connect() {
					Ok(( parts_chan, parts_admin_chan )) => {
						( parts_chan, parts_admin_chan )
					}
					Err( err ) => {
						// I have not written the rs that this would report to
						println( err.prepend_err( Bootstrap::reply_error_trace_info(~"must_bank.rs", ~"ipHe17fctVPegreA") ).to_str() );
						fail!();
					}
				}};
			let stately_chan = SharedChan::new( StateServ::connect( parts_chan.clone() ) );
			match MustBank::load_parts( stately_chan.clone(), parts_admin_chan.clone() ) {  
				Ok( _ ) => {
					let ( goodby_port, goodby_chan ) = stream();
					let goodby_chan = SharedChan::new( goodby_chan );
					let mut t_count = 0u;
					loop {
						let ( recv_trans, recv_goodby ) = {
							if t_count == 0u {
								( true, false )
							} else {
								( port.peek(), goodby_port.peek() )
							}};
						if recv_trans {
							match port.try_recv().expect( "must_bank.rs NW8YXw4Pkxl9WtKk" ) {
								MBTranscript( strand_key, args, chan_one ) => {
									t_count += 1;
									let t_key = Must::new();
									Transcriptor::connect( strand_key, copy t_key ).send( ( ~FitArgs::from_doc( args ), parts_chan.clone(), goodby_chan.clone() ) );  // ( strand_key, t_key )  the kickoff strand for new requests
									chan_one.send( Ok( t_key.to_obj() ) );
								}
								MBRelease( ack_chan ) => {
									println( "checking t_count" );
									while t_count > 0 {
										goodby_port.try_recv().expect( "must_bank.rs 5twBBIomSNA5JFeN" );
										t_count -= 1;
									}
									println( "releasing stately" );
									stately_chan.send( StatelyRelease );
									
									println( "releasing parts" );
									let ( p, c ) = oneshot();
									parts_admin_chan.send( ParTsRelease( c ) );
									p.try_recv().expect( "must_bank.rs ifYVoW38bDAYRDeZ" ); // waiting for Ack
									ack_chan.send(());
									break;
								}
							}
						}
						if recv_goodby {
							match goodby_port.try_recv().expect( "must_bank.rs Jen1sH2LNXOjxnfd" ) {	//the submiter already has their key and is long gone
								Ok( _ ) => {}
								Err( fit_errs ) => {
									println( fit_errs.to_str() );
								}
							}
							t_count -= 1;
						}
						if !( recv_trans || recv_goodby ) {
							yield();
						}
					}					
				}
				Err( fit_errs ) => {
					println( fit_errs.to_str() ); fail!();
				}
			}			
		}		
	}

	priv fn load_parts( stately_chan: SharedChan<LoopOutComm>, parts_admin_chan: SharedChan<ParTInAdminComm> ) -> Result<bool, ~FitErrs> {

		match MustBank::load_par( Bootstrap::err_fit_key(), 20u, stately_chan.clone(), parts_admin_chan.clone()  ) {
			Ok( _ ) => {}
			Err( errs ) => {
				return Err( errs );
			}
		}
		match MustBank::load_par( Bootstrap::doc_slice_prep_key(), 20u, stately_chan.clone(), parts_admin_chan.clone()  ) {
			Ok( _ ) => {}
			Err( errs ) => {
				return Err( errs );
			}
		}
		match MustBank::load_par( Bootstrap::file_append_slice_key(), 20u, stately_chan.clone(), parts_admin_chan.clone()  ) {
			Ok( _ ) => {}
			Err( errs ) => {
				return Err( errs );
			}
		}
		match MustBank::load_par( Bootstrap::file_get_slice_key(), 20u, stately_chan.clone(), parts_admin_chan.clone() ) {
			Ok( _ ) => {}
			Err( errs ) => {
				return Err( errs );
			}
		}
		match MustBank::load_par( Bootstrap::stately_tester_key(), 20u, stately_chan.clone(), parts_admin_chan.clone()  ) {
			Ok( _ ) => {}
			Err( errs ) => {
				return Err( errs );
			}
		}		
		Ok( true )	
	}
	
	priv fn load_par( reg_key: ~str, spawn_cap: uint, stately_chan: SharedChan<LoopOutComm>, admin_chan: SharedChan<ParTInAdminComm> ) -> Result<bool, ~FitErrs> {
	
		let fit_chan = {
			match Fits::make_fit( copy reg_key, stately_chan ) {
				Ok( fit_chan ) => {
					fit_chan
				}
				Err( errs ) => {
					return Err( errs );	
				}
			}};
		let ( p, c ) = oneshot();
		admin_chan.send( AddParT( copy reg_key, spawn_cap, fit_chan, c ) );
		match p.try_recv().expect( "must_bank.rs k2LXIJG5s0NMN2zA" ) {
			Ok( _ ) => {}
			Err( errs ) => {
				return Err( errs );
			}
		}
		Ok( true )
	}
}


#[test]
fn add_document_strand() {

	let must_bank_in = {
		match MustBank::connect() {
			Ok( mbi ) => {
				mbi
			}
			Err( _ ) => {
				fail!();
			}
		}};
	let must_bank_in = SharedChan::new( must_bank_in );
	let max = 1000i;
	let mut i = 1i;
	std::io::println( "Inserting " + max.to_str() + " documents." );
	let( port, chan ) = stream();
	let chan = SharedChan::new( chan );
	while i <= max {
		let mbs = must_bank_in.clone();
		let started = chan.clone();
		let count = i;
		do spawn {
			let mut doc = ~HashMap::new();
			doc.insert( ~"message",String( ~"must_bank " + count.to_str() + " reporting for duty." ) );
			let mut args = ~HashMap::new();
			args.insert( ~"user", String( ~"va4wUFbMV78R1AfB" ) );
			args.insert( ~"acct", String( ~"ofWU4ApC809sgbHJ" ) );
			//args.insert( ~"must", Must::new().to_json() );	
			args.insert( ~"doc", doc.to_json() );
			args.insert( ~"spec_key", String( Bootstrap::spec_add_doc_key() ).to_json() );
			let ( p, c ) = oneshot();
			mbs.send( MBTranscript( Bootstrap::must_start_strand(), args, c ) );
			match p.try_recv().expect( "must_bank.rs MMzO6ygIScG1IhxX" ) {
				Ok( _ ) => { // immediatly returns a t_key that can be used to get the doc key and so forth
					//std::io::println( extra::json::to_pretty_str(&(t_key.to_json())));
				}
				Err( err ) => { std::io::println( err.to_str() ); fail!(); }
			}
			started.send(());
		}
		i += 1;
	}
	println( "documents submitted" );
	
	//We need to confirm that all the transcriptors get started before calling MBRelease
	i = 1;
	while i <= max {
		port.try_recv().expect( "must_bank.rs y1UG1rNTqZs3cSBi" );
		i += 1;
	}

	let mut doc = ~HashMap::new();
	doc.insert( ~"message",String( ~"must_bank error reporting for duty." ) );
	let mut args = ~HashMap::new();
	args.insert( ~"user", String( ~"va4wUFbMV78R1AfB" ) );
	args.insert( ~"acct", String( ~"ofWU4ApC809sgbHJ" ) );
	//args.insert( ~"must", Must::new().to_json() );	
	args.insert( ~"doc", doc.to_json() );
	args.insert( ~"spec_key", String( Bootstrap::spec_add_doc_key() ).to_json() );
	{
		let ( p, c ) = oneshot();
		must_bank_in.clone().send( MBTranscript( Bootstrap::must_start_strand(), copy args, c ) );
		match p.try_recv().expect( "must_bank.rs fRKSRL9kKz5NFjxf" ) {
			Ok( _ ) => { // immediatly returns a t_key that can be used (once indexes are up and running) to get the error and so forth
				//std::io::println( extra::json::to_pretty_str(&(t_key.to_json())));
			}
			Err( err ) => {std::io::println( err.to_str() ); fail!(); }
		}
	}
	{
		let ( p, c ) = oneshot();
		must_bank_in.clone().send( MBTranscript( Bootstrap::stately_tester_strand(), args, c ) );
		match p.try_recv().expect( "must_bank.rs PPtdwnL2Cjk0jYV9" ) {
			Ok( _ ) => { 
				//std::io::println( extra::json::to_pretty_str(&(t_key.to_json())));
			}
			Err( err ) => {std::io::println( err.to_str() ); fail!(); }
		}	
	}
	{				
		let ( p, c ) = oneshot();
	    must_bank_in.clone().send( MBRelease( c ) );
	    std::io::println( "waiting for ack" );
		p.try_recv().expect( "must_bank.rs 67PBqYz6JO2HnV6U" );  // wait for the ack
		std::io::println( "reminder: check and delete test.json" );
	}
}


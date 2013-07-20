//	Copyright 2013 Jeff Murray.

//	Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//	http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//	<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//	option. This file may not be copied, modified, or distributed
//	except according to those terms.
 
 #[link(name = "must_bank", vers = "1.0")];  
 
//	rustc --lib must_bank.rs -L . -L fits
//	rustc must_bank.rs --test -o must_bank-tests -L . -L fits
//	./must_bank-tests

extern mod std;
extern mod extra;
extern mod jah_mut;
extern mod jah_spec;
extern mod jah_args;
extern mod par;
extern mod fit;
extern mod bootstrap;
extern mod strand;
extern mod must;
extern mod parts;
use jah_args::{ JahArgs, MissingKey, WrongDataType };
use jah_spec::{ JahSpec };
use par::{ FitOutcome, ParTrans };
use fit::{ FitOk, FitErr, FitSysErr, FitErrs, FitArgs };
use parts::{ ParTs, ParTInComm, ParTInAdminComm, AddParT,  GetParTChan, ParTChan, ParTErr, ParTsRelease };
use must::{ Must };
use strand::{ Ribosome, DoFit, NextErr, NextOk, EndOfStrand, LogicErr };
use jah_mut::{ JahMutReq, GetStr, GetJson, JahMut, LoadMap, MergeArgs, Release, GetAttach };
use extra::json::{ Object, String, ToJson }; // , to_pretty_str
use bootstrap::{ Bootstrap };
use std::comm::{ oneshot, recv_one, ChanOne, stream, SharedChan };
use std::hashmap::HashMap;
use std::task::{ spawn, yield };

struct MustBank;

enum MustBankInComm {
	MBTranscript( ~Object, ChanOne<Result< ~Object, ~FitErrs >> ), // args, Result( trans_key info ) or Err info 
	MBRelease( ChanOne<()> ) //( ack_chan )
}

struct Transcriptor;

impl MustBank {
	
	pub fn connect() -> Result<Chan<MustBankInComm>, ~FitErrs> {
	
		let (port, chan ) = stream();
		MustBank::loop_in_spawn( port );
		Ok( chan )
	}
	
	priv fn loop_in_spawn( port: Port< MustBankInComm > ) {
	
		do spawn {
			match MustBank::load_parts() {  //leaving the warning to remind me to tidy this up
				Ok(( user_parts_chan, admin_parts_chan )) => {
					let user_parts_chan = SharedChan::new( user_parts_chan );
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
							match port.recv() {
								MBTranscript( args, chan_one ) => {
									t_count += 1;
									Transcriptor::connect( ~"UWmoVWUMfKsL8oyr").send( ( args, chan_one, user_parts_chan.clone(), goodby_chan.clone() ) );  // ( strand_key )  the kickoff strand for new requests
								}
								MBRelease( ack_chan ) => {
									while t_count > 0 { //TODO: put a timeout here?
										goodby_port.recv();
										t_count -= 1;
									}
									let ( p, c ) = oneshot();
									admin_parts_chan.send( ParTsRelease( c ) );
									recv_one( p );
									ack_chan.send(());
									break;
								}
							}
						}
						if recv_goodby {
							goodby_port.recv();
							t_count -= 1;
						}
						if !( recv_trans || recv_goodby ) {
							yield();
						}
					}					
				}
				Err( _ ) => {
					fail!(); //yuck here for now :( I'm guessing this is rare but leaving warning to keep the issue open in my mind
				}
			}			
		}		
	}

	priv fn load_parts() -> Result<( Chan<ParTInComm>, Chan<ParTInAdminComm> ), ~FitErrs> {
	
		let ( user_chan, admin_chan ) = {
			match ParTs::connect() {
				Ok(( user_chan, admin_chan )) => {
					( user_chan, admin_chan )
				}
				Err( err ) => {
					return Err( err.prepend_err( Bootstrap::reply_error_trace_info(~"must_bank.rs", ~"ipHe17fctVPegreA") ) );
				}
			}};
		match {	let ( p, c ) = oneshot();
				admin_chan.send( AddParT( Bootstrap::file_append_slice_key(), c ) );  //FileAppendSlice
				recv_one( p )
			} {
				Ok( _ ) => {}
				Err( _ ) => { fail!(); }
		}
		match {	let ( p, c ) = oneshot();
				admin_chan.send( AddParT( Bootstrap::err_fit_key(), c ) );  // ErrFit
				recv_one( p )
			} {
				Ok( _ ) => {}
				Err( _ ) => { fail!(); }
		}
		match {	let ( p, c ) = oneshot();
				admin_chan.send( AddParT( Bootstrap::doc_slice_prep_key(), c ) );  // DocSlicePrep
				recv_one( p )
			} {
				Ok( _ ) => {}
				Err( _ ) => { fail!(); }
		}
		match {	let ( p, c ) = oneshot();
				admin_chan.send( AddParT(  Bootstrap::file_get_slice_key(), c ) );  // FileGetSlice
				recv_one( p )
			} {
				Ok( _ ) => {}
				Err( _ ) => { fail!(); }
		}
		
		Ok(( user_chan, admin_chan ))
	}
}

impl Transcriptor {

	fn connect( kickoff_strand_key: ~str ) -> Chan<((~Object, ChanOne<Result< ~Object, ~FitErrs >>, SharedChan<ParTInComm>, SharedChan<int>))> {
	
		let ( start_port, start_chan ) = stream();
		
		do spawn {
			let kickoff_strand_key = copy kickoff_strand_key;	
			let ( args, home_chan_one, parts_chan, goodby_chan ): (~Object, ChanOne<Result< ~Object, ~FitErrs >>, SharedChan<ParTInComm>, SharedChan<int>) = start_port.recv();
			let t_key = Must::new();
			home_chan_one.send(  Ok( Transcriptor::make_t_key( copy t_key ) ) );
			let ( arg_bank_user_chan, arg_bank_admin_chan ) = JahMut::connect();  //  <-- the arg bank
			let arg_bank_sh_chan = SharedChan::new( arg_bank_user_chan );
			arg_bank_admin_chan.send( LoadMap( copy args ) );
			let ( rib_port, rib_chan ) = Ribosome::connect( kickoff_strand_key, arg_bank_sh_chan.clone() );
			loop {
				let reg_key = { 
					match rib_port.recv() {
						DoFit( key ) => { key }
						LogicErr( err ) => { std::io::println( extra::json::to_pretty_str(&(err.to_json())));break; } //  <- temp band-aid, pure logic errors should be pretty rare 
						EndOfStrand	=> { break; }
					}};
				let spec_key = { //get the latest spec that was loaded in the arg bank
					match { let ( p, c ) = oneshot();
						arg_bank_sh_chan.clone().send( GetStr( ~"spec_key", c ) );
						recv_one( p ) }
					{ 	Some( spec_key ) => { spec_key }
						None => { std::io::println( "no spec key found in must_bank.rs" ); break; }
					}};
				let args = { 
					match Transcriptor::speced_arg_excerpt( &Bootstrap::find_spec( spec_key ), arg_bank_sh_chan.clone() ) {
						Ok( args ) => { args }													  //Leaving warning for now	
						Err( _ ) => { std::io::println( "speced_arg_excerpt error" ); break; }  //Reporting this error will require the indexes be up and running
					}};																			  //Transcribing this can get tied in with the rest of the reporting
				// get the Par chan and send args
				let fo: FitOutcome = {
					match { let ( p, c ) = oneshot();
						parts_chan.send( GetParTChan( reg_key, c ) ); // ChanOne<ParTOutComm>
						recv_one( p )
						} {	ParTChan( part_chan ) => { // ( part_chan ) ChanOne<ParInComm>
								let ( p2, c2 ) = oneshot();
								part_chan.send( ParTrans( args, c2 ) ); // ChanOne<ParTOutComm>
								recv_one( p2 )
							} 
							ParTErr( err ) => { std::io::println( err.to_str() ); break; }
					}};
				// Record the fit performance once the indexing system is up and running
				// update the arg_bank
				
				match copy fo.outcome {
					FitOk( rval ) => {
						match rval.doc.get_str( ~"spec_key" ) {
							Ok( key ) => {
								match JahSpec::check_args( &Bootstrap::find_spec( key ), &rval.doc ) {
									Ok( _ ) => {
										arg_bank_admin_chan.send( MergeArgs( rval ) );
										rib_chan.send( NextOk );					
									}
									Err( errs ) => {
										let fit_errs = FitErrs::from_objects( errs);
										//println( to_pretty_str( &Object( copy doc ).to_json() ) );
										arg_bank_admin_chan.send( MergeArgs( ~FitArgs::from_doc( fit_errs.to_args() ) ) );
										rib_chan.send( NextErr );									
									}
								}
							}
							Err( err_type ) => {
								let errs = {
									match err_type {
										MissingKey => {
											FitErrs::from_object( Bootstrap::logic_error(Bootstrap::arg_spec_key_arg_must_exist(), ~"spec_key", ~"TWRUF69B4hv4v5Iz", ~"must_bank.rs" ) )
										}
										WrongDataType => {
											FitErrs::from_object( Bootstrap::logic_error(Bootstrap::arg_rule_arg_must_be_string_key(), ~"spec_key", ~"iwpCbbmXqKyvc9VL", ~"must_bank.rs" ) )
										}
									}};
								arg_bank_admin_chan.send( MergeArgs( ~FitArgs::from_doc( errs.to_args() ) ) );
								rib_chan.send( NextErr );														
							}
						}
					}
					FitErr( rval ) => {
						let doc = rval.to_args();
						//println( to_pretty_str( &Object( copy doc ).to_json() ) );
						arg_bank_admin_chan.send( MergeArgs( ~FitArgs::from_doc( doc ) ) );
						rib_chan.send( NextErr );
					}
					FitSysErr( err ) => {
						println( err.to_str() );
						break;
					}
				}
			}
			arg_bank_admin_chan.send( Release );
			goodby_chan.send(1i);	
		}
		start_chan
	}
		
	//  Maybe I should bring up an issue I am pondering at the moment.
	//  What I like about the arg_bank is that when a reasonably large (1.4MB max) 
	//	document first comes in, it can be loaded into the arg_bank and all the
	//	Fits that validate credentials and so forth will not have the big stuff
	//  sent to them unless their spec calls for it.  Then in the end, when every 
	//	thing is validated, the big stuff can be sent on a channel and saved.  If 
	//  a Fit, in-between the user submission an the final save, needs to examine 
	//  the big stuff, then it can be included in the spec. That system puts a lot 
	//  of power and responsibility in the hands of a Logic Strand script writers. 
	
	//  In Must, the current design, which was chosen partly because it is easy to implement,
	//	takes the the incoming args, loads them, and calls the Fit, and when that
	//  Fit returns args, those args get loaded into the arg bank, overwriting
	//  any args with that share names, such as spec_key.    
  
	//  speced_arg_excerpt takes the expected spec of the upcoming Fit and attempts to extract 
	//  those args, by key, from the arg bank.  It then checks those extracted args for 
	//  adherence to the expected spec. If everything passes, Ok( args ) is returned, 
	//  otherwise descriptive error messages are returned, according to spec ;).
	
	priv fn speced_arg_excerpt( spec: &~Object, arg_bank_chan: SharedChan<JahMutReq> )-> Result<~FitArgs, ~FitErrs> {
		
		let mut rval = ~HashMap::new();
		let keys = { 
			match JahSpec::allowed_keys( spec ) {
				Ok( keys ) => { keys } 
				Err( err ) => { return Err( FitErrs::from_objects( err ) ) }
				}};
		for keys.iter().advance | key | {
				match {let ( p, c ) = oneshot();
				arg_bank_chan.send( GetJson( copy *key, c ) );  // not sure all this back and forth is worth it
				recv_one( p )
			}
			{	Some( arg_val ) => { 
					rval.insert( copy *key, arg_val ); // <-- a little bit lost?  Maybe iter() belongs on jah_mut to reduce communication overhead?
				}
				None => {}  // leaving the ramifications of this missing arg to the spec check	
			}
		}
		match JahSpec::check_args( spec, &rval  ) {
			Ok( _ ) => {
				match rval.get_str(~"attach") {
					Ok( attached_name ) => {
						let ( p, c ) = oneshot();
						arg_bank_chan.send( GetAttach( copy attached_name, c ) );
						match recv_one( p ) {
							Some( attached_bytes ) => {
								Ok( ~FitArgs{ doc: rval, attach: attached_bytes } ) 
							}
							None => {
								Err( FitErrs::from_object( Bootstrap::logic_error( Bootstrap::named_attachment_is_missing(), copy attached_name, ~"Kyzltdf11TRcTIiI", ~"must_bank.rs" ) ) )
							}
						}
					}
					Err( _ ) => { //  not really an error, just no need to send an attachment
						Ok( ~FitArgs::from_doc( rval ) )
					}
				}
			}
			Err( errs ) => {
				Err( FitErrs::from_objects( errs ) )
			}
		}
	}
	
	//t = transcription.
	priv fn make_t_key( t_key: Must ) -> ~Object {
	
		let mut rval= ~HashMap::new();
		rval.insert( ~"t_key", t_key.to_json() );
		rval.insert( ~"spec_key", String( ~"CelvpCNzHNiPPUKL" ) );		
		rval
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
	while i <= max {
		let mbs = must_bank_in.clone();
		let count = i;
		do spawn {
			let mut doc = ~HashMap::new();
			doc.insert( ~"message",String( ~"must_bank " + count.to_str() + " reporting for duty." ) );
			let mut args = ~HashMap::new();
			args.insert( ~"user", String( ~"va4wUFbMV78R1AfB" ) );
			args.insert( ~"acct", String( ~"ofWU4ApC809sgbHJ" ) );
			args.insert( ~"must", Must::new().to_json() );	
			args.insert( ~"doc", doc.to_json() );
			args.insert( ~"spec_key", String( Bootstrap::spec_add_doc_key() ).to_json() );
			let ( p, c ) = oneshot();
			mbs.send( MBTranscript( args, c ) );
			match recv_one( p ) {
				Ok( _ ) => { // immediatly returns a t_key that can be used to get the doc key and so forth
					//std::io::println( extra::json::to_pretty_str(&(t_key.to_json())));
				}
				Err( err ) => { std::io::println( err.to_str() ); fail!(); }
			}
		}
		i += 1;
	}
	let mut doc = ~HashMap::new();
	doc.insert( ~"message",String( ~"must_bank error reporting for duty." ) );
	let mut args = ~HashMap::new();
	args.insert( ~"user", String( ~"va4wUFbMV78R1AfB" ) );
	args.insert( ~"acct", String( ~"ofWU4ApC809sgbHJ" ) );
	args.insert( ~"must", Must::new().to_json() );	
	args.insert( ~"doc", doc.to_json() );
	args.insert( ~"spec_key", String( Bootstrap::spec_add_doc_key() ).to_json() );
	let ( p, c ) = oneshot();
	must_bank_in.clone().send( MBTranscript( args, c ) );
	match recv_one( p ) {
		Ok( _ ) => { // immediatly returns a t_key that can be used (once indexes are up and running) to get the error and so forth
			//std::io::println( extra::json::to_pretty_str(&(t_key.to_json())));
		}
		Err( err ) => {std::io::println( err.to_str() ); fail!(); }
	}
	
	// The reason for these yields is that they prevent task failures in teardown
	// I thinlk I need to figure out how not to call MBRelease until after after all the 
	// transscriptors are up and running.  Since this is a teardown issue, I am
	// to chew on it a bit while I debate with myself :) whether it is worth adding
	//	paying for the extra plumbing to keep track of this.	
	yield();yield();yield();yield();yield();yield();yield();yield();yield();yield();   // <- temp fix 
						
	let ( p, c ) = oneshot();
    must_bank_in.clone().send( MBRelease( c ) );
	recv_one( p );  // wait for the ack
	std::io::println( "reminder: check and delete test.json" );
}


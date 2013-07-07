//	Copyright 2013 Jeff Murray.

//	Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//	http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//	<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//	option. This file may not be copied, modified, or distributed
//	except according to those terms.
 
 #[link(name = "must_bank", vers = "1.0")];  
 
//	rustc --lib must_bank.rs -L .
//	rustc must_bank.rs --test -o must_bank-tests -L .
//	./must_bank-tests

extern mod std;
extern mod core;
extern mod jah_mut;
extern mod jah_spec;
extern mod jah_args;
extern mod par;
extern mod fit;
extern mod bootstrap;
extern mod strand;
extern mod must;
extern mod parts;
use jah_args::{ JahArgs };
use jah_spec::{ JahSpec };
use par::{ FitOutcome, ParTrans };
use fit::{ FitOk, FitErr, FitSysErr };
use parts::{ ParTs, ParTInComm, ParTInAdminComm, AddParT,  GetParTChan, ParTChan, ParTErr, ParTsRelease };
use must::{ Must };
use strand::{ Ribosome, DoFit, NextErr, NextOk, EndOfStrand, LogicErr };
use jah_mut::{ JahMutReq, GetStr, GetJson, JahMut, LoadMap, MergeArgs, Release }; // 
use std::json::{ Object, String, ToJson };
use bootstrap::{ Bootstrap };
use core::comm::{ oneshot, recv_one, ChanOne, stream, SharedChan };
use core::hashmap::linear::LinearMap;
use core::task::{ spawn };

struct MustBank;

enum MustBankInComm {
	MBTranscript( ~Object, ChanOne<Result< ~Object, ~Object >> ), // args, Result( trans_key info ) or Err info 
	MBRelease( ChanOne<()> ) //( ack_chan )
}

struct Transcriptor;

impl MustBank {
	
	pub fn connect() -> Chan<MustBankInComm> {
	
		let (port, chan ) = stream();
		MustBank::loop_in_spawn( port );
		chan
	}
	
	priv fn loop_in_spawn( port: Port< MustBankInComm > ) {
	
		do spawn {
			let ( user_parts_chan, admin_parts_chan ) = MustBank::load_parts();  //leaving the warning to remind me to tidy this up
			let user_parts_chan = SharedChan( user_parts_chan );
			let ( goodby_port, goodby_chan ) = stream();
			let goodby_chan = SharedChan( goodby_chan );
			let mut t_count = 0u;
			let mut releasing = false;
			loop {
				let ( recv_trans, recv_goodby ) = {
					if t_count == 0u {
						( true, false )
					} else if releasing {
						( false, true )
					} else {
						( port.peek(), goodby_port.peek() )
					}};
				
				if recv_trans {
					match port.recv() {
						MBTranscript( args, chan_one ) => {
							t_count += 1;
							io::println( ~"t_count = " + t_count.to_str() );
							Transcriptor::connect( ~"DROOg7Vt2GXiVl00").send( ( args, chan_one, user_parts_chan.clone(), goodby_chan.clone() ) );  // ( strand_key )  the kickoff strand for new requests
						}
						MBRelease( ack_chan ) => {
							while t_count > 0 { //TODO: put a timeout here
								goodby_port.recv();
								t_count -= 1;
								io::println( ~"t_count = " + t_count.to_str() );
							}
							let ( c, p ) = oneshot::init();
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
					task::yield();
				}
			}
		}		
	}

	priv fn load_parts() -> ( Chan<ParTInComm>, Chan<ParTInAdminComm> ) {
	
		let ( user_chan, admin_chan ) = ParTs::connect();
		match {	let ( c, p ) = oneshot::init();
				admin_chan.send( AddParT( ~"S68yWotrIh06IdE8", c ) ); // FileAppendJSON
				recv_one( p )
			} {
				Ok( _ ) => {}
				Err( _ ) => { fail!(); }
		}
		
		match {	let ( c, p ) = oneshot::init();
				admin_chan.send( AddParT( ~"Zbh4OJ4uE1R1Kkfr", c ) ); // ErrFit
				recv_one( p )
			} {
				Ok( _ ) => {}
				Err( _ ) => { fail!(); }
		}
		( user_chan, admin_chan )
	}
}

impl Transcriptor {

	fn connect( kickoff_strand_key: ~str ) -> Chan<((~Object, ChanOne<Result< ~Object, ~Object >>, SharedChan<ParTInComm>, SharedChan<int>))> {
	
		let ( start_port, start_chan ) = stream();
		
		do spawn {
			let kickoff_strand_key = copy kickoff_strand_key;	
			let ( args, home_chan_one, parts_chan, goodby_chan ): (~Object, ChanOne<Result< ~Object, ~Object >>, SharedChan<ParTInComm>, SharedChan<int>) = start_port.recv();
			let t_key = Must::new();
			home_chan_one.send(  Ok( Transcriptor::make_t_key( copy t_key ) ) );
			let ( arg_bank_user_chan, arg_bank_admin_chan ) = JahMut::connect();  //  <-- the arg bank
			let arg_bank_sh_chan = SharedChan( arg_bank_user_chan );
			arg_bank_admin_chan.send( LoadMap( copy args ) );
			let ( rib_port, rib_chan ) = Ribosome::connect( kickoff_strand_key, arg_bank_sh_chan.clone() );
			loop {
				let reg_key = { 
					match rib_port.recv() {
						DoFit( key ) => { key }
						LogicErr( err ) => { io::println( std::json::to_pretty_str(&(err.to_json())));break; } //  <- temp band-aid, pure logic errors should be pretty rare 
						EndOfStrand	=> { io::println( ~"EndOfStrand" ); break; }
					}};
				let spec_key = { //get the latest spec that was loaded in the arg bank
					match { let ( c, p ) = oneshot::init();
						arg_bank_sh_chan.clone().send( GetStr( ~"spec_key", c ) );
						recv_one( p ) }
					{ 	Some( spec_key ) => { spec_key }
						None => { io::println( ~"no spec key found in must_bank.rs" ); break; }
					}};
				let args = { 
					match Transcriptor::speced_arg_excerpt( Bootstrap::find_spec( spec_key ), arg_bank_sh_chan.clone() ) {
						Ok( args ) => { args }
						Err( err ) => { io::println( ~"speced_arg_excerpt" ); io::println( std::json::to_pretty_str(&(err.to_json()))); break; }  //Reporting this error will require the indexes be up and running
					}};																						//Transcribing this can get tied in with the rest of the reporting
				// get the Par chan and send args
				let fo: FitOutcome = {
					match { let ( c, p ) = oneshot::init();
						parts_chan.send( GetParTChan( reg_key, c ) ); // ChanOne<ParTOutComm>
						recv_one( p )
						} {	ParTChan( part_chan ) => { // ( part_chan ) ChanOne<ParInComm>
								let ( c2, p2 ) = oneshot::init();
								part_chan.send( ParTrans( copy args , c2 ) ); // ChanOne<ParTOutComm>
								recv_one( p2 )
							} 
							ParTErr( err ) => { io::println( std::json::to_pretty_str(&(err.to_json()))); break; }
					}};
				// Record the fit performance once the indexing system is up and running
				// update the arg_bank
				
				match copy fo.outcome {
					FitOk( rval ) => {
						arg_bank_admin_chan.send( MergeArgs( copy rval ) );
						rib_chan.send( NextOk );
					}
					FitErr( rval ) => {
						arg_bank_admin_chan.send( MergeArgs( copy rval ) );
						rib_chan.send( NextErr );
					}
					FitSysErr( err ) => {
						io::println( ~"dgfhjk" + JahArgs::new( err ).to_str() );
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
	
	priv fn speced_arg_excerpt( spec: ~Object, arg_bank_chan: SharedChan<JahMutReq> )-> Result<~Object, ~Object> {
		
		let jah_spec = JahSpec::new( spec );
		let mut rval = ~LinearMap::new();
		for { match jah_spec.allowed_keys() {
				Ok( keys ) => { keys } 
				Err( err ) => { return Err( err ); }
			}
		}.each | key | 
		{	match {
				let ( c, p ) = oneshot::init();
				arg_bank_chan.send( GetJson( copy *key, c ) );
				recv_one( p )
				}
			{	Some( arg_val ) => { 
					rval.insert( copy *key, arg_val ); // <--
				}
				None => {}  // leaving the ramifications of this missing arg to the spec check	
			}
		}
		match jah_spec.check_args( JahArgs::new( copy rval ) ) {
			Ok( _ ) => {
				Ok( rval ) // we have validated args, ready to roll
			}
			Err( errs ) => {
				Err( Bootstrap::mk_mon_err( errs ) ) 
			}
		}
	}
	
	//t = transcription.
	priv fn make_t_key( t_key: Must ) -> ~Object {
	
		let mut rval= ~LinearMap::new();
		rval.insert( ~"t_key", t_key.to_json() );
		rval.insert( ~"spec_key", String( ~"CelvpCNzHNiPPUKL" ) );		
		rval
	}
}

/*#[test]
fn add_document_strand() {

	let must_bank_in = MustBank::connect();
	let max = 1000i;
	let mut i = 1i;
	io::println( ~"Inserting " + max.to_str() + " documents." );
	while i <= max {
		let mut doc = ~LinearMap::new();
		doc.insert( ~"message",String( ~"must_bank " + i.to_str() + " reporting for duty." ) );
		let mut args = ~LinearMap::new();
		args.insert( ~"user", String( ~"va4wUFbMV78R1AfB" ) );
		args.insert( ~"acct", String( ~"ofWU4ApC809sgbHJ" ) );
		args.insert( ~"must", Must::new().to_json() );	
		args.insert( ~"doc", doc.to_json() );
		args.insert( ~"spec_key", String(~"uHSQ7daYUXqUUPSo").to_json() );
		let ( c, p ) = oneshot::init();
		must_bank_in.send( MBTranscript( args, c ) );
		match recv_one( p ) {
			Ok( _ ) => { // immediatly returns a t_key that can be used to get the doc key and so forth
				//io::println( std::json::to_pretty_str(&(t_key.to_json())));
			}
			Err( _ ) => { fail!(); }
		}
		i += 1;
	}
	let ( c, p ) = oneshot::init();
	must_bank_in.send( MBRelease( c ) );
	recv_one( p );  // wait for the ack
	io::println( ~"reminder: check and delete test.json" );
}*/

#[test]
fn error_strand() {
	
	let must_bank_in = MustBank::connect();
	let mut doc = ~LinearMap::new();
	doc.insert( ~"message",String( ~"must_bank error reporting for duty." ) );
	let mut args = ~LinearMap::new();
	args.insert( ~"user", String( ~"va4wUFbMV78R1AfB" ) );
	args.insert( ~"acct", String( ~"ofWU4ApC809sgbHJ" ) );
	args.insert( ~"must", Must::new().to_json() );	
	args.insert( ~"doc", doc.to_json() );
	args.insert( ~"spec_key", String(~"uHSQ7daYUXqUUPSo").to_json() );
	let ( c, p ) = oneshot::init();
	must_bank_in.send( MBTranscript( args, c ) );
	match recv_one( p ) {
		Ok( _ ) => { // immediatly returns a t_key that can be used (once indexes are up and running) to get the error and so forth
			//io::println( std::json::to_pretty_str(&(t_key.to_json())));
		}
		Err( err ) => {io::println( std::json::to_pretty_str(&(err.to_json()))); fail!(); }
	}
	let ( c2, p2 ) = oneshot::init();
	task::yield();
	must_bank_in.send( MBRelease( c2 ) );
	recv_one( p2 );  // wait for the ack
	io::println( ~"Check that the error output was written to console" );
}

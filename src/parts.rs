//	Copyright 2013 Jeff Murray.

//	Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//	http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//	<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//	option. This file may not be copied, modified, or distributed
//	except according to those terms.

//	rustc --lib parts.rs -L . -L fits
//	rustc parts.rs --test -o parts-tests -L . -L fits
//	./parts-tests

#[link(name = "parts", vers = "0.0")];

extern mod std;
extern mod extra;
extern mod par;
extern mod fit;
extern mod bootstrap;
extern mod must;
use par::{ Par, ParInComm, ParCommEndChan, ParTrans, FitOutcome }; //ParTrans and FitOutcome are used in unit tests
use fit::{ Parfitable, ParFitComm, FitErrs, FitOk, FitErr, FitSysErr, FitArgs };// FitOk, FitErr, FitSysErr and FitArgs are used in unit tests
use bootstrap::{ Bootstrap };
use must::{ Must }; //Must is not used in unit tests
use std::hashmap::HashMap;
use extra::json::{ String, ToJson };
use std::comm::{ stream, Chan, SharedChan, ChanOne, oneshot };
use std::task::{ spawn, yield };

//  ParTs is the place where live Parfitables and their channels can be loaded and accessed 

//	T = Terminal
//	ParT: holds a shared channel to a "live" instance of a Par
//	Pronounce it Part or Par Tee 

type ParT = SharedChan<ParInComm>;

struct ParTs;

enum ParTInComm {
	GetParTChan( ~str, ChanOne<ParTOutComm> )  // ( reg_key, oneshot to in chan )
}

enum ParTOutComm {
	ParTChan( ChanOne<ParInComm> ), // ( part_chan )
	ParTErr( ~FitErrs ) // spec VWnPY4CStrXkk4SU
}

enum ParTInAdminComm {
	AddParT( ~str, uint, Chan<ParFitComm>, ChanOne<Result< bool, ~FitErrs >> ), // ( reg_key, fit_chan, result_chan ),
	ParTsRelease( ChanOne<()> )
}

impl ParTs {

	pub fn connect() -> Result<( SharedChan<ParTInComm>, SharedChan<ParTInAdminComm> ), ~FitErrs> {

		let ( user_port, user_chan ) = stream();
		let ( admin_port, admin_chan ) = stream();
		let user_chan = SharedChan::new( user_chan );
		do spawn {
			let mut parts = ~HashMap::new();			
			loop {
				let mut recvd = false;
				let mut break_again = false;  //Haven't figured out how to directly exit a spawn from an inner loop without using fail!()
				while admin_port.peek() {
					recvd = true;
					let part: ParTInAdminComm = admin_port.recv();
					match part {
						AddParT( reg_key, spawn_cap, fit_chan, result_chan ) => {
							let par = Par::new( spawn_cap );
							match par.connect(fit_chan) {
								Ok( par_chan ) => {
									//this whole part adding thing needs some diagnostics built in
									if !parts.contains_key( &reg_key ) {
										parts.insert( reg_key, SharedChan::new( par_chan ) );
										result_chan.send( Ok( true ) );
										println("part added");
									} else {
										result_chan.send( Err( FitErrs::from_obj( Bootstrap::logic_error(Bootstrap::part_key_added_twice_key(), ~"reg_key", ~"b2eXWGdcdmEgc5Tu", ~"parts.rs") ) ) );
										println("dup part");
									}									
								}
								Err( error ) => {
									result_chan.send( Err( error.prepend_err( Bootstrap::reply_error_trace_info( ~"parts.rs", ~"seGs8AWBelJ7C4cD") ) ) );
									println("part err");
								}
							}
						},
						ParTsRelease( ack_chan ) => {
							do parts.consume | reg_key , chan| { 
								println( ~"releasing " + reg_key );
								let ( p, c ) = oneshot();
								chan.send( ParCommEndChan( c ) );
								p.try_recv().expect("parts 99") ;
							}
							break_again = true;
							ack_chan.send( () );
							break;
						}
					}
				}
				if break_again { break; }
				if user_port.peek() {
					recvd = true;
					let usr_req: ParTInComm = user_port.recv();
					match usr_req {
						GetParTChan( reg_key, par_chan_one ) => { 
							let opt_chan = parts.find( &reg_key );
							match opt_chan {
								Some( chan ) => {
									ParTs::no_wait_reply( chan.clone() ).send( par_chan_one );
								}
								None => {
									par_chan_one.send( ParTErr( FitErrs::from_obj( Bootstrap::logic_error( Bootstrap::part_does_not_exist(), copy reg_key, ~"Q5jmEpjJ4yNzywjv", ~"parts.rs" ) ) ) );
								}
							}
						}
					}
				}
				if !recvd { yield(); }				
			}
		}	
		Ok( ( user_chan.clone(), SharedChan::new( admin_chan ) ) )
	}

	priv fn no_wait_reply( par_chan: SharedChan<ParInComm> ) -> Chan<ChanOne<ParTOutComm>> {

		// the last thing I want is for ParTs to get hung up waiting on a 
		// send that does not get picked up, or that is slow to get picked up.
		//let par_chan = part.chan.clone();
		let ( port, chan ) = stream();
		do spawn {
			let chan_one: ChanOne<ParTOutComm> = port.recv();
			let ( p, c ) = oneshot();
			chan_one.send( ParTChan( c ) ); // ChanOne<ParInComm>
			par_chan.send( p.recv() );
		}
		chan
	}	
}

/*#[test]
fn various_parts() {

	let ( user_chan, admin_chan ) = {
		match ParTs::connect() {
			Ok( ( user_chan, admin_chan ) ) => {
				( user_chan, admin_chan )
			} _ => { fail!(); }
		}};
	match {	let ( p, c ) = oneshot();
			admin_chan.send( AddParT( Bootstrap::file_append_slice_key(), c ) );  //FileAppendSlice
			p.recv()
		} {
			Ok( _ ) => {}
			Err( _ ) => { fail!(); }
	}
	match {	let ( p, c ) = oneshot();
			admin_chan.send( AddParT( Bootstrap::err_fit_key(), c ) );  // ErrFit
			p.recv()
		} {
			Ok( _ ) => {}
			Err( _ ) => { fail!(); }
	}
	match {	let ( p, c ) = oneshot();
			admin_chan.send( AddParT( Bootstrap::doc_slice_prep_key(), c ) );  // DocSlicePrep
			p.recv()
		} {
			Ok( _ ) => {}
			Err( _ ) => { fail!(); }
	}
	match {	let ( p, c ) = oneshot();
			admin_chan.send( AddParT(  Bootstrap::file_get_slice_key(), c ) );  // FileGetSlice
			p.recv()
		} {
			Ok( _ ) => {}
			Err( _ ) => { fail!(); }
	}
	let mut doc = ~HashMap::new();
	doc.insert( ~"message",String( ~"Hello from inside ParTs::connect()!" ) );
	let mut args = ~HashMap::new();
	args.insert( ~"user", String( ~"va4wUFbMV78R1AfB" ) );
	args.insert( ~"acct", String( ~"ofWU4ApC809sgbHJ" ) );
	//args.insert( ~"must", Must::new().to_json() );	
	args.insert( ~"doc", doc.to_json() );
	args.insert( ~"spec_key", String( Bootstrap::spec_add_doc_key() ).to_json() );
	let fo: FitOutcome = {
			match { let ( p, c ) = oneshot();
			user_chan.send( GetParTChan( ~"6Ssa58eFrC5Fpmys", c ) ); // ChanOne<ParTOutComm>
			p.recv()
		} {	ParTChan( part_chan ) => { // ( part_chan ) ChanOne<ParInComm>
				let ( p2, c2 ) = oneshot();
				part_chan.send( ParTrans( ~FitArgs::from_doc( copy args ), c2 ) ); // ChanOne<ParTOutComm>
				p2.recv()
			} 
			ParTErr( _ ) => { fail!(); } // spec VWnPY4CStrXkk4SU
		}};
	match copy fo.outcome {
		FitOk( _ ) => {
		}
		FitSysErr( _ ) => {
			fail!();
		}
		FitErr( _ ) => {
			fail!();
		}
	}
	let ( p, c ) = oneshot();
	admin_chan.send( ParTsRelease( c ) );
	p.recv();
}*/
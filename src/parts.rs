//	Copyright 2013 Jeff Murray.

//	Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//	http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//	<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//	option. This file may not be copied, modified, or distributed
//	except according to those terms.

//	rustc --lib parts.rs -L .
//	rustc parts.rs --test -o parts-tests -L .
//	./parts-tests

#[link(name = "parts", vers = "1.0")];

extern mod std;
extern mod core;
extern mod par;
extern mod fit;
extern mod bootstrap;
extern mod must;
extern mod jah_spec;
//excuse me while I load the fits here for now.
extern mod file_append_json;
extern mod err_fit;
use err_fit::{ ErrFit };
use file_append_json::{ FileAppendJSON };
//***
use jah_spec::{ JahSpeced };
use par::{ Par, ParInComm, ParTrans, ParCommEndChan, FitOutcome };
use fit::{ Parfitable, ParFitComm, FitOk, FitErr, FitSysErr };
use bootstrap::{ Bootstrap };
use must::{ Must };
use core::hashmap::linear::LinearMap;
use std::json::{ Object, String, ToJson };
use core::comm::{ stream, Chan, SharedChan, ChanOne, oneshot, recv_one };
use core::task::{ spawn };

//  ParTs is the place where live Parfitables and their channels can be loaded and accessed 

//	T = Terminal
//	ParT: holds a shared channel to a "live" instance of a Par
//	Pronounce it Part or Par Tee 

struct ParT {
	chan: SharedChan<ParInComm>,
	specs_in: ~[~str],
	specs_out: ~[~str]
}

struct ParTs;

enum ParTInComm {
	GetParTChan( ~str, ChanOne<ParTOutComm> )  // ( reg_key, oneshot to in chan )
}

enum ParTOutComm {
	ParTChan( ChanOne<ParInComm> ), // ( part_chan )
	ParTErr( ~Object ) // spec VWnPY4CStrXkk4SU
}

enum ParTInAdminComm {
	AddParT( ~str, ChanOne<Result< bool, ~Object >> ), // ( reg_key, result_chan ),
	ParTsRelease
}

impl ParTs {

	pub fn connect() -> ( Chan<ParTInComm>, Chan<ParTInAdminComm> ) {
	
		let ( user_port, user_chan ) = stream();
		let ( admin_port, admin_chan ) = stream();	
		do spawn {
			let mut parts = ~LinearMap::new();
			loop {
				let mut recvd = false;
				let mut break_again = false;  //Haven't figured out how to directly exit a spawn from an inner loop
				while admin_port.peek() {
					recvd = true;
					let part: ParTInAdminComm = admin_port.recv();
					match part {
						AddParT( reg_key, result_chan ) => {
							match ParTs::load_part( copy reg_key ) {
								Ok( part ) => {
									parts.insert( reg_key, part );
									result_chan.send( Ok( true ) );
								}
								Err( error ) => {					
									result_chan.send( Err( Bootstrap::mk_mon_err( ~[ Bootstrap::reply_error_trace_info( ~"parts.rs", ~"seGs8AWBelJ7C4cD"), error ] ) ) );
								}
							}
						},
						ParTsRelease => {
							for parts.each |&(_, part)| {
								let chan = part.chan.clone();
								chan.send( ParCommEndChan );
							}
							break_again = true;
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
							let opt_part = parts.find( &reg_key );
							match opt_part {
								Some( part ) => {
									ParTs::no_wait_reply( part ).send( par_chan_one );
								}
								None => {
									par_chan_one.send( ParTErr( Bootstrap::mk_mon_err( ~[ Bootstrap::logic_error( Bootstrap::part_does_not_exist(), copy reg_key, ~"Q5jmEpjJ4yNzywjv", ~"parts.rs" ) ] ) ) );
								}
							}
						}
					}
				}
				if !recvd { task::yield(); }				
			}
		}	
		( user_chan, admin_chan )
	}

	priv fn no_wait_reply( part: &~ParT ) -> Chan<ChanOne<ParTOutComm>> {

		// the last thing I want is for ParTs to get hung up waiting on a 
		// send that does not get picked up, or that is slow to get picked up.
		let par_chan = part.chan.clone();
		let ( port, chan ) = stream();
		do spawn {
			let chan_one: ChanOne<ParTOutComm> = port.recv();
			let ( c, p ) = oneshot::init();
			chan_one.send( ParTChan( c ) ); // ChanOne<ParInComm>
			par_chan.send( recv_one( p ) );
		}
		chan
	}	

	// I'm planning to make a document based fit registry after the indexing systems up and running
	// for now they will get hard-coded
	
	priv fn start_part<T: Parfitable+JahSpeced>( par: ~Par, fit: ~T ) -> Result<~ParT, ~Object> { //notice how Rust generics kick ass?
	
		let fit_chan = {
			let rslt: Result<Chan<ParFitComm>, ~Object> = fit.connect();
			match rslt {
				Ok( fit_chan ) => {
					fit_chan
				}
				Err( obj ) => {
					return Err( Bootstrap::mk_mon_err( ~[obj] ) );
				}
			}};
		
		match par.connect(fit_chan) {
			Ok( par_chan ) => {
				Ok( ~ParT { chan: SharedChan( par_chan ), specs_in: copy fit.spec_keys_in(), specs_out: copy fit.spec_keys_out() } )
			}
			Err( err ) => {
				Err( Bootstrap::mk_mon_err( ~[err] ) )
			}
		}		
	}
	
	// The reg_key identifies a specific live instance of a fit.  The reason the fit_key is not used
	// here is because having multiple instances of the same Fit can be handy.
	priv fn load_part( reg_key: ~str ) -> Result<~ParT, ~Object> {
	
		match reg_key {
			~"S68yWotrIh06IdE8" => {
				//	Appends a document to a file along with >> TODO MD5 <<  and file slice information
				//	Takes spec uHSQ7daYUXqUUPSo
				//	Returns Ok spec 5W6emlWjT77xoGOH Err spec VWnPY4CStrXkk4SU
				
				let fit = ~FileAppendJSON{ file_args: {
						let mut config = ~LinearMap::new();
						config.insert( ~"path", String(~"test.json").to_json() );
						config.insert( ~"num", 1u.to_json() );
						config.insert( ~"spec_key", String(~"5W6emlWjT77xoGOH").to_json() );
						config 
					}};	
				ParTs::start_part( Par::new( 20u ), fit )
			}
			~"Zbh4OJ4uE1R1Kkfr" => {
				// Writes errors to the terminal window formated to a pretty string
				// Takes any Object
				// Returns spec er5OWig71VG9oNjK (the empty spec) 

				ParTs::start_part( Par::new( 20u ), ~ErrFit{ settings: ~LinearMap::new() } )
			}			
			_ => {
				Err( Bootstrap::logic_error( Bootstrap::part_does_not_exist(), copy reg_key, ~"9ZwGwLZSSwByYfs7", ~"parts.rs" ) )
			}
		}
	}	
}

#[test]
fn various_parts() {
	
	let ( user_chan, admin_chan ) = ParTs::connect();
	match {	let ( c, p ) = oneshot::init();
			admin_chan.send( AddParT( ~"S68yWotrIh06IdE8", c ) );
			recv_one( p )
		} {
			Ok( _ ) => {}
			Err( _ ) => { fail!(); }
	}
	match {	let ( c, p ) = oneshot::init();
			admin_chan.send( AddParT( ~"Zbh4OJ4uE1R1Kkfr", c ) );
			recv_one( p )
		} {
			Ok( _ ) => {}
			Err( _ ) => { fail!(); }
	}
	let mut doc = ~LinearMap::new();
	doc.insert( ~"message",String( ~"Hello from inside ParTs::connect()!" ) );
	let mut args = ~LinearMap::new();
	args.insert( ~"user", String( ~"va4wUFbMV78R1AfB" ) );
	args.insert( ~"acct", String( ~"ofWU4ApC809sgbHJ" ) );
	args.insert( ~"must", Must::new().to_json() );	
	args.insert( ~"doc", doc.to_json() );
	args.insert( ~"spec_key", String(~"uHSQ7daYUXqUUPSo").to_json() );
	let fo: FitOutcome = match { let ( c, p ) = oneshot::init();
		user_chan.send( GetParTChan( ~"S68yWotrIh06IdE8", c ) ); // ChanOne<ParTOutComm>
		recv_one( p )
		} {	ParTChan( part_chan ) => { // ( part_chan ) ChanOne<ParInComm>
				let ( c2, p2 ) = oneshot::init();
				part_chan.send( ParTrans( copy args , c2 ) ); // ChanOne<ParTOutComm>
				recv_one( p2 )
			} 
			ParTErr( _ ) => { fail!(); } // spec VWnPY4CStrXkk4SU
		};
	match copy fo.outcome {
		FitOk( _ ) => {
		}
		FitSysErr( _ ) => {
			//io::println( JahArgs::new( err ).to_str() );
			fail!();
		}
		FitErr( _ ) => {
			//io::println( JahArgs::new( err ).to_str() );
			fail!();
		}
	}
	admin_chan.send( ParTsRelease );
}	


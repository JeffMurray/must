//	Copyright 2013 Jeff Murray.

//	Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//	http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//	<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//	option. This file may not be copied, modified, or distributed
//	except according to those terms.
 
 #[link(name = "stately", vers = "0.0")];  
 
//	rustc --lib stately.rs -L . -L fits
//	rustc stately.rs --test -o stately-tests -L . -L fits
//	./stately-tests

extern mod std;
extern mod extra;
extern mod jah_args;
extern mod par;
extern mod fit;
extern mod bootstrap;
extern mod must;
extern mod parts;
extern mod transcriptor;
use transcriptor::{ Transcriptor };
use jah_args::{ JahArgs };
use par::{ ParTrans };
use fit::{ FitArgs };
use parts::{ ParTInComm, GetParTChan, ParTChan, ParTErr };
use must::{ Must };
use bootstrap::{ Bootstrap };
use std::comm::{ oneshot, ChanOne, stream, SharedChan };
use std::task::{ spawn };

enum LoopOutComm {
	ComebackIfOk( ~FitArgs, ~FitArgs, ~str, Must, ChanOne<LoopInComm> ), // ( state, args, strand_key, t_key, comeback_chan ),
	StatelyRelease
}

enum LoopInComm {
	Comeback( ~FitArgs, ~FitArgs ), // ( state, args )
	LostToErr( ~FitArgs ) // ( state )
}

struct StateServ;

impl StateServ {
	
	pub fn connect( parts_chan: SharedChan<ParTInComm> ) -> Chan<LoopOutComm> {
	
		let ( in_port, in_chan ) = stream();
		StateServ::go( in_port, parts_chan.clone() );
		in_chan
	}
	
	priv fn go( port: Port<LoopOutComm>, parts_chan: SharedChan<ParTInComm> ) {

		do spawn {
			loop {
				match port.try_recv().expect("stately.rs 9aaCGY2qUQLWnbC0") {
					ComebackIfOk( state, args, strand_key, t_key, comeback_chan ) => {
						let ( port, chan ) = stream();
						StateServ::do_comeback( port, parts_chan.clone() );
						chan.send( ComebackIfOk( state, args, strand_key, t_key, comeback_chan ) );					
					}
					StatelyRelease => {
						break;
					}
				}
			}					
		} 		
	}
		
	priv fn do_comeback( port:  Port<LoopOutComm>, parts_chan: SharedChan<ParTInComm> ) {
		
		do spawn {
			//let stdin = std::io::stdin();
			match port.try_recv().expect("stately.rs yuuylItuDTAzTrGt") {
				ComebackIfOk( state, args, strand_key, t_key, comeback_chan ) => {
		  				println( "ComebackIfOk" );
						//stdin.read_line();				
					let ( gb_port, gb_chan ) = stream();
					let goodby_chan = SharedChan::new( gb_chan );
					let t_chan = Transcriptor::connect( strand_key, copy t_key ) ;
					t_chan.send( ( args, parts_chan.clone(), goodby_chan.clone() ) );
					match gb_port.try_recv().expect("stately.rs pZMohLpjPywldwUO") {
						Ok( fit_args ) => {
							comeback_chan.send( Comeback( state, fit_args ) );
						}
						Err( fit_errs ) => {
							println( "- E R R O R -" );
							println( fit_errs.to_str() );
							comeback_chan.send( LostToErr( state ) );
						}
					}
				}
				StatelyRelease => {
					fail!(); // the calling loop already checks for this
				}				
			}			
		}
	}
}

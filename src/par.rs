//	Copyright 2013 Jeff Murray.

//	Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//	http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//	<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//	option. This file may not be copied, modified, or distributed
//	except according to those terms.

//	rustc --lib par.rs -L .
//	rustc par.rs --test -o par-tests -L .
//	./par-tests

#[link(name = "par", vers = "1.0")];

extern mod std;
extern mod core;
extern mod fit;
use std::time::Timespec;
use fit::{ ParFitable, FitComm, ParFitComm, FitOk, FitErr, FitTryFail, DoFit, ParFitCommEndChan };
use core::comm::{ stream, Port, Chan };
use std::comm::DuplexStream;
use std::json::{ Object };
use core::task::{ spawn };


//	If there is a central component in Must, it is the Programmable Argument Relay (Par).

//	Think of a Par as single cube that receives arguments from the cube to the left, and then 
//	spawns a Functionally Isolated Transaction (Fit) to perform some work and handles all 
//	communication related to passing on, or reporting errors. By implementing ParFitable the 
//	impl will declare whether it is spawn-able or not.  It is generally advocated that Fits be 
//	spawn-able, but in rare cases, like when writing to a file, it there is reason to choose 
//	not to be spawn-able.

//  When the Fit is run,  
//	if errors are reported the Par will send them to the error relay that is connected to it, 
//	or if Ok the arguments reported are sent to the next receiving Par.

//				********************			********************			********************  
//				*****system &*******			*****system &*******			*****system &*******
//				***	error relay	 ***			*** error relay	 ***			***	error relay  ***
//				********************			********************			********************
//						recv							recv							recv
//
//						send							send							send
//				********************			********************			********************
//				********************			********************			********************
//				********************			********************			********************
//		 recv	********Par*********send  recv	********Par*********send  recv	********Par*********send   >> still good >>
//				********************			********************			********************
//				********************			********************			********************
//				********************			********************			********************
//				********************			********************			********************
//					 
//					***********						***********						***********
//					**	Fit	***						**	Fit	***						**	Fit ***
//					***********						***********						***********


//  While the functional guts of the application reside in the Functionally Isolated Transactions,
//	the entire communication path is defined through relays. 

//	I am trying to solve a few goals with this Programmable Argument Relay ( Par ):

//	*The guts of the communication system should be made abstract to day to day production programming.

//	*It should take copious advantage of the scrumptious Rust task spawning features.

//	*It should be easy to specify and cap the amount of concurrent spawns allowed in each relay. 

//	*It should take advantage of the core::comm and std::comm task communication ports and chans.

//	*Implement a transaction key that ties series of transactions across relays..

//	*Pass transaction detail and, forward relay ending errors to an error relay equipped with a fit that knows what to
//	do with them. The monitoring detail will automatically include the transaction id, a time-stamp, and the concurrent 
//	number of spawns active in the relay. 

//	I plan to use JahArgs to pass arguments around in Must, but I'm going to make the Par
//	and Fit base implementations generic so that the relays can be put to other uses.

struct Par {
	priv spawn_cap: uint,
	priv timeout_ms: uint,
	priv fit: ParFitable
}

//Mon = Monitor
enum MonComm { 
	FitStart( ~str, Timespec, ~str ),
	FitEndOkay( ~str, Timespec, ~str ),
	FitEndError( ~str, Timespec, ~str, Object ),
	FitEndTryFail( ~str, Timespec, ~str),
	MonCommEndChan
}
enum ParComm {
	ParTrans( ~str, Object ),
	ParCommEndChan
}
enum ToDo {
	RecvLeftChan,
	RecvSpawnChan,
	Yield
}

impl Par {

	fn new(&self, fit: ParFitable, timeout_ms: uint, spawn_cap: uint ) -> Par {
	
		Par {
			timeout_ms: timeout_ms,
			spawn_cap: spawn_cap,
			fit: fit
		}
	}
	
	fn connect( &self, left: Port<ParComm>, monitor: Chan<MonComm> ) -> Port<ParComm> {
	
		let (right_port, right_chan): (Port<ParComm>, Chan<ParComm>) = stream();
		let (par_plex, fit_plex) = DuplexStream();
		self.fit.connect( fit_plex );
		self.spawn_and_run( left, monitor, right_chan, par_plex );
		right_port
	}
	
	priv fn spawn_and_run( &self, left: Port<ParComm>, monitor: Chan<MonComm>, right: Chan<ParComm>, par_plex: DuplexStream<ParFitComm, FitComm> ) {
	
		let spawn_cap = if self.fit.spawnable() && self.spawn_cap > 0 { self.spawn_cap } else { 1u };
		let fit_key = self.fit.key();
		do spawn {
			let mut current_spawns = 0u;
			loop {
				for Par::make_to_do_list( current_spawns, spawn_cap, left.peek(), par_plex.peek() ).each | to_do | {
					match *to_do {
						RecvLeftChan => {
							let new_req = left.recv();
							current_spawns += 1;
							match new_req {
								ParTrans( t_key, args ) => {
									//recording the start time of this fit
									let ts = std::time::at_utc( std::time::get_time() ).to_timespec();
									let t = FitStart( copy t_key, ts, copy fit_key );
									par_plex.send( DoFit( copy t_key, copy args ) );
									monitor.send( t );
								}
								ParCommEndChan => { 
									//I am considering letting spawned tasks finish
									//but not in round 1
									monitor.send( MonCommEndChan );
									right.send( ParCommEndChan );
									par_plex.send( ParFitCommEndChan );
									break; 
								}
							}					
						}
						RecvSpawnChan => {
							let fit_val = par_plex.recv();
							current_spawns -= 1;
							match fit_val {
								FitOk( t_key, t_stamp, args ) => {
									right.send( ParTrans(  copy t_key, copy args ) );
									monitor.send( FitEndOkay( copy t_key, t_stamp, copy fit_key ) );
								}
								FitErr( t_key, t_stamp, args ) => {
									monitor.send( FitEndError( copy t_key, t_stamp, copy fit_key, args ) );
								}
								FitTryFail( t_key, t_stamp ) => {
									monitor.send( FitEndTryFail( copy t_key, t_stamp, copy fit_key ) );
								}
							} 					
						}
						Yield => {
							task::yield();
						}				
					}
				}
			}
		}
	}
	
	priv fn make_to_do_list(current_spawns: uint, spawn_cap: uint, left_peak: bool, spawn_peak: bool) -> ~[ToDo] {
	
		let mut to_do = ~[];
		if current_spawns == spawn_cap {
			to_do.push ( RecvSpawnChan );
		} else if current_spawns == 0u { 
			to_do.push ( RecvLeftChan );
		} else {
			if spawn_peak { 
				to_do.push ( RecvSpawnChan ); 
			}
			if left_peak { 
				to_do.push ( RecvLeftChan ); 
			}
			if to_do.len() == 0 { 
				to_do.push( Yield ); 
			}				 
		}
		to_do
	}
}



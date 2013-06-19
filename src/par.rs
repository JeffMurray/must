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
use core::owned::{Eq};
use std::time::Timespec;
use fit::{ ParFitable, FitOk, FitErr, FitTryFail, FitSysErr, DoFit, ParFitCommEndChan };
use std::json::{ Object };
use core::comm::{ stream, Port, Chan, SharedChan };
use core::task::{ spawn };

//	If there is a central component in Must, it is the Programmable Argument Relay (Par).

//	Think of a Par as single cube that receives arguments from the left, and then 
//	spawns a message for a Functionally Isolated Transaction (Fit) to perform some work.  
//	The Par handles all communication related to passing on valid results, or reporting errors.  
//	The Par will always spawn the DoFit request, but the Fit can decide whether or not to implement   
/
//  While the functional guts of the application reside in the Functionally Isolated Transactions,
//	the entire communication path will be defined and implemented through relays, linked through
//	by a strand with an initial goal to take a transaction through the relays that can reach a 
//	successful end, or bow out gracefully. 

//	I am trying to solve a few goals with this Programmable Argument Relay ( Par ):

//	*The guts of the communication system should be made abstract to day to day production programming.

//	*Use and encourage spawning.

//	*Specify and cap the amount of concurrent spawns allowed in each relay.
//		(note: It intuitively seems right that limiting concurrent spawns per relay to 20 or so would
//		serve as a good way to insure all other relays get adequate processor time.)

//	*Use core::comm and std::comm task communication ports and chans.

//	*Implement a transaction key that ties a request as its strand instructs a series of actions to be 
//	performed through communicating with relays holding fits that perform the grunt work.

//	The monitoring detail will automatically include the transaction id, a time-stamp, and the concurrent 
//	number of spawns active in the relay. Almost all of the error and performance monitoring will make its way to system 
//	indexes where they can be examined and linked to documentation about the monitored item.

//  Relay logic will be embedded strands, with each link contain ... more on strands later

//	ParT: holds a "live" instance of a Par
struct ParT {
	port: Chan<ParComm>,
	par: ~Par
}

struct Par {
	priv spawn_cap: uint,
	priv fit: ~ParFitable
}

#[deriving(Eq)]
enum ParComm {
	ParTrans( ~str, ~Object, fork: ~ChanFork), // ( t_key, args )
	ParCommEndChan
}
#[deriving(Eq)]
enum ToDo {
	RecvLeftChan,
	RecvFitChan,
	Yield
}

impl Par {

	fn new( spawn_cap: uint, fit: ~ParFitable ) -> ~Par {
	
		~Par {
			spawn_cap: spawn_cap,
			fit: fit
		}
	}
	
	fn connect( &self) -> Chan<ParComm> {
	
		let (right_port, right_chan) = stream();
		self.spawn_and_run( left_port );
		right_chan
	}
	
	priv fn spawn_and_run( &self, in_port: Port<ParComm>, mon_chan: Chan<MonComm>, par_chan: Chan<ParComm> ) {
	
		let spawn_cap = if self.spawn_cap > 0 { self.spawn_cap } else { 20u };
		let fit_key = self.fit.fit_key();
		let ( fit_port, fit_chan ) = self.fit.connect();
		let fit_chan = SharedChan( fit_chan );
		do spawn  {
			let mut current_spawns = 0u;
			loop {
				let to_do_list = {		
					let mut to_do = ~[];
					if current_spawns == spawn_cap { 
						to_do.push ( RecvFitChan );
					} else if current_spawns == 0u { 
						to_do.push ( RecvLeftChan );
					} else {
						if fit_port.peek() { 
							to_do.push ( RecvFitChan ); 
						}
						if left_port.peek() { 
							to_do.push ( RecvLeftChan ); 
						}
						if to_do.len() == 0 { 
							to_do.push( Yield ); 
						}				 
					}
					to_do
				};
				for to_do_list.each | to_do_item | {
					match *to_do_item {
						RecvLeftChan => {
							let new_req = left_port.recv();
							current_spawns += 1;
							match new_req {
								ParTrans( t_key, args, ( ok_par_chan, mon_chan ) => {
									//recording the start time of this fit
									let ts = std::time::at_utc( std::time::get_time() ).to_timespec();
									let t = FitStart( copy t_key, ts, copy fit_key );
									let fit_sh_ch = fit_chan.clone();
									//let mon_chan_sh = mon_chan.clone();
									do spawn {
										fit_sh_ch.send( DoFit( copy t_key, copy args ) );
										mon_chan.send( copy t );
									}
								}
								ParCommEndChan => { 
									//I am considering letting spawned tasks finish
									//but not in round 1
									let mon_chan_sh = mon_chan.clone();
									mon_chan_sh.send( MonCommEndChan );
									let right_chan_sh = right_chan.clone();
									right_chan_sh.send( ParCommEndChan );
									let fit_sh_ch = fit_chan.clone();
									fit_sh_ch.send( ParFitCommEndChan );
									return; 
								}
							}					
						}
						RecvFitChan => {
							let fit_val = fit_port.recv();
							let ts = std::time::at_utc( std::time::get_time() ).to_timespec();
							current_spawns -= 1;
							let mon_chan_sh = mon_chan.clone();
							let fit_key = copy fit_key;
							match fit_val {
								FitOk( t_key, args ) => {
									let right_chan_sh = right_chan.clone();
									do spawn {
										mon_chan_sh.send( FitEndOkay( copy t_key, ts, copy fit_key ) );
										right_chan_sh.send( ParTrans(  copy t_key, copy args ) );
									}
								}
								FitErr( t_key, args ) => {
									do spawn {
										mon_chan_sh.send( FitEndError( copy t_key, ts, copy fit_key, copy args ) );
									}
								}
								FitTryFail( t_key ) => {
									do spawn {
										mon_chan_sh.send( FitEndTryFail( copy t_key, ts, copy fit_key ) );
									}
								}
								FitSysErr( args ) => {
									do spawn {	
										io::println( ~"FitSysErr: " + std::json::to_pretty_str( &( Object(copy args) ) ) );
										mon_chan_sh.send( FitMonSysErr( ts, copy fit_key, copy args ) );
									}
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
}

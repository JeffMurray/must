//	Copyright 2013 Jeff Murray.

//	Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//	http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//	<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//	option. This file may not be copied, modified, or distributed
//	except according to those terms.

//	rustc --lib par.rs -L .
//	par.rs is tested in parts.rs 

#[link(name = "par", vers = "0.0")];

extern mod std;
extern mod extra;
extern mod fit;
extern mod must;
use must::{ Must };
use extra::time::Timespec;
use fit::{ DoFit, ParFitCommEndChan, ParFitComm, FitComm, FitArgs, FitErrs };
use std::comm::{ stream, Port, Chan, SharedChan, ChanOne, oneshot };
use std::task::{ spawn, yield };

//	A Par (Programmable Argument Relay) a Rust impl that with each request, takes some args and 
//	spawns a message for a Fit (Functionally Isolated Transaction) and waits for it to perform some work.  
//	The Par handles spawn control through a startup-designated maximum.
  
//	The Par will always spawn the DoFit request up to the concurrent spawn limit, but the Fit can decide
//	whether or not to respond to the spawed requests sequentially or concurrently.

//  While the functional guts of Must reside in Pars and Fits, the entire communication path will be defined 
//	through logic strands (see strand.rs).

//	I am trying to solve a few goals with this Programmable Argument Relay ( Par ):

//	*Encapsulate and encourage multi-processor spawning.

//	*Specify and cap the amount of concurrent spawns allowed in each relay.
//		(note: It intuitively seems right that limiting concurrent spawns per relay to 20 or so would
//		serve as a good way to insure all other relays get adequate processor time.)

//	*Take advantage of std::comm ports and chans.

//	*Natively collect monitoring detail that will include:
//  -start time: that the the Par calls send to the Fit.  
//	-number of seconds: that the Fit took to respond. ( for most high performance Fits, this should be zero and this Par is going to call it like it is ;)
//	-number of nanoseconds: remaining after adding the number of seconds, that it took for the fit to send back its reply  
//	-number of concurrent spawns: including this one, that were active in the relay at the time of starting the fit.

struct Par {
	priv spawn_cap: uint
}

enum ParInComm {
	ParTrans( ~FitArgs, ~Must, ChanOne<FitOutcome> ), // ( args, t_key, chan )
	ParCommEndChan( ChanOne<()> )
}

enum SpawnComm {
	SpawnDoFit( ~FitArgs, ~Must, SharedChan<ParFitComm> , ChanOne<FitOutcome>, SharedChan<()>, uint ) // args, t_key, fit_chan, home_chan, good_by_chan, spawns
}

struct FitOutcome {
	started: Timespec,
	span_sec: i32,
	span_nsec: i32,
	outcome: FitComm,
	spawns: uint
}

enum ToDo {
	RecvInPort,
	RecvGoodByPort,
	Yield
}

impl Par {

	pub fn new( spawn_cap: uint ) -> ~Par {
	
		~Par {
			spawn_cap: spawn_cap,
		}
	}
	
	pub fn connect( &self, fit_chan: Chan<ParFitComm> ) -> Result<Chan<ParInComm>, ~FitErrs> {
	
		let (in_port, in_chan) = stream();
		match self.spawn_and_run( in_port, fit_chan ) {
			Ok( _ ) => {
				Ok( in_chan )
			}
			Err( errs ) => {
				Err( errs ) 	
			}
		}
	}
	
	//This is the spawn that actually communicates with the fit
	priv fn go() -> Chan<SpawnComm> {
	
		let (in_port, in_chan): ( Port<SpawnComm>, Chan<SpawnComm>) = stream();
		do spawn {
			match in_port.try_recv().expect("par.rs g7wb0nQEtNVLlCMQ") {
				SpawnDoFit( args, t_key, fit_chan, home_chan, par_chan, spawns ) => {
					let start = extra::time::at_utc( extra::time::get_time() ).to_timespec();
					let ( p, c ) = oneshot();
					fit_chan.send( DoFit( copy args, copy t_key, c ) );
					let outcome =  p.try_recv().expect("par.rs dlqn8QjzHFQNbBr5");
					let end = extra::time::at_utc( extra::time::get_time() ).to_timespec();
					let mut sec_diff = end.sec - start.sec;
					let mut nsec_diff = end.nsec - start.nsec;
					if sec_diff > 0 { //I could not find a native timespan function at the time I did this
						if nsec_diff  < 0 { //First time I have done this math, hope it is right :)
							nsec_diff = 1000000000 + nsec_diff;
							sec_diff -= 1;
						}
					}
					home_chan.send ( FitOutcome {
						started: start,
						span_sec: sec_diff.to_i32(),
						span_nsec: nsec_diff.to_i32(),
						outcome: outcome,
						spawns: spawns
					} );		
					par_chan.send( () ); //sending a notice to decrement the spawn counter			
				}
			}
		}
		return in_chan; 
	}
	
	priv fn spawn_and_run( &self, in_port: Port<ParInComm>, fit_chan: Chan<ParFitComm> ) -> Result<bool, ~FitErrs> {
	
		let spawn_cap = if self.spawn_cap > 0 { self.spawn_cap } else { 20u };
		let ( good_by_port, good_by_chan ) = stream();
		let fit_ch = SharedChan::new( fit_chan );
		let good_by_chan = SharedChan::new( good_by_chan );
		do spawn  {
			let mut current_spawns = 0u;
			loop {
				let to_do_list = Par::to_do_list( in_port.peek(), good_by_port.peek(), current_spawns, spawn_cap );
				let mut break_again = false;
				for to_do_list.iter().advance | to_do_item | {
					match *to_do_item {
						Yield => {
							yield();
						},
						RecvGoodByPort => {
							good_by_port.try_recv().expect("par.rs  5T24V2yzQfVZd7sw"); // spawn is saying good-by
							current_spawns -= 1;	
						},
						RecvInPort => {
							let new_req = in_port.try_recv().expect("par.rs  NFZjeAbPep9Gx9r5");
							match new_req {
								ParTrans(  args, t_key, home_chan ) => {
									current_spawns += 1;
									let spawn_chan = Par::go();
									spawn_chan.send( SpawnDoFit( args, t_key, fit_ch.clone(), home_chan, good_by_chan.clone(), current_spawns ) );
								}
								ParCommEndChan( ack_chan ) => {
									while current_spawns > 0 {
										good_by_port.try_recv().expect("par.rs  mdnXvERN553g60zA"); // spawn is saying good-by
										current_spawns -= 1;	
									}
									fit_ch.send( ParFitCommEndChan );
									ack_chan.send( () );
									break_again = true;
									break; 
								}
							}					
						}				
					}
				}
				if break_again { break; }
			}
		}
		Ok( true )
	}
	
	priv fn to_do_list( in_peek: bool, spawn_peek: bool, current_spawns: uint, spawn_cap: uint ) -> ~[ToDo] {
	
		let mut to_do = ~[];
		if current_spawns == spawn_cap { 
			//println(~"Full: spawns = " + current_spawns.to_str() + " cap = " + spawn_cap.to_str());	
			to_do.push ( RecvGoodByPort );
		} else if current_spawns == 0u { 
			to_do.push ( RecvInPort );
		} else {
			if spawn_peek { 
				to_do.push ( RecvGoodByPort ); 
			}
			if in_peek { 
				to_do.push ( RecvInPort ); 
			}
			if to_do.len() == 0 { 
				to_do.push( Yield ); 
			}				 
		}
		to_do
	}
}

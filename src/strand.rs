//	Copyright 2013 Jeff Murray.

//	Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//	http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//	<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//	option. This file may not be copied, modified, or distributed
//	except according to those terms.
 
 #[link(name = "strand", vers = "1.0")];
 
//	rustc --lib strand.rs -L .
//	rustc strand.rs --test -o strand-tests -L .
//	./strand-tests

// Transactions and their arguments are monitored and shuttled around to ParTs by a StrandWalker,
// an ephemeral spawn, that is navigates a Strand of Logic to its end through a series of
// successes or nicely handled error processing, and to an end.

// I plan on making a lot of ParFitables, and I hope others will too.  With every new Fit, the
// number of ways they can all be combined will grow.

// A strand of logic can encoded and decoded to Json, and will be the backbone of a configurable 
// logic system for Must.

// Logic is an enum:
// OkErr( reg_key, error_strand ) => requests that the Fit associated with reg_key to be executed.
//		if the Fit returns Ok, the StrandWalker moves to the next logic in the strand
//		if the Fit returns Err, the StrandWalker makes the error strand its current strand
//			and begins to strand walk that.
// KeyMatch( arg_key, strand_map ) => does not call a Fit directly, rather it queries the arg_bank
//		for a value identified by arg_key, the mapped strands are then searched for a key matching
//		arg_bank, if a strand is found, the StrandWalker makes the matched strand the current strand, 
//		and begins walking it. 



extern mod std;
extern mod core;
use core::hashmap::linear::LinearMap;
use core::task::spawn;

type Strand = ~[~Logic];

enum Logic {
	OkErr( ~str, ~str ), // ( reg_key, error_strand )
	KeyMatch( ~str, ~StrandKeyMap ) // ( arg_key, strand_map )
}

struct StrandKeyMap {
	mapped_strands: MappedStrands,
	no_match_strand_key: ~str
}

struct StrandWalker; 

type MappedStrands = ~LinearMap<~str, ~str>;

enum LogicInComm {
	NextOk,
	NextErr,
	NextEnd
}

enum LogicOutComm {
	DoFit( ~str ),  //reg_fit_key, args
	EndOfStrand
}

impl StrandKeyMap {
	fn get_strand_key( key: ~str ) -> ~str {
		match strand_map.find( key ) {
			Some( strand_key ) => {
				copy *strand_key
			}
			None => {
				copy no_match_strand_key
			}
		}		
	}
}

impl StrandWalker {

	fn go( strand: Strand, arg_bank: Chan<JahMutReq> ) -> ( Port<LogicOutComm>, Chan<LogicInComm> ) {
		let ( walker_port, s1_chan ) = stream();
		let ( s2_port, walker_chan ) = stream();
		do spawn {
			let mut strand =  copy strand;
			let mut pos = 0u;
			loop {
				if pos == strand.len() {
					walker_chan.send( EndOfStrand );
					break;
				}
				{	let logic = copy strand[ pos ];
					match logic {
						OkErr( ok_fit_reg_key, _ ) => {
							walker_chan.send( DoFit( copy ok_fit_reg_key ) );
						}
						MatchKey( args_key, strand_map ) => {
							let val = { let ( c, p ) = oneshot::init();
								arg_bank.send( GetStr( copy args_key, c ) );
								recv_one( p )
								};
							let s_key = strand_map.get_strand_key( val );

						}
					}
				}
				let reply: LogicInComm = walker_port.recv();
				match reply {
					NextOk => { pos += 1; }
					NextErr => {
						let logic = copy strand[ pos ];
						strand = copy logic.err_strand;
						pos = 0;						
					}
					NextEnd => {
						break;
					}
				}
			}
		}
		( s2_port, s1_chan )
	}
	fn get_strand( strand_key: ~str ) -> ~Strand {
		//The plan is to make a tool to assemble these in a multi-language kind of way
		//once the user interface us up and running
		match strand_key {
			//Add document
			~"0loMIC2O3UW1yuTW" => { ~[ OkErr( ~"S68yWotrIh06IdE8", ~"DROOg7Vt2GXiVl00" ) ] }
			//Error println to terminal
			~"DROOg7Vt2GXiVl00" => { ~[ OkErr( ~"EHR6DFySwtSHzlb2", ~"fUhzdaBaEYITxXET" ) ] }
			//Empty strand
			~"fUhzdaBaEYITxXET" => { ~[] }
		}
	}
}

#[test]
fn various() {
	let fk1 = ~"fk1";
	let fk2 = ~"fk2";
	let fk3 = ~"fk3";
	
	let strand1 = ~[ 
		Logic { ok_key: copy fk1, err_strand: ~[] },  
		Logic { ok_key: copy fk2, err_strand: ~[Logic { ok_key: copy fk3, err_strand: ~[] }, 
		Logic { ok_key: ~"fk4", err_strand: ~[] }] }
		];
		
	let ( port, chan ) = StrandWalker::go( strand1 );
	
	match port.recv() {
		DoFit( key ) => { assert!( key == fk1 ) }
		EndOfStrand	=> { fail!() }	
	}
	
	chan.send( NextOk );
	
	match port.recv() {
		DoFit( key ) => { assert!( key == fk2 ) }
		EndOfStrand	=> { fail!() }	
	}	

	chan.send( NextErr );
	
	match port.recv() {
		DoFit( key ) => { assert!( key == fk3 ) }
		EndOfStrand	=> { fail!() }	
	}
	
	chan.send( NextOk );
	
	match port.recv() {
		DoFit( key ) => { assert!( key == ~"fk4" ) }
		EndOfStrand	=> { fail!() }	
	}
		
	chan.send( NextOk );
	
	match port.recv() {
		DoFit( _ ) => { fail!() }
		EndOfStrand	=> {  }	
	}	
}	


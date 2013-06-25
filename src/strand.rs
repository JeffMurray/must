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

extern mod std;
extern mod core;
extern mod jah_mut;
extern mod bootstrap;
use jah_mut::{ JahMutReq, GetStr, JahMut, Release, InsertOrUpdate };
use std::json::{ Object, String, ToJson };
use bootstrap::{ Bootstrap };
use core::comm::{ oneshot, recv_one };
use core::hashmap::linear::LinearMap;
use core::task::spawn;

// Transactions and their arguments are monitored and shuttled around to ParTs by an ephemeral spawn 
// called StrandWalker::go The StrandWalker takes a Strand of Logic through a series of
// successes or nicely handled error responses to the end of a final strand it travels across.

// I plan on making a lot of ParFitables, and I hope others will too.  With every new Fit, the
// number of ways they can be combined will grow, in ways the Fit writer may not have thought of.  

// A strand of logic can be encoded and decoded to Json, and will enable a configurable 'document-level'
// logic system for Must.  For instance one Fit might be able to produce digital signatures, but 
// the logic strands would only include it in instances where it was called for.

// Logic in Must is an enum and so far it has:
// OkErr( reg_key, error_strand ) => requests that the Fit associated with reg_key be executed.
//		if the Fit returns Ok, the StrandWalker is sent NextOk and moves to the next logic in the 
//			strand
//		if the Fit returns Err, the StrandWalker takes the error strand and begins to walk that.
// KeyMatch( arg_key, strand_map ) => does not call a Fit directly, rather it queries the arg_bank
//		for a value identified by arg_key, the mapped strands are then searched for a key matching
//		arg_bank, if a strand is found, the StrandWalker makes the matched strand the current strand, 
//		and begins walking it.
// It is possible there is a good reason for logic types other than OkErr and KeyMatch, but given 
// that Fits can produce keys using there own key value logic, and that strands can be configured 
// to use those fits and match those keys, I would like to hear that reason.

type Strand = ~[Logic];

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
	Err( ~Object ),
	EndOfStrand
}

impl StrandKeyMap {

	fn get_strand_key( &self, key: ~str ) -> ~str {
	
		match self.mapped_strands.find( &key ) {
			Some( strand_key ) => {
				copy *strand_key
			}
			None => {
				copy self.no_match_strand_key
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
				let mut break_again = false;
				loop {
					let logic = copy strand[ pos ];
					match logic {
						OkErr( ok_fit_reg_key, _ ) => {
							walker_chan.send( DoFit( copy ok_fit_reg_key ) );
							break;
						}
						KeyMatch( args_key, strand_map ) => {
							let val = { 
								let ( c, p ) = oneshot::init();
								arg_bank.send( GetStr( copy args_key, c ) );
								recv_one( p )
								};
							match val {
								Some( key ) => { 
									strand = copy StrandWalker::get_strand( strand_map.get_strand_key( key ) );
									pos = 0;
								} None => {
									walker_chan.send( Err( Bootstrap::spec_rule_error( Bootstrap::arg_rule_key_arg_must_exist(), copy args_key, ~"euG3S9MfQQmlRe6B", ~"strand.rs" ) ) );
									break_again = true;
									break;
								}
							}
						}
					}
				}
				if break_again { break; }
				let reply: LogicInComm = walker_port.recv();
				match reply {
					NextOk => { pos += 1; }
					NextErr => {
						match copy strand[ pos ] {
							OkErr( _ , err_strand_key ) => {
								strand = copy StrandWalker::get_strand( err_strand_key );
								pos = 0;
							} 
							_ => {
								walker_chan.send( Err( Bootstrap::spec_rule_error( Bootstrap::arg_rule_key_arg_must_exist(), ~"not OkErr", ~"qu0Mbo5S23uVXyXy", ~"strand.rs" ) ) );
								break;								
							}							
						}
					}
					NextEnd => {
						break;
					}
				}
			}
		}
		( s2_port, s1_chan )
	}
	
	priv fn get_strand( strand_key: ~str ) -> Strand {
	
		//The plan is to make a tool to assemble these in a multi-language kind of way
		//once the user interface us up and running
		match strand_key {
			//Add document
			~"0loMIC2O3UW1yuTW" => { ~[ OkErr( ~"S68yWotrIh06IdE8", ~"DROOg7Vt2GXiVl00" ) ] }
			//Error println to terminal
			~"DROOg7Vt2GXiVl00" => { ~[ OkErr( ~"EHR6DFySwtSHzlb2", ~"fUhzdaBaEYITxXET" ) ] }
			//Empty strand
			~"fUhzdaBaEYITxXET" => { ~[] }
			//Default
			_ => { ~[] }
		}
	}
}

#[test]
fn various() {

	let mut mapped_strands = ~LinearMap::new();
	mapped_strands.insert( ~"ants_are", ~"0loMIC2O3UW1yuTW" );
	mapped_strands.insert( ~"my_friends", ~"DROOg7Vt2GXiVl00" ); 	
	let strand_map = ~StrandKeyMap {
		mapped_strands: copy mapped_strands,
		no_match_strand_key: ~"fUhzdaBaEYITxXET"
		};
	let strand1 = ~[ 
		OkErr( ~"Fit 1", ~"DROOg7Vt2GXiVl00" ),  
		OkErr( ~"Fit 2", ~"DROOg7Vt2GXiVl00" ),
		KeyMatch( ~"some_arg_key", strand_map ),
		OkErr( ~"Fit 3", ~"DROOg7Vt2GXiVl00" )
		];
	
	//Setup an arg_bank
	let (admin_port, admin_chan ) = stream();
	let (user_port, user_chan ) = stream();
	JahMut::connect_new( user_port, admin_port );			
	admin_chan.send ( InsertOrUpdate( ~"some_arg_key", String( ~"ants_are" ).to_json() ) );
	let ( port, chan ) = StrandWalker::go( strand1, user_chan );
	match port.recv() {
		DoFit( key ) => { assert!( key == ~"Fit 1" ) }
		Err( err ) => { fail!() }
		EndOfStrand	=> { fail!() }	
	}
	chan.send( NextOk );
	match port.recv() {
		DoFit( key ) => { assert!( key == ~"Fit 2" ) }
		Err( err ) => { fail!() }		
		EndOfStrand	=> { fail!() }	
	}	
	
	chan.send( NextOk );
	match port.recv() {
		DoFit( key ) => { assert!( key == ~"S68yWotrIh06IdE8" ) }
		Err( err ) => { io::println( std::json::to_pretty_str(&(err.to_json()))); fail!() }		
		EndOfStrand	=> { fail!() }	
	}	
	
	chan.send( NextErr );
	match port.recv() {
		DoFit( key ) => { assert!( key == ~"EHR6DFySwtSHzlb2" ) } // the first fit in the error strand
		Err( err ) => { io::println( std::json::to_pretty_str(&(err.to_json()))); fail!() }		
		EndOfStrand	=> { fail!() }	
	}
	chan.send( NextOk );
	match port.recv() {
		DoFit( _ ) => { fail!() }
		Err( err ) => { fail!() }		
		EndOfStrand	=> {  }	
	}	
	admin_chan.send( Release );
}	


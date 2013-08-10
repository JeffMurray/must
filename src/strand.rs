//	Copyright 2013 Jeff Murray.

//	Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//	http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//	<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//	option. This file may not be copied, modified, or distributed
//	except according to those terms.
 
 #[link(name = "strand", vers = "0.0")];
 
//	rustc --lib strand.rs -L .
//	rustc strand.rs --test -o strand-tests -L .
//	./strand-tests

extern mod std;
extern mod extra;
extern mod bootstrap;
use extra::json::{ Object, String, ToJson };// String and ToJson are use in unit tests
use bootstrap::{ Bootstrap };
use std::comm::{ oneshot, ChanOne };
use std::hashmap::HashMap;
use std::task::spawn;

// Transactions and their arguments are monitored and shuttled around to ParTs by an ephemeral spawn 
// created with Ribosome::connect. The Ribosome takes a Strand of Logic through a series of
// successes or nicely handled error responses to the end of the final strand.

// I plan on making a lot of Fits, and I hope others will too.  With every new Fit, the
// number of ways they can be combined will grow.  

// A strand of logic can be encoded and decoded to Json, and will enable a configurable 'document-level'
// logic system for Must.  For instance one Fit might be able to produce digital signatures, but 
// the logic strands would only include it in instances where it was called for.  New logic can be applied
// without re-compiling, but adding or fixing fits requires a re-compile, at the moment. 

// Logic in Must is an enum and so far it has:
// OkErr( reg_key, error_strand ) => requests that the Fit associated with reg_key be executed.
//		if the Fit returns Ok, the Ribosome is sent NextOk and moves to the next logic in the 
//			strand
//		if the Fit returns Err, the StrandWalker takes the error strand and begins to walk that.
// KeyMatch( arg_key, strand_map ) => does not call a Fit directly, rather it queries the arg_bank
//		for a value identified by arg_key, the mapped strands are then searched for a key matching
//		arg_bank, if a strand is found, the Ribosome makes the matched strand the current strand, 
//		and begins walking it.
// It is possible there is a good reason for logic types other than OkErr and KeyMatch, but given 
// that Fits can produce keys using there own key value logic, and that strands can be configured 
// to use those fits and match those keys, keeping new Logic enums to a minimum ought to be possible.

type Strand = ~[Logic];

enum Logic {
	OkErr( ~str, ~str ), // ( reg_key, error_strand )
	KeyMatch( ~str, ~StrandKeyMap ) // ( arg_key, strand_map )
}


struct StrandKeyMap {
	mapped_strands: MappedStrands,
	no_match_strand_key: ~str
}

// In honor of our handy little friends
struct Ribosome; 

type MappedStrands = ~HashMap<~str, ~str>; // match_key, strand_key
//type LiveStrands = ~HashMap<~str, Strand>; // match_key, strand

enum LogicInComm {
	NextOk,
	NextErr,
	NextEnd
}


enum LogicOutComm {
	//Fit = Functionally Isolated Transaction
	DoFit( ~str ),  // ( reg_key )
	LogicErr( ~Object ),
	GetArgStr( ~str, ChanOne<Option<~str>> ),
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

impl Ribosome {

	pub fn connect( strand_key: ~str ) -> ( Port<LogicOutComm>, Chan<LogicInComm> ) {

		// Finds the Strand of Logic using strand_key and then calls Parfitables and accumulates an arg_bank
		// that can be used to satisfy jah_spec requirements of Fits while working its way over the strands.

		let ( rib_port, s1_chan ) = stream();
		let ( s2_port, rib_chan ) = stream();
		do spawn {
			let mut strand =  Ribosome::get_strand( copy strand_key );
			let mut pos = 0u;
			loop {
				if pos == strand.len() {
					rib_chan.send( EndOfStrand );
					break;
				}
				let logic = copy strand[ pos ];
				match logic {
					OkErr( ok_fit_reg_key, err_strand_key ) => {
						rib_chan.send( DoFit( copy ok_fit_reg_key ) );
						let reply: LogicInComm = rib_port.recv();
						match reply {
							NextOk => { pos += 1; }
							NextErr => {
								strand = Ribosome::get_strand( err_strand_key );
								pos = 0;							
							}
							NextEnd => {
								break;
							}
						}						
					}
					KeyMatch( args_key, strand_map ) => {
						let val = { 
							let ( p, c ) = oneshot();
							rib_chan.send( GetArgStr( copy args_key, c ) );
							p.recv()
							};
						match val {
							Some( key ) => { 
								strand = Ribosome::get_strand( strand_map.get_strand_key( key ) );
								pos = 0;
							} None => {
								rib_chan.send( LogicErr( Bootstrap::spec_rule_error( Bootstrap::arg_rule_key_arg_must_exist(), copy args_key, ~"euG3S9MfQQmlRe6B", ~"strand.rs" ) ) );
								break;
							}
						}
					}
				}
			}
		}
		( s2_port, s1_chan )
	}
	
	priv fn get_strand( strand_key: ~str ) -> Strand {

		//The plan is to make a tool to assemble strands in a multi-language kind of way
		//once the user interface us up and running
		match strand_key {
			//the start strand in Must Document System Object
			~"UWmoVWUMfKsL8oyr" => { ~[ {
				let mut mapped_strands =  ~HashMap::new();
					//I can only do add document with this spec at the moment
					mapped_strands.insert( Bootstrap::spec_add_doc_key(), ~"0loMIC2O3UW1yuTW" );

					let strand_map = ~StrandKeyMap {
							mapped_strands: copy mapped_strands,
							no_match_strand_key: ~"DROOg7Vt2GXiVl00"
						};
					KeyMatch( ~"spec_key", strand_map )			
				 	} ] }			
			//Add document
			~"0loMIC2O3UW1yuTW" => { ~[ 
				OkErr( Bootstrap::doc_slice_prep_key(), ~"DROOg7Vt2GXiVl00" ),
				OkErr( Bootstrap::file_append_slice_key(), ~"DROOg7Vt2GXiVl00" ) // next step is index services
				//OkErr( Bootstrap::file_get_slice_key(), ~"DROOg7Vt2GXiVl00" ),
				//OkErr( Bootstrap::err_fit_key(), ~"DROOg7Vt2GXiVl00" ) //output to screen
				] }
			//Error output to terminal
			~"DROOg7Vt2GXiVl00" => { ~[ OkErr( Bootstrap::err_fit_key(), ~"fUhzdaBaEYITxXET" ) ] }
			//Empty strand
			~"fUhzdaBaEYITxXET" => { ~[] }
			~"o88KanesoJ6J19uN" => { 
				let mut mapped_strands = ~HashMap::new();
				mapped_strands.insert( ~"ants_are", ~"0loMIC2O3UW1yuTW" );
				mapped_strands.insert( ~"my_friends", ~"DROOg7Vt2GXiVl00" ); 	
				let strand_map = ~StrandKeyMap {
					mapped_strands: copy mapped_strands,
					no_match_strand_key: ~"fUhzdaBaEYITxXET"
				};
				~[ //test thread
				OkErr( ~"Fit 1", ~"DROOg7Vt2GXiVl00" ),  
				OkErr( ~"Fit 2", ~"DROOg7Vt2GXiVl00" ),
				KeyMatch( ~"some_arg_key", strand_map ),
				OkErr( ~"Fit 3", ~"DROOg7Vt2GXiVl00" ) ] }
			// TestStately
			~"tuUZAYq5uby19tYG" => {
				~[OkErr( Bootstrap::stately_tester_key(), ~"fUhzdaBaEYITxXET" )]
			}
			//Default
			_ => { ~[] }
		}
	}
}

#[test]
fn various() {

	//Setup an arg_bank
	let mut arg_bank = ~HashMap::new();			
	arg_bank.insert( ~"some_arg_key", String( ~"ants_are" ).to_json()  );
	let ( port, chan ) = Ribosome::connect( ~"o88KanesoJ6J19uN" );
	match port.recv() {
		DoFit( key ) => { assert!( key == ~"Fit 1" ) }
		LogicErr( err ) => { std::io::println( extra::json::to_pretty_str(&(err.to_json()))); fail!() }
		GetArgStr( arg_key, chan ) => { std::io::println( arg_key ); fail!() }
		EndOfStrand	=> { fail!() }	
	}
	chan.send( NextOk );
	match port.recv() {
		DoFit( key ) => { assert!( key == ~"Fit 2" ) }
		LogicErr( err ) => { std::io::println( extra::json::to_pretty_str(&(err.to_json()))); fail!() }	
		GetArgStr( arg_key, chan ) => { std::io::println( arg_key ); fail!() }	
		EndOfStrand	=> { fail!() }	
	}	
	chan.send( NextOk );
	match port.recv() {
		DoFit( key ) => { println( key ); fail!(); }
		LogicErr( err ) => { std::io::println( extra::json::to_pretty_str(&(err.to_json()))); fail!() }		
		GetArgStr( arg_key, chan ) => {
			match arg_bank.find( &arg_key ) {
				Some( val ) => {
					let val = copy *val;
					match val {
						String( s ) => {
							chan.send( Some( copy s ) );
						} _ => fail!()
					}	
				}
				None => {
					chan.send( None );
				}
			}
		}
		EndOfStrand	=> { fail!() }	
	}
	match port.recv() {
		DoFit( key ) => { println( key ); assert!( key == Bootstrap::doc_slice_prep_key() ) }
		LogicErr( err ) => { std::io::println( extra::json::to_pretty_str(&(err.to_json()))); fail!() }	
		GetArgStr( arg_key, chan ) => { std::io::println( arg_key ); fail!() }	
		EndOfStrand	=> { fail!() }	
	}
	chan.send( NextErr );
	match port.recv() {
		DoFit( key ) => { assert!( key == Bootstrap::err_fit_key() ) } // the first fit in the error strand
		LogicErr( err ) => { std::io::println( extra::json::to_pretty_str(&(err.to_json()))); fail!() }		
		GetArgStr( arg_key, chan ) => { std::io::println( arg_key ); fail!() }
		EndOfStrand	=> { fail!() }	
	}
	chan.send( NextOk );
	match port.recv() {
		DoFit( key ) => { std::io::println( key ); fail!() }
		LogicErr( err ) => { std::io::println( extra::json::to_pretty_str(&(err.to_json()))); fail!() }		
		GetArgStr( arg_key, chan ) => { std::io::println( arg_key ); fail!() }
		EndOfStrand	=> {  }	
	}
}


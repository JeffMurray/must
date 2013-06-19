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

enum LogicInComm {
	NextOk,
	NextErr,
	NextEnd
}

enum LogicOutComm {
	DoFit( ~str ),  //reg_fit_key, args
	EndOfStrand
}

type Strand = ~[Logic];
struct Logic {
	ok_key: ~str, 
	err_strand: Strand
}

struct StrandWalker; 

impl StrandWalker {

	fn go( strand: Strand ) -> ( Port<LogicOutComm>, Chan<LogicInComm> ) {
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
					walker_chan.send( DoFit( copy logic.ok_key  ) );
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


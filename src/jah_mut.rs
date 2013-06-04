//	Copyright 2013 Jeff Murray.

//	Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//	http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//	<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//	option. This file may not be copied, modified, or distributed
//	except according to those terms.

//	Here is a simple use for a rust DuplexStream.
//	I figured out I did not need a long lived mutable map for
//	the purpose I started writing this, but I went ahead and
//	finished because I wanted to get a working communication model
//	to use in the Programmable Argument Relay that I am
//	working on at the moment.


//	rustc --lib jah_mut.rs -L .
//	rustc jah_mut.rs --test -o jah_mut-tests -L .
//	./jah_mut-tests

#[link(name = "jah_mut", vers = "1.0")];

extern mod std;
extern mod core;
use std::json ::{Json, String};
use core::hashmap::linear::LinearMap;
use core::task::spawn;
use std::comm::DuplexStream;
use core::option::{Some, None};

enum JahMutReq {
	InsertOrUpdate( ( ~str, Json ) ),
	GetJson( ~str ),
	GetMapCopy,
	Remove( ~str ),
	EndChan
}

enum JahMutGetRep {
	KeyNotFound,
	Value( Json ),
	MapCopy( ~LinearMap<~str, Json> )
}

struct JahMut;

impl JahMut {
	
	fn connect_new() -> DuplexStream< JahMutReq, JahMutGetRep > {
	
		let (parent, child) = DuplexStream();
		JahMut::spawn_task(parent);
		child
	}
	
	priv fn spawn_task(parent: DuplexStream<  JahMutGetRep, JahMutReq > ) {
	
		do spawn {
			let mut map = ~LinearMap::new();
			loop {
				match parent.recv() {
					GetJson( key ) => {
						match map.find( &key ) {
							Some( json_value ) => {
								parent.send( Value( copy *json_value ) );
							}
							None => {
								parent.send( KeyNotFound );
							}
						}
					}
					GetMapCopy => {
						parent.send( MapCopy( copy map ) );
					}
					Remove( key ) => {
						if map.contains_key( &key ) {
							map.remove( &key );
						}						
					}
					EndChan	=> {
						break;
					}
					InsertOrUpdate( (key, val) ) => {
						if map.contains_key( &key ) {
							map.remove( &key );
						}
						map.insert( key, val );									
					}				
				}
			}
		}	
	}
}

#[test]
fn test_insert_or_update(){

	let dplx = JahMut::connect_new();
	dplx.send( InsertOrUpdate( ( ~"is", String( ~"ought" ) ) ) );
	dplx.send( GetJson( ~"is" ) );
	match dplx.recv() {
		Value( val ) => {
			match val {
				String( val ) => {
					assert!( val == ~"ought");
				}, _ => fail!()
			}
		}, _ => fail!()
	}
	dplx.send( InsertOrUpdate( ( ~"is", String( ~"not" ) ) ) );
	dplx.send( GetJson( ~"is" ) );
	match dplx.recv() {
		Value( val ) => {
			match val {
				String( val ) => {
					assert!( val == ~"not");
				}, _ => fail!()
			}
		}, _ => fail!()
	}	
	dplx.send( EndChan );
}

#[test]
fn test_remove(){

	let dplx = JahMut::connect_new();
	dplx.send( InsertOrUpdate( ( ~"is", String( ~"ought" ) ) ) );
	dplx.send( GetJson( ~"is" ) );
	match dplx.recv() {
		Value( val ) => {
			match val {
				String( val ) => {
					assert!( val == ~"ought");
				}, _ => fail!()
			}
		}, _ => fail!()
	}
	dplx.send( Remove( ~"is" ) );
	dplx.send( GetJson( ~"is" ) );
	match dplx.recv() {
		KeyNotFound => {
			assert!( true );
		}, _ => fail!()
	}	
	dplx.send( EndChan );
}

#[test]
fn test_get_map(){

	let dplx = JahMut::connect_new();
	dplx.send( InsertOrUpdate( ( ~"is", String( ~"ought" ) ) ) );
	dplx.send( GetJson( ~"is" ) );
	match dplx.recv() {
		Value( val ) => {
			match val {
				String( val ) => {
					assert!( val == ~"ought");
				}, _ => fail!()
			}
		}, _ => fail!()
	}
	
	dplx.send( GetMapCopy );
	match dplx.recv() {
		MapCopy( map ) => {
			match map.find( &~"is" ) {
				Some( val ) => {
					match copy *val {
						String( s ) => {
							assert!( s == ~"ought" );
						},_ => fail!()
					}
				}, None => fail!()
			}
		},_ => fail!()
	}
	dplx.send( EndChan );	
}


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
//	to use in the Programmable Argument Relay.


//	rustc --lib jah_mut.rs -L .
//	rustc jah_mut.rs --test -o jah_mut-tests -L .
//	./jah_mut-tests

#[link(name = "jah_mut", vers = "1.0")];

extern mod std;
extern mod extra;
extern mod jah_args;
extern mod fit;
use jah_args::{ JahArgs };
use fit::{ FitArgs };
use extra::json ::{ Json, ToJson, String, Number, Boolean };
use std::hashmap::HashMap;
use std::task::{spawn, yield};
use std::option::{Some, None};
use std::comm::{ChanOne, Chan, Port, stream, oneshot, recv_one}; // oneshot and recv_one are used with unit tests

enum JahMutReq {
	GetJson( ~str, ChanOne<Option<Json>> ),
	GetStr( ~str, ChanOne<Option<~str>>  ),
	GetFloat( ~str, ChanOne<Option<float>> ),
	GetBool( ~str, ChanOne<Option<bool>> ),	
	GetAttach( ~str, ChanOne<Option<~[u8]>> ),		
	GetMapCopy( ChanOne<Option< ~HashMap<~str, Json>>> )
}

enum JahMutAdmin {
	InsertOrUpdate( ~str, Json ),
	LoadMap( ~HashMap<~str, Json> ),
	MergeArgs( ~FitArgs ),
	Remove( ~str ),
	Release
}

enum JahMutToDo {
	RecvUserPort,
	RecvAdminPort,
	JahMutYield
}

struct JahMut;

impl JahMut {
	
	pub fn connect() -> ( Chan<JahMutReq>, Chan<JahMutAdmin> ) {
	
		let ( admin_port, admin_chan ) = stream();
		let ( user_port, user_chan ) = stream();
		JahMut::spawn_task( user_port, admin_port );
		( user_chan, admin_chan )
	}
	
	priv fn spawn_task(  user_port: Port<JahMutReq>, admin_port: Port<JahMutAdmin> ) {
	
		do spawn {
			let mut map = ~HashMap::new();
			let mut attached = ~HashMap::new();
			loop {
				let to_do = {
					let mut tds = ~[];
					if admin_port.peek() {
						tds.push(RecvAdminPort);
					}
					if user_port.peek() {
						tds.push(RecvUserPort);
					}
					if tds.len() == 0 {
						tds.push(JahMutYield);
					}
					tds };
				let mut release = false;
				for to_do.iter().advance | td | {		
					match *td {
						RecvAdminPort => {
							match admin_port.recv() {
								Remove( key ) => {
									if map.contains_key( &key ) {
										map.remove( &key );
									}						
								}
								InsertOrUpdate( key, val ) => {
									if map.contains_key( &key ) {
										map.remove( &key );
									}
									map.insert( key, val );									
								}	
								LoadMap( new_map ) => {
									map = new_map;
								}
								MergeArgs( args ) => {
									let keys = args.doc.arg_keys();
									for keys.iter().advance | key | {
										if map.contains_key( key ) {
											map.remove( key );
										}
										map.insert( copy *key, args.doc.get_json_val( copy *key ).to_json() );	
									}
									match args.doc.get_str( ~"attach" ) {
										Ok( atch_name ) => {
											if attached.contains_key( &atch_name ) {
												attached.remove( &atch_name );
											}
											attached.insert(  atch_name, copy args.attach );												
										} _ =>{}
									}
								}
								Release	=> {
									release = true;
									break;
								}		
							}
						}
						RecvUserPort => {
							match user_port.recv() {
								GetJson( key, chan ) => {
									match map.find( &key ) {
										Some( json_value ) => {
											chan.send( Some( copy *json_value ) );
										}
										None => {
											chan.send( None );
										}
									}
								}
								GetAttach( key, chan ) => {
									match attached.find( &key ) {
										Some( attach ) => {
											chan.send( Some( copy *attach ) );
										}
										None => {
											chan.send( None );
										}
									}
								}
								GetStr( key, chan ) => {
									match map.find( &key ) {
										Some( json_value ) => {
											match copy *json_value {
												String( value ) => {
													chan.send( Some( copy value ) );
												}
												_ => {
													chan.send( None );
												}
											}
										}
										None => {
											chan.send( None );
										}
									}
								}
								GetFloat( key, chan ) => {
									match map.find( &key ) {
										Some( json_value ) => {
											match *json_value {
												Number( value ) => {
													chan.send( Some( copy value ) );
												}
												_ => {
													chan.send( None );
												}
											}
										}
										None => {
											chan.send( None );
										}
									}
								}	
								GetBool( key, chan ) => {
									match map.find( &key ) {
										Some( json_value ) => {
											match *json_value {
												Boolean( value ) => {
													chan.send( Some ( copy value ) );
												}
												_ => {
													chan.send( None );
												}
											}
										}
										None => {
											chan.send( None );
										}
									}
								}															
								GetMapCopy( chan ) => {
									chan.send( Some( copy map ) );
								}	
							}			
						}
						JahMutYield => {
							yield();
						}
					}
				}
				if release {
					break;
				}
			}	
		}
	}
}

#[test]
fn test_insert_or_update(){

	let ( user_chan, admin_chan ) = JahMut::connect();
	admin_chan.send( InsertOrUpdate( ~"is", String( ~"ought" ) ) );
	match {	let ( p, c ) = oneshot();
		user_chan.send( GetJson( ~"is", c ) );
		recv_one( p )
	} {
		Some( val ) => {
			match val {
				String( val ) => {
					assert!( val == ~"ought");
				}, _ => fail!()
			}
		}, _ => fail!()
	}
	admin_chan.send( InsertOrUpdate( ~"is", String( ~"not" ) ) );
	match {	let ( p, c ) = oneshot();
		user_chan.send( GetJson( ~"is", c ) );
		recv_one( p )
	} {
		Some( val ) => {
			match val {
				String( val ) => {
					assert!( val == ~"not");
				}, _ => fail!()
			}
		}, _ => fail!()
	}	
	admin_chan.send( Release );
}

#[test]
fn test_data_conversions(){

	let ( user_chan, admin_chan ) = JahMut::connect();
	admin_chan.send( InsertOrUpdate( ~"is", String( ~"ought" ) ) );
	match {	let ( p, c ) = oneshot();
		user_chan.send( GetStr( ~"is", c ) );
		recv_one( p )
	} {
		Some( val ) => {
			assert!( val == ~"ought");
		}, _ => fail!()
	}
	match {	let ( p, c ) = oneshot();
		user_chan.send( GetFloat( ~"is", c ) );
		recv_one( p )
	} {
		Some( _ ) => {
			fail!();
		}, _ => {}
	}
	admin_chan.send( InsertOrUpdate( ~"answer", Number( 42f ) ) );	
	match {	let ( p, c ) = oneshot();
		user_chan.send( GetFloat( ~"answer", c ) );
		recv_one( p )
	} {
		Some( answer ) => {
			assert!( answer == 42f);
		}, _ => fail!()
	}
	admin_chan.send( InsertOrUpdate( ~"desert", Boolean( true ) ) );	
	match {	let ( p, c ) = oneshot();
		user_chan.send( GetBool( ~"desert", c ) );
		recv_one( p )
	} {
		Some( desert ) => {
			assert!( desert );
		}, _ => fail!()
	}
	admin_chan.send( InsertOrUpdate( ~"desert", Boolean( false ) ) );	
	match {	let ( p, c ) = oneshot();
		user_chan.send( GetBool( ~"desert", c ) );
		recv_one( p )
	} {
		Some( desert ) => {
			assert!( !desert );
		}, _ => fail!()
	}	
	admin_chan.send( Release );
}

#[test]
fn test_remove(){

	let ( user_chan, admin_chan ) = JahMut::connect();
	admin_chan.send( InsertOrUpdate( ~"is", String( ~"ought" ) ) );
	match { let ( p, c ) = oneshot();
		user_chan.send( GetJson( ~"is", c ) );
		recv_one( p )
	} {
		Some( val ) => {
			match val {
				String( val ) => {
					assert!( val == ~"ought");
				}, _ => fail!()
			}
		}, _ => fail!()
	}
	admin_chan.send( Remove( ~"is" ) );
	match { let ( p, c ) = oneshot();
		user_chan.send( GetJson( ~"is", c ) );
		recv_one( p )
	} {
		None => {
			assert!( true );
		}, _ => fail!()
	}	
	admin_chan.send( Release );
}

#[test]
fn test_get_attach(){

	let ( user_chan, admin_chan ) = JahMut::connect();
	let mut doc = ~HashMap::new();
	doc.insert( ~"attach", String( ~"prize" ).to_json() );
	admin_chan.send( MergeArgs( ~FitArgs{ doc: doc, attach: ~[1,2,3,4] } ) );
	
	// check Some
	match { let ( p, c ) = oneshot();
		user_chan.send( GetAttach( ~"prize", c ) );
		recv_one( p )
	} {
		Some( val ) => {
			assert!( val == ~[1,2,3,4] );
		}, _ => fail!()
	}
	
	// check None
	match { let ( p, c ) = oneshot();
		user_chan.send( GetAttach( ~"attach", c ) );
		recv_one( p )
	} {
		Some( _ ) => {
			fail!();
		}, _ => {}
	}	

	admin_chan.send( Release );	
}

#[test]
fn test_get_map(){

	let ( user_chan, admin_chan ) = JahMut::connect();
	admin_chan.send( InsertOrUpdate( ~"is", String( ~"ought" ) ) );
	match { let ( p, c ) = oneshot();
		user_chan.send( GetJson( ~"is", c ) );
		recv_one( p )
	} {
		Some( val ) => {
			match val {
				String( val ) => {
					assert!( val == ~"ought");
				}, _ => fail!()
			}
		}, _ => fail!()
	}
	match { let ( p, c ) = oneshot();
		user_chan.send( GetMapCopy( c ) );
		recv_one( p )
	} {
		Some( map ) => {
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
	admin_chan.send( Release );	
}


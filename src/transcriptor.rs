//	Copyright 2013 Jeff Murray.

//	Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//	http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//	<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//	option. This file may not be copied, modified, or distributed
//	except according to those terms.
 
 #[link(name = "transcriptor", vers = "0.0")];  
 
//	rustc --lib transcriptor.rs -L . -L fits
//	tested in must_bank.rs

extern mod std;
extern mod extra;
extern mod jah_spec;
extern mod jah_args;
extern mod par;
extern mod fit;
extern mod bootstrap;
extern mod strand;
extern mod must;
extern mod parts;
use jah_args::{ JahArgs, MissingKey, WrongDataType };
use jah_spec::{ JahSpec };
use par::{ FitOutcome, ParTrans };
use fit::{ FitOk, FitErr, FitSysErr, FitErrs, FitArgs };
use parts::{ ParTInComm, GetParTChan, ParTChan, ParTErr };
use must::{ Must };
use strand::{ Ribosome, DoFit, NextErr, NextOk, EndOfStrand, LogicErr, GetArgStr, LogicInComm, NextEnd };
use extra::json::{ Object, String, ToJson, Json }; // , to_pretty_str
use bootstrap::{ Bootstrap };
use std::comm::{ oneshot, stream, SharedChan };
use std::hashmap::HashMap;
use std::task::{ spawn };

struct Transcriptor;

impl Transcriptor {

	pub fn connect( kickoff_strand_key: ~str, t_key: Must ) -> Chan<((~FitArgs, SharedChan<ParTInComm>, SharedChan<Result<~FitArgs, ~FitErrs>>))> {
	
		let ( start_port, start_chan ) = stream();
		do spawn {
			
			let kickoff_strand_key = copy kickoff_strand_key;
			let t_key = ~copy t_key;  //  Must will eventually be transcribing status info with this key 
			let ( fit_args, parts_chan, goodby_chan )
				: (~FitArgs, SharedChan<ParTInComm>, SharedChan<Result<~FitArgs, ~FitErrs>>) 
				= start_port.try_recv().expect("transcription.rs giaI4MbRd9VbpRPe");
			if Transcriptor::incoming_spec_ok( &fit_args.doc, goodby_chan.clone() ) {
				let mut arg_bank: ~HashMap<~str, Json> = ~HashMap::new();
				let mut attached: ~HashMap<~str, ~[u8]> = ~HashMap::new();
				let ( rib_port, rib_chan ) = Ribosome::connect( kickoff_strand_key );
				Transcriptor::merge_args( &fit_args, &mut arg_bank, &mut attached );
				loop {
					match rib_port.try_recv().expect("transcription.rs XX7PlN67cnxOr9ku")  {
						GetArgStr( key, chan ) => {
							match arg_bank.get_str( key ) { 
								Ok( val ) => {
									chan.send( Some( val ) );
								}
								Err( _ ) => {
									chan.send( None );
								}
							}
						}
						LogicErr( errs ) => {
							goodby_chan.send( Err( FitErrs::from_objs( ~[Bootstrap::reply_error_trace_info( ~"transcriptor.rs", ~"OYKXYUpuTz14Ew0m" ),errs] ) ) ); 
						} 
						EndOfStrand	=> { 
							match Transcriptor::extract_args( &mut arg_bank, &mut attached ) {
								Ok( args ) => {
									goodby_chan.send( Ok( args ) );
								}
								Err( errs ) => {
									goodby_chan.send( Err( errs.prepend_err( Bootstrap::reply_error_trace_info( ~"transcriptor.rs", ~"Xl5waGnIWTWPsw2G" ) ) ) );
								}						
							}
							break; 
						}
						DoFit( reg_key ) => { 
							match Transcriptor::do_fit( reg_key, &mut arg_bank , &mut attached, parts_chan.clone(), copy t_key ) {
								Ok( signal ) => {
									rib_chan.send( signal );
								}
								Err( errs ) => {
									//The fit did not even get called if we are here
									rib_chan.send( NextEnd );
									goodby_chan.send( Err( errs.prepend_err( Bootstrap::reply_error_trace_info( ~"transcriptor.rs", ~"zJ3goGbt6DOJWEEB") ) ) );
									break;
								}
							}
						}
					}
				}
			}
		}
		start_chan
	}
	
	priv fn extract_args( arg_bank: &mut ~Object, attached: &mut ~HashMap<~str, ~[u8]> ) -> Result<~FitArgs, ~FitErrs> {
		
		let spec_key = { //get the latest spec that was loaded in the arg bank
			match arg_bank.get_str( ~"spec_key" ) {
				Ok( spec_key ) => { spec_key }
				Err( err ) => {
					match err {
						MissingKey => {
							return Err( FitErrs::from_obj( Bootstrap::logic_error(Bootstrap::arg_spec_key_arg_must_exist(), ~"spec_key", ~"0vKBkZjRUMVei1QX", ~"transcriptor.rs" ) ) )
						}
						WrongDataType => {
							return Err( FitErrs::from_obj( Bootstrap::logic_error(Bootstrap::arg_rule_arg_must_be_string_key(), ~"spec_key", ~"QyKtHrBE8GXB0WEf", ~"transcriptor.rs" ) ) )
						}
					}
				} 
			}};
		match Transcriptor::speced_arg_excerpt( &Bootstrap::find_spec( spec_key ), arg_bank, attached ) {
			Ok( args ) => {
				Ok( args )
			}													  
			Err( errs ) => {
				Err( errs.prepend_err( Bootstrap::reply_error_trace_info( ~"transcriptor.rs", ~"P590aja1zCctfAVJ" ) ) )
			}
		}				
	}
	
	priv fn do_fit( reg_key: ~str, arg_bank: &mut ~Object, attached: &mut ~HashMap<~str, ~[u8]>, parts_chan: SharedChan<ParTInComm>, t_key: ~Must ) -> Result<LogicInComm, ~FitErrs>  {
	
		let args = {
			match Transcriptor::extract_args( arg_bank, attached ) {
				Ok( args ) => {
					args
				}
				Err( errs ) => {
					return Err( errs.prepend_err( Bootstrap::reply_error_trace_info( ~"transcriptor.rs", ~"xyWRC6uUDdAOK0TN" ) ) );
				}
			}};
		// get the Par chan and send args
		let fo: FitOutcome = {
			match { let ( p, c ) = oneshot();
				parts_chan.send( GetParTChan( copy reg_key, c ) ); // ChanOne<ParTOutComm>
				p.try_recv().expect("transcription.rs 9MTGxNZ0jqcUjUGK")
				} {	ParTChan( part_chan ) => { // ( part_chan ) ChanOne<ParInComm>
						let ( p2, c2 ) = oneshot();
						part_chan.send( ParTrans( args, copy t_key, c2 ) ); // ChanOne<ParTOutComm>
						p2.try_recv().expect("transcription.rs PmQMvaPfAOtDyZM6")
					} 
					ParTErr( err ) => {
						return Err( err.prepend_err( Bootstrap::reply_error_trace_info( ~"transcriptor.rs", ~"P590aja1zCctfAVJ" ) ) );
					}
			}};
		// Record the fit performance once the indexing system is up and running
		// update the arg_bank
		match copy fo.outcome {
			FitOk( rval ) => {
				match rval.doc.get_str( ~"spec_key" ) {
					Ok( key ) => {
						match JahSpec::check_args( &Bootstrap::find_spec( key ), &rval.doc ) {
							Ok( _ ) => {
								Transcriptor::merge_args( &rval, arg_bank, attached );
								Ok( NextOk )					
							}
							Err( errs ) => {
								let fit_errs = FitErrs::from_objs( errs);
								Transcriptor::merge_args( &~FitArgs::from_doc( fit_errs.to_args() ), arg_bank, attached );
								Ok( NextOk )								
							}
						}
					}
					Err( err_type ) => {
						let errs = {
							match err_type {
								MissingKey => {
									FitErrs::from_obj( Bootstrap::logic_error(Bootstrap::arg_spec_key_arg_must_exist(), ~"spec_key", ~"TWRUF69B4hv4v5Iz", ~"transcriptor.rs" ) )
								}
								WrongDataType => {
									FitErrs::from_obj( Bootstrap::logic_error(Bootstrap::arg_rule_arg_must_be_string_key(), ~"spec_key", ~"iwpCbbmXqKyvc9VL", ~"transcriptor.rs" ) )
								}
							}};
						Transcriptor::merge_args( &~FitArgs::from_doc( errs.to_args() ), arg_bank, attached );
						Ok( NextErr )														
					}
				}
			}
			FitErr( rval ) => {
				let doc = rval.to_args();
				Transcriptor::merge_args( &~FitArgs::from_doc( doc ), arg_bank, attached );
				Ok( NextErr )
			}
			FitSysErr( err ) => {
				Err( err )
			}
		}			
	} 
	
	priv fn merge_args( args: &~FitArgs, arg_bank: &mut ~Object, attached: &mut ~HashMap<~str, ~[u8]> ) {

		let keys = args.doc.arg_keys();
		for keys.iter().advance | key | {
			if arg_bank.contains_key( key ) {
				arg_bank.remove( key );
			}
			arg_bank.insert( copy *key, args.doc.get_json_val( copy *key ).to_json() );	
		}
		match args.doc.get_str( ~"attach" ) {
			Ok( atch_name ) => {
				if attached.contains_key( &atch_name ) {
					attached.remove( &atch_name );
				}
				attached.insert(  atch_name, copy args.attach );	
				
			} _ => {}
		}
	}		
		
	priv fn speced_arg_excerpt( spec: &~Object, arg_bank: &~HashMap<~str, Json>, attached: &~HashMap<~str, ~[u8]> )-> Result<~FitArgs, ~FitErrs> {
		
		let mut rval = ~HashMap::new();
		let keys = { 
			match JahSpec::allowed_keys( spec ) {
				Ok( keys ) => { keys } 
				Err( err ) => { return Err( FitErrs::from_objs( ~[Bootstrap::reply_error_trace_info( ~"transcriptor.rs", ~"RqTr8enRtmwjwWrf" )] + err ) ) }
				}};
		for keys.iter().advance | key | {
			match arg_bank.find( key ) {
				Some( arg_val ) => { 
					rval.insert( copy *key, copy *arg_val );
				}
				None => {}  // leaving the ramifications of this missing arg to the upcomming spec check	
			}
		}
		let attch = {
			match JahSpec::check_args( spec, &rval  ) {
				Ok( _ ) => {
					match rval.get_str(~"attach") {
						Ok( attached_name ) => {
							match attached.find( &attached_name ) {
								Some( attached_bytes ) => {
									copy *attached_bytes
								}
								None => {
									return Err( FitErrs::from_obj( Bootstrap::logic_error( Bootstrap::named_attachment_is_missing(), attached_name, ~"Kyzltdf11TRcTIiI", ~"transcriptor.rs" ) ) )
								}
							}
						}
						Err( _ ) => { //  not really an error, just no need to send an attachment
							~[]
						}
					}
				}
				Err( errs ) => {
					return Err( FitErrs::from_objs( ~[Bootstrap::reply_error_trace_info( ~"transcriptor.rs", ~"FHLGfPficrDnNzao" )] + errs ) );
				}	
			}};
		Ok( ~FitArgs{ doc: rval, attach: attch } )
	}	
	
	priv fn incoming_spec_ok( args: &~Object, gb_chan: SharedChan<Result<~FitArgs,~FitErrs>> ) -> bool {
	
		match args.get_str( ~"spec_key" ) {
			Ok( spec_key ) => {	
				match JahSpec::check_args( &Bootstrap::find_spec( spec_key ), args ) {
					Ok( _ ) => {
						true
					}
					Err( errs ) => {
						gb_chan.send( Err( FitErrs::from_objs( errs ).prepend_err( Bootstrap::reply_error_trace_info( ~"transcriptor.rs", ~"OAGw7LnsZ0j03d4W") ) ) );
						false
					}
				}
			}
			Err( reason ) => {
				match reason {
					MissingKey => {
						gb_chan.send( Err( FitErrs::from_obj( Bootstrap::spec_rule_error( Bootstrap::arg_spec_key_arg_must_exist(), ~"spec_key", Bootstrap::spec_jah_spec_corrupt_key() , ~"JRkY9TDbehfPPA2F") ) ) );
					}
					WrongDataType => {
						gb_chan.send( Err( FitErrs::from_obj( Bootstrap::spec_rule_error( Bootstrap::arg_rule_arg_must_be_string_key(), ~"spec_key", Bootstrap::spec_jah_spec_corrupt_key(), ~"rkO0NumJeuQy7Fnu") ) ) );
					}
				}
				false
			}
		}
	}		
	//t = transcription.
	priv fn make_t_key( t_key: Must ) -> ~Object {
	
		let mut rval= ~HashMap::new();
		rval.insert( ~"t_key", t_key.to_json() );
		rval.insert( ~"spec_key", String( ~"CelvpCNzHNiPPUKL" ) );		
		rval
	}
}
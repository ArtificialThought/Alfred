extern crate discord;

use std::env;
use std::path::Path;
use std::thread;
use discord::{Discord, State};
use discord::model::Event;

mod settings;
mod modules;
mod bot;



// Main program loop
pub fn main() {
    settings::load();
    modules::load();
    bot::connect();
    bot::join();
//LOAD MODULES
//CONNECT TO SERVER
//BIND TO CHANNELS
//JOIN VOICE CHANNEL


    // EVENT BASED LOOP!!!!!!!!!!!!!!!!
    // THREADED

}

fn warn<T, E: ::std::fmt::Debug>(result: Result<T, E>) {
	match result {
		Ok(_) => {},
		Err(err) => println!("[Warning] {:?}", err)
	}
}

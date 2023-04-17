#[macro_use]
extern crate log;

pub mod server;

use android_logger::Config;
use log::LevelFilter;
use std::{
    ffi::{c_char, CStr},
    thread,
};

#[no_mangle]
#[export_name = "Java_com_tsirysndr_songbirdlib_Songbird_00024Companion_start"]
pub extern "C" fn start() {
    android_logger::init_once(Config::default().with_max_level(LevelFilter::Trace));
    debug!(
        r#"
    __  ___           _      ____  __                     
   /  |/  /_  _______(_)____/ __ \/ /___ ___  _____  _____
  / /|_/ / / / / ___/ / ___/ /_/ / / __ `/ / / / _ \/ ___/
 / /  / / /_/ (__  ) / /__/ ____/ / /_/ / /_/ /  __/ /    
/_/  /_/\__,_/____/_/\___/_/   /_/\__,_/\__, /\___/_/     
                                       /____/             

A simple music player written in Rust"#
    );
    thread::spawn(|| {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        runtime.block_on(server::start_all()).unwrap();
    });
}

#[no_mangle]
#[export_name = "Java_com_tsirysndr_songbirdlib_Songbird_00024Companion_start_1blocking"]
pub extern "C" fn start_blocking(socket_path: *const c_char) {
    let c_str = unsafe {
        assert!(!socket_path.is_null());
        CStr::from_ptr(socket_path)
    };
    let socket_path = c_str.to_string_lossy().into_owned();

    android_logger::init_once(Config::default().with_max_level(LevelFilter::Trace));
    debug!(
        r#"
    __  ___           _      ____  __                     
   /  |/  /_  _______(_)____/ __ \/ /___ ___  _____  _____
  / /|_/ / / / / ___/ / ___/ /_/ / / __ `/ / / / _ \/ ___/
 / /  / / /_/ (__  ) / /__/ ____/ / /_/ / /_/ /  __/ /    
/_/  /_/\__,_/____/_/\___/_/   /_/\__,_/\__, /\___/_/     
                                       /____/             

A simple music player written in Rust"#
    );
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime
        .block_on(server::start_over_uds(socket_path))
        .unwrap();
}

#[no_mangle]
#[export_name = "Java_com_tsirysndr_songbirdlib_Songbird_00024Companion_example"]
pub extern "C" fn example() {
    android_logger::init_once(Config::default().with_max_level(LevelFilter::Trace));
    debug!("Hello Android!");
    debug!("this is a debug {}", "message");
    error!("this is printed by default");
}

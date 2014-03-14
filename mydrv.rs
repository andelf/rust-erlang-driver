#[feature(globs)];
#[allow(dead_code)];

#[crate_type="dylib"];

extern crate time;
extern crate extra;

use std::libc::*;
use std::cast::transmute;
use std::c_str::CString;
use std::mem::size_of;
use extra::c_vec::CVec;


use erl_driver::*;
// ~/repos/rust-bindgen/bindgen -match erl_drv_nif.h -match erl_driver.h  /usr/lib/erlang/usr/include/erl_driver.h > ~/works/rust/erl_driver.rs
mod erl_driver;

// flags
static PORT_CONTROL_FLAG_BINARY : c_int = (1 << 0);

static ERL_DRV_EXTENDED_MARKER : c_int = 0xfeeeeeed;
static ERL_DRV_EXTENDED_MAJOR_VERSION : c_int = 2;
static ERL_DRV_EXTENDED_MINOR_VERSION : c_int = 2;

static ERL_DRV_FLAG_USE_PORT_LOCKING : c_int = (1 << 0);


struct MyDrvData {
    start_time: time::Tm,
    port: ErlDrvPort,
}

extern "C" fn my_start(port: ErlDrvPort, command: *mut c_char) -> ErlDrvData {
    // alloc use erl driver function
    let mut my_data : &mut MyDrvData = unsafe { transmute(driver_alloc(size_of::<MyDrvData>() as ErlDrvSizeT)) };
    println!("calls my_start()");
    my_data.start_time = time::now();
    my_data.port = port;
    unsafe { set_port_control_flags(port, PORT_CONTROL_FLAG_BINARY) };
    let cmd = unsafe { CString::new(command as *c_char, false) };
    println!("\n(gommand => {:?})", cmd.as_str());
    unsafe { transmute(my_data) }
}

extern "C" fn my_stop(drv_data: ErlDrvData) {
    let mut my_data = unsafe { transmute::<ErlDrvData,&mut MyDrvData>(drv_data) };
    println!("start_time => {:?}", my_data.start_time.rfc822());
    println!("calls free");
    unsafe { driver_free(drv_data) };
}

// called when we have output from erlang to the port
extern "C" fn my_output(drv_data: ErlDrvData, buf: *mut c_char, len: ErlDrvSizeT) {
    let mut my_data = unsafe { transmute::<ErlDrvData,&mut MyDrvData>(drv_data) };
    let what : CVec<c_char> = unsafe { CVec::new(buf, len as uint) };
    println!("what => {:?}", what.as_slice());
    let time_str = my_data.start_time.rfc822();
    let ret = time_str.as_bytes();
    unsafe {
        driver_output(my_data.port, transmute(&ret[0]), ret.len() as ErlDrvSizeT)
    };
}

extern "C" fn my_control(drv_data: ErlDrvData, command: c_uint, buf: *mut c_char, len: ErlDrvSizeT,
                         rbuf: *mut *mut c_char, rlen: ErlDrvSizeT) -> ErlDrvSSizeT {
    let mut my_data = unsafe { transmute::<ErlDrvData,&mut ~MyDrvData>(drv_data) };
    0
}


// driver_name => "mydrv"
static driver_name : &'static [u8] = bytes!("mydrv\0");

static mut driver_entry : ErlDrvEntry = Struct_erl_drv_entry {
    init: None,
    start: Some(my_start),
    stop: Some(my_stop),
    output: Some(my_output),
    ready_input: None, ready_output: None,
    driver_name: 0 as *mut c_schar,  // set latter
    finish: None, handle: 0 as *mut c_void,
    control: Some(my_control),
    timeout: None, outputv: None, ready_async: None,
    flush: None, call: None, event: None,
    extended_marker: ERL_DRV_EXTENDED_MARKER,
    major_version: ERL_DRV_EXTENDED_MAJOR_VERSION,
    minor_version: ERL_DRV_EXTENDED_MINOR_VERSION,
    driver_flags: ERL_DRV_FLAG_USE_PORT_LOCKING,
    handle2: 0 as *mut c_void,
    process_exit: None, stop_select: None,
};


#[no_mangle] pub extern "C" fn driver_init() -> *ErlDrvEntry {
    unsafe {
        driver_entry.driver_name = transmute(&driver_name[0]);
        &driver_entry as *ErlDrvEntry
    }
}

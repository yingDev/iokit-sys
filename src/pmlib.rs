 // IOKit/IOPMLib.h
use libc::{c_void};
use types::*;
use super::{IONotificationPortRef, IOServiceInterestCallback};

extern 
{
	pub fn IORegisterForSystemPower(refcon: *mut c_void, thePortRef: *mut IONotificationPortRef, callback: IOServiceInterestCallback, notifier: *mut io_object_t) -> io_connect_t;
}

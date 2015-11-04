// exports from <IOKit/IOTypes.h>

use libc::{c_char,c_int,c_uint};

use mach::port::mach_port_t;
use mach::vm_types::mach_vm_address_t;

pub type IOOptionBits = u32;
pub type IOFixed      = i32;
pub type IOVersion    = u32;
pub type IOItemCount  = u32;
pub type IOCacheMode  = u32;

pub type IOByteCount32 = u32;
pub type IOByteCount64 = u64;

pub type IOPhysicalAddress32 = u32;
pub type IOPhysicalAddress64 = u64;
pub type IOPhysicalLength32  = u32;
pub type IOPhysicalLength64  = u64;


#[cfg(target_pointer_width = "64")] pub type IOVirtualAddress = mach_vm_address_t;
#[cfg(target_pointer_width = "32")] pub type IOVirtualAddress = vm_address_t;

#[cfg(target_pointer_width = "64")] pub type IOByteCount = IOByteCount64;
#[cfg(target_pointer_width = "32")] pub type IOByteCount = IOByteCount32;

pub type IOLogicalAddress = IOVirtualAddress;

#[cfg(target_pointer_width = "64")] pub type IOPhysicalAddress = IOPhysicalAddress64;
#[cfg(target_pointer_width = "64")] pub type IOPhysicalLength = IOPhysicalLength64;

#[cfg(target_pointer_width = "32")] pub type IOPhysicalAddress = IOPhysicalAddress32;
#[cfg(target_pointer_width = "32")] pub type IOPhysicalLength = IOPhysicalLength32;

#[repr(C)]
pub struct IOPhysicalRange {
    address: IOPhysicalAddress,
    length: IOByteCount
}

#[repr(C)]
pub struct IOVirtualRange {
    address: IOVirtualAddress,
    length: IOByteCount
}

#[cfg(target_pointer_width = "64")]
pub type IOAddressRange = IOVirtualRange;

#[cfg(target_pointer_width = "32")]
#[repr(C)]
pub struct IOAddressRange {
    address: mach_vm_address_t,
    length: mach_vm_size_t
}

#[repr(C)]
pub struct IONamedValue {
    value: c_int,
    name: *const c_char
}

pub type IOAlignment = c_uint;

pub type io_object_t = mach_port_t;

pub type io_connect_t        = io_object_t;
pub type io_enumerator_t     = io_object_t;
pub type io_iterator_t       = io_object_t;
pub type io_registry_entry_t = io_object_t;
pub type io_service_t        = io_object_t;

pub const IO_OBJECT_NULL: io_object_t = 0;

// IOConnectMapMemory memory types
pub const kIODefaultMemoryType: c_int = 0;

// cache types
pub const kIODefaultCache:       c_int = 0;
pub const kIOInhibitCache:       c_int = 1;
pub const kIOWriteThruCache:     c_int = 2;
pub const kIOCopybackCache:      c_int = 3;
pub const kIOWriteCombineCache:  c_int = 4;
pub const kIOCopybackInnerCache: c_int = 5;

// IOMemory mapping options
pub const kIOMapAnywhere:           c_int = 0x00000001;
pub const kIOMapCacheMask:          c_int = 0x0000700;
pub const kIOMapCacheShift:         c_int = 8;
pub const kIOMapDefaultCache:       c_int = kIODefaultCache       << kIOMapCacheShift;
pub const kIOMapInhibitCache:       c_int = kIOInhibitCache       << kIOMapCacheShift;
pub const kIOMapWriteThruCache:     c_int = kIOWriteThruCache     << kIOMapCacheShift;
pub const kIOMapCopybackCache:      c_int = kIOCopybackCache      << kIOMapCacheShift;
pub const kIOMapWriteCombineCache:  c_int = kIOWriteCombineCache  << kIOMapCacheShift;
pub const kIOMapCopybackInnerCache: c_int = kIOCopybackInnerCache << kIOMapCacheShift;
pub const kIOMapUserOptionsMask:    c_int = 0x00000FFF;
pub const kIOMapReadOnly:           c_int = 0x00001000;
pub const kIOMapStatic:             c_int = 0x01000000;
pub const kIOMapReference:          c_int = 0x02000000;
pub const kIOMapUnique:             c_int = 0x04000000;
pub const kIOMapPrefault:           c_int = 0x10000000;

// scale factors
pub const kNanosecondScale:   c_int = 1;
pub const kMicrosecondScale:  c_int = 1000;
pub const kMillisecondScale:  c_int = 1000 * 1000;
pub const kSecondScale:       c_int = 1000 * 1000 * 1000;
pub const kTickScale:         c_int = (kSecondScale / 100);

pub const kIOConnectMethodVarOutputSize: c_int = -3;

pub type IODeviceNumber = c_uint;

// IOMessage
pub const kIOMessageServiceIsTerminated: u32 = 0xe0000010;
pub const kIOMessageServiceIsSuspended: u32 = 0xe0000020;
pub const kIOMessageServiceIsResumed: u32 = 0xe0000030;
pub const kIOMessageServiceIsRequestingClose: u32 = 0xe0000100;
pub const kIOMessageServiceIsAttemptingOpen: u32 = 0xe0000101;
pub const kIOMessageServiceWasClosed: u32 = 0xe0000110;
pub const kIOMessageServiceBusyStateChange: u32 = 0xe0000120;
pub const kIOMessageConsoleSecurityChange: u32 = 0xe0000128;
pub const kIOMessageServicePropertyChange: u32 = 0xe0000130;
pub const kIOMessageCopyClientID: u32 = 0xe0000330;
pub const kIOMessageSystemCapabilityChange: u32 = 0xe0000340;
pub const kIOMessageDeviceSignaledWakeup: u32 = 0xe0000350;
pub const kIOMessageDeviceWillPowerOff: u32 = 0xe0000210;
pub const kIOMessageDeviceHasPoweredOn: u32 = 0xe0000230;
pub const kIOMessageSystemWillPowerOff: u32 = 0xe0000250;
pub const kIOMessageSystemWillRestart: u32 = 0xe0000310;
pub const kIOMessageSystemPagingOff: u32 = 0xe0000255;
pub const kIOMessageCanSystemSleep: u32 = 0xe0000270;
pub const kIOMessageSystemWillNotSleep: u32 = 0xe0000290;
pub const kIOMessageSystemWillSleep: u32 = 0xe0000280;
pub const kIOMessageSystemWillPowerOn: u32 = 0xe0000320;
pub const kIOMessageSystemHasPoweredOn: u32 = 0xe0000300;
pub const kIOMessageCanDevicePowerOff: u32 = 0xe0000200;
pub const kIOMessageDeviceWillNotPowerOff: u32 = 0xe0000220;
pub const kIOMessageSystemWillNotPowerOff: u32 = 0xe0000260;
pub const kIOMessageCanSystemPowerOff: u32 = 0xe0000240;
pub const kIOMessageDeviceWillPowerOn: u32 = 0xe0000215;
pub const kIOMessageDeviceHasPoweredOff: u32 = 0xe0000225;

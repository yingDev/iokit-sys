
// <IOKit/IOMessage.h>

pub const kIOMessageServiceIsTerminated: u32 		= 0xe0000010;
pub const kIOMessageServiceIsSuspended: u32 		= 0xe0000020;
pub const kIOMessageServiceIsResumed: u32 			= 0xe0000030;
pub const kIOMessageServiceIsRequestingClose: u32 	= 0xe0000100;
pub const kIOMessageServiceIsAttemptingOpen: u32 	= 0xe0000101;
pub const kIOMessageServiceWasClosed: u32 			= 0xe0000110;
pub const kIOMessageServiceBusyStateChange: u32 	= 0xe0000120;
pub const kIOMessageConsoleSecurityChange: u32 		= 0xe0000128;
pub const kIOMessageServicePropertyChange: u32 		= 0xe0000130;
pub const kIOMessageCopyClientID: u32 				= 0xe0000330;
pub const kIOMessageSystemCapabilityChange: u32 	= 0xe0000340;
pub const kIOMessageDeviceSignaledWakeup: u32 		= 0xe0000350;
pub const kIOMessageDeviceWillPowerOff: u32 		= 0xe0000210;
pub const kIOMessageDeviceHasPoweredOn: u32 		= 0xe0000230;
pub const kIOMessageSystemWillPowerOff: u32 		= 0xe0000250;
pub const kIOMessageSystemWillRestart: u32 			= 0xe0000310;
pub const kIOMessageSystemPagingOff: u32 			= 0xe0000255;
pub const kIOMessageCanSystemSleep: u32 			= 0xe0000270;
pub const kIOMessageSystemWillNotSleep: u32 		= 0xe0000290;
pub const kIOMessageSystemWillSleep: u32 			= 0xe0000280;
pub const kIOMessageSystemWillPowerOn: u32 			= 0xe0000320;
pub const kIOMessageSystemHasPoweredOn: u32 		= 0xe0000300;
pub const kIOMessageCanDevicePowerOff: u32 			= 0xe0000200;
pub const kIOMessageDeviceWillNotPowerOff: u32 		= 0xe0000220;
pub const kIOMessageSystemWillNotPowerOff: u32 		= 0xe0000260;
pub const kIOMessageCanSystemPowerOff: u32 			= 0xe0000240;
pub const kIOMessageDeviceWillPowerOn: u32 			= 0xe0000215;
pub const kIOMessageDeviceHasPoweredOff: u32 		= 0xe0000225;
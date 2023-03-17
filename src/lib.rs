extern crate libc;

#[macro_use]
extern crate ioctl_sys as ioctl;

use std::mem;

use libc::{c_char, c_int, c_uint, c_ulong};
use libc::{uint16_t, int32_t, uint32_t};
use libc::timeval;

macro_rules! uin {
	(write $name:ident with $ioty:expr, $nr:expr; $ty:ty) => (
		pub unsafe fn $name(fd: c_int, val: $ty) -> c_int {
			ioctl::ioctl(fd, iow!($ioty, $nr, mem::size_of::<$ty>()) as c_ulong, val)
		}
	);
}

mod events;
pub use events::*;

pub const UINPUT_MAX_NAME_SIZE: c_int = 80;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct input_event {
	pub time:  timeval,
	pub kind:  uint16_t,
	pub code:  uint16_t,
	pub value: int32_t,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct input_id {
	pub bustype: uint16_t,
	pub vendor:  uint16_t,
	pub product: uint16_t,
	pub version: uint16_t,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct input_absinfo {
	pub value:         int32_t,
	pub minimum:       int32_t,
	pub maximum:       int32_t,
	pub fuzz:          int32_t,
	pub flat:          int32_t,
	pub resolution:    int32_t,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct uinput_setup {
	pub id:    input_id,
	pub name:  [c_char; UINPUT_MAX_NAME_SIZE as usize],

	pub ff_effects_max: uint32_t,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct uinput_abs_setup {
	pub code:      uint16_t, /* axis code */
	/* filler:     uint16_t, */
	pub absinfo:   input_absinfo,
}

#[repr(C)]
pub struct uinput_user_dev {
	pub name: [c_char; UINPUT_MAX_NAME_SIZE as usize],
	pub id:   input_id,

	pub ff_effects_max: uint32_t,
	pub absmax:  [int32_t; ABS_CNT as usize],
	pub absmin:  [int32_t; ABS_CNT as usize],
	pub absfuzz: [int32_t; ABS_CNT as usize],
	pub absflat: [int32_t; ABS_CNT as usize],
}

//#[repr(C)]
//pub struct uinput_ff_upload {
//	pub request_id: uint32_t,
//	pub retval:     int32_t,
//	pub effect:     ff_effect,
//	pub old:        ff_effect,
//}
//
//#[repr(C)]
//pub struct uinput_ff_erase {
//	pub request_id: uint32_t,
//	pub retval:     int32_t,
//	pub effect_id:  uint32_t,
//}

ioctl!(none ui_dev_create with b'U', 1);
ioctl!(none ui_dev_destroy with b'U', 2);

ioctl!(write ui_dev_setup with b'U', 3; uinput_setup);
ioctl!(write ui_abs_setup with b'U', 4; uinput_abs_setup);

uin!(write ui_set_evbit   with b'U', 100; c_int);
uin!(write ui_set_keybit  with b'U', 101; c_int);
uin!(write ui_set_relbit  with b'U', 102; c_int);
uin!(write ui_set_absbit  with b'U', 103; c_int);
uin!(write ui_set_mscbit  with b'U', 104; c_int);
uin!(write ui_set_ledbit  with b'U', 105; c_int);
uin!(write ui_set_sndbit  with b'U', 106; c_int);
uin!(write ui_set_ffbit   with b'U', 107; c_int);
uin!(write ui_set_phys    with b'U', 108; *const c_char);
uin!(write ui_set_swbit   with b'U', 109; c_int);
uin!(write ui_set_propbit with b'U', 110; c_int);

//ioctl!(readwrite ui_begin_ff_upload with b'U', 200, uinput_ff_upload);
//ioctl!(readwrite ui_end_ff_upload with b'U', 201, uinput_ff_upload);

//ioctl!(readwrite ui_begin_ff_erase with b'U', 200, uinput_ff_erase);
//ioctl!(readwrite ui_end_ff_erase with b'U', 201, uinput_ff_erase);

ioctl!(read ui_get_version with b'U', 45; c_uint);

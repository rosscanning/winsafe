#![allow(non_snake_case)]

use crate::co;
use crate::HWND;
use crate::com::{IUnknown, IUnknownVtbl};
use crate::ffi::Void;

/// [`ITaskbarList`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist)
/// ->
/// [`IUnknown`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown).
pub struct ITaskbarList {
	/// Base
	/// [`IUnknown`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown).
	pub base: IUnknown,
}

#[repr(C)]
pub struct ITaskbarListVtbl {
	iUnknownVtbl: IUnknownVtbl,
	hrInit: fn(*const *const ITaskbarListVtbl) -> u32,
	addTab: fn(*const *const ITaskbarListVtbl, *const Void) -> u32,
	deleteTab: fn(*const *const ITaskbarListVtbl, *const Void) -> u32,
	activateTab: fn(*const *const ITaskbarListVtbl, *const Void) -> u32,
	setActiveAlt: fn(*const *const ITaskbarListVtbl, *const Void) -> u32,
}

impl From<*const *const ITaskbarListVtbl> for ITaskbarList {
	/// Creates a new object from a pointer to a pointer to its virtual table.
	fn from(ppv: *const *const ITaskbarListVtbl) -> Self {
		Self {
			base: IUnknown::from(ppv as *const *const IUnknownVtbl)
		}
	}
}

impl ITaskbarList {
	/// [`ITaskbarList::SetActiveAlt`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist-setactivealt)
	/// method.
	pub fn SetActiveAlt(&self, hwnd: HWND) -> Result<(), co::ERROR> {
		let ppv = self.base.ppv::<ITaskbarListVtbl>();
		let pfun = unsafe { (*(*ppv)).setActiveAlt };

		match co::ERROR::from(pfun(ppv, hwnd.as_ptr())) {
			co::ERROR::S_OK => Ok(()),
			err => Err(err),
		}
	}
}
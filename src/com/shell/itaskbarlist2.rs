#![allow(non_snake_case)]

use crate::{ComInterface, HWND, IID};
use crate::co;
use crate::ffi::Void;
use crate::shell::{ITaskbarList, ITaskbarListVtbl};

type PPVtbl = *const *const ITaskbarList2Vtbl;

#[repr(C)]
pub struct ITaskbarList2Vtbl {
	iTaskbarListVtbl: ITaskbarListVtbl,
	MarkFullscreenWindow: fn(PPVtbl, *const Void, u32) -> u32,
}

impl ComInterface for ITaskbarList2Vtbl {
	fn Iid() -> IID {
		IID::new(0x602d4995, 0xb13a, 0x429b, 0xa66e, 0x1935e44f4317)
	}
}

//------------------------------------------------------------------------------

/// [`ITaskbarList2`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-itaskbarlist2)
/// COM interface.
///
/// Inherits from:
/// * [`ITaskbarList`](crate::shell::ITaskbarList);
/// * [`IUnknown`](crate::IUnknown).
///
/// Usually instantiated as:
/// ```rust,ignore
/// let mut obj = shell::ITaskbarList2::from(
///   CoCreateInstance(
///     &shell::clsid::TaskbarList,
///     None,
///     co::CLSCTX::INPROC_SERVER,
///   ),
/// );
/// ```
pub struct ITaskbarList2 {
	/// Base
	/// [`ITaskbarList`](crate::shell::ITaskbarList).
	pub iTaskbarList: ITaskbarList,
}

impl From<PPVtbl> for ITaskbarList2 {
	/// Creates a new object from a pointer to a pointer to its virtual table.
	fn from(ppv: PPVtbl) -> Self {
		Self {
			iTaskbarList: ITaskbarList::from(ppv as *const *const ITaskbarListVtbl),
		}
	}
}

impl ITaskbarList2 {
	/// [`ITaskbarList2::MarkFullscreenWindow`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-itaskbarlist2-markfullscreenwindow)
	/// method.
	pub fn MarkFullscreenWindow(
		&self, hwnd: HWND, fFullscreen: bool) -> Result<(), co::ERROR>
	{
		unsafe {
			let ppv = self.iTaskbarList.iUnknown.ppv::<ITaskbarList2Vtbl>();
			let pfun = (*(*ppv)).MarkFullscreenWindow;

			match co::ERROR::from(pfun(ppv, hwnd.as_ptr(), fFullscreen as u32)) {
				co::ERROR::S_OK => Ok(()),
				err => Err(err),
			}
		}
	}
}
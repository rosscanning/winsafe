const_type! { QUALITY, u8,
	/// [`LOGFONT`](crate::LOGFONT) `lfQuality` (`u8`).

	DEFAULT, 0
	DRAFT, 1
	PROOF, 2
	NONANTIALIASED, 3
	ANTIALIASED, 4
	CLEARTYPE, 5
	CLEARTYPE_NATURAL, 6
}

const_type! { QS, u32,
	/// [`GetQueueStatus`](crate::GetQueueStatus) `flags` (`u32`).

	KEY, 0x0001
	MOUSEMOVE, 0x0002
	MOUSEBUTTON, 0x0004
	POSTMESSAGE, 0x0008
	TIMER, 0x0010
	PAINT, 0x0020
	SENDMESSAGE, 0x0040
	HOTKEY, 0x0080
	ALLPOSTMESSAGE, 0x0100
	RAWINPUT, 0x0400
	TOUCH, 0x0800
	POINTER, 0x1000
	MOUSE, Self::MOUSEMOVE.0 | Self::MOUSEBUTTON.0
	INPUT, Self::MOUSE.0 | Self::KEY.0 | Self::RAWINPUT.0 | Self::TOUCH.0 | Self::POINTER.0
	ALLINPUT, Self::INPUT.0 | Self::POSTMESSAGE.0 | Self::TIMER.0 | Self::PAINT.0 | Self::HOTKEY.0 | Self::SENDMESSAGE.0
}

const_type! { REG, u32,
	/// Registry
	/// [value types](https://docs.microsoft.com/en-us/windows/win32/sysinfo/registry-value-types)
	/// (`u21`).

	NONE, 0
	SZ, 1
	EXPAND_SZ, 2
	BINARY, 3
	DWORD, 4
	DWORD_LITTLE_ENDIAN, 4
	DWORD_BIG_ENDIAN, 5
	LINK, 6
	MULTI_SZ, 7
	RESOURCE_LIST, 8
	FULL_RESOURCE_DESCRIPTOR, 9
	RESOURCE_REQUIREMENTS_LIST, 10
	QWORD, 11
	QWORD_LITTLE_ENDIAN, 11
}

const_type! { REG_OPTION, u32,
	/// [`RegOpenKeyEx`](crate::HKEY::RegOpenKeyEx) `uOptions` (`u32`).

	RESERVED, 0x00000000
	NON_VOLATILE, 0x00000000
	VOLATILE, 0x00000001
	CREATE_LINK, 0x00000002
	BACKUP_RESTORE, 0x00000004
	OPEN_LINK, 0x00000008
}

const_type! { REGION, i32,
	/// [`GetUpdateRgn`](crate::HWND::GetUpdateRgn),
	/// [`GetWindowRgn`](crate::HWND::GetWindowRgn) and
	/// [`SelectObject`](crate::HDC::SelectObject) return value (`i32`).

	NULL, 1
	SIMPLE, 2
	COMPLEX, 3
}

const_type! { RRF, u32,
	/// [`RegGetValue`](crate::HKEY::RegGetValue) `dwFlags` (`u32`).

	RT_REG_NONE, 0x00000001
	RT_REG_SZ, 0x00000002
	RT_REG_EXPAND_SZ, 0x00000004
	RT_REG_BINARY, 0x00000008
	RT_REG_DWORD, 0x00000010
	RT_REG_MULTI_SZ, 0x00000020
	RT_REG_QWORD, 0x00000040
	RT_DWORD, Self::RT_REG_BINARY.0 | Self::RT_REG_DWORD.0
	RT_QWORD, Self::RT_REG_BINARY.0 | Self::RT_REG_QWORD.0
	RT_ANY, 0x0000ffff

	SUBKEY_WOW6464KEY, 0x00010000
	SUBKEY_WOW6432KEY, 0x00020000
	WOW64_MASK, 0x00030000

	NOEXPAND, 0x10000000
	ZEROONFAILURE, 0x20000000
}
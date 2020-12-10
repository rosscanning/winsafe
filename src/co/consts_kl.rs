#![allow(non_snake_case)]

use crate::co;

const_type! { KEY, u32,
	/// [`RegOpenKeyEx`](crate::HKEY::RegOpenKeyEx) `samDesired`.

	QUERY_VALUE, 0x0001
	SET_VALUE, 0x0002
	CREATE_SUB_KEY, 0x0004
	ENUMERATE_SUB_KEYS, 0x0008
	NOTIFY, 0x0010
	CREATE_LINK, 0x0020
	WOW64_32KEY, 0x0200
	WOW64_64KEY, 0x0100
	WOW64_RES, 0x0300
	READ, (co::STANDARD_RIGHTS::READ.0 | Self::QUERY_VALUE.0 | Self::ENUMERATE_SUB_KEYS.0 | Self::NOTIFY.0) & !co::ACCESS_RIGHTS::SYNCHRONIZE.0
	WRITE, (co::STANDARD_RIGHTS::WRITE.0 | Self::SET_VALUE.0 | Self::CREATE_SUB_KEY.0) & !co::ACCESS_RIGHTS::SYNCHRONIZE.0
	EXECUTE, Self::READ.0 & !co::ACCESS_RIGHTS::SYNCHRONIZE.0
	ALL_ACCESS, (co::STANDARD_RIGHTS::ALL.0 | Self::QUERY_VALUE.0 | Self::SET_VALUE.0 | Self::CREATE_SUB_KEY.0 | Self::ENUMERATE_SUB_KEYS.0 | Self::NOTIFY.0 | Self::CREATE_LINK.0) & !co::ACCESS_RIGHTS::SYNCHRONIZE.0
}

const_type! { LANG, u16,
	/// [`FormatMessage`](crate::co::ERROR::FormatMessage) `dwLanguageId`, used
	/// with [`SUBLANG`](crate::co::SUBLANG).

	NEUTRAL, 0x00
	INVARIANT, 0x7f
	AFRIKAANS, 0x36
	ALBANIAN, 0x1c
	ALSATIAN, 0x84
	AMHARIC, 0x5e
	ARABIC, 0x01
	ARMENIAN, 0x2b
	ASSAMESE, 0x4d
	AZERI, 0x2c
	AZERBAIJANI, 0x2c
	BANGLA, 0x45
	BASHKIR, 0x6d
	BASQUE, 0x2d
	BELARUSIAN, 0x23
	BENGALI, 0x45
	BRETON, 0x7e
	BOSNIAN, 0x1a
	BOSNIAN_NEUTRAL, 0x781a
	BULGARIAN, 0x02
	CATALAN, 0x03
	CENTRAL_KURDISH, 0x92
	CHEROKEE, 0x5c
	CHINESE, 0x04
	CHINESE_SIMPLIFIED, 0x04
	CHINESE_TRADITIONAL, 0x7c04
	CORSICAN, 0x83
	CROATIAN, 0x1a
	CZECH, 0x05
	DANISH, 0x06
	DARI, 0x8c
	DIVEHI, 0x65
	DUTCH, 0x13
	ENGLISH, 0x09
	ESTONIAN, 0x25
	FAEROESE, 0x38
	FARSI, 0x29
	FILIPINO, 0x64
	FINNISH, 0x0b
	FRENCH, 0x0c
	FRISIAN, 0x62
	FULAH, 0x67
	GALICIAN, 0x56
	GEORGIAN, 0x37
	GERMAN, 0x07
	GREEK, 0x08
	GREENLANDIC, 0x6f
	GUJARATI, 0x47
	HAUSA, 0x68
	HAWAIIAN, 0x75
	HEBREW, 0x0d
	HINDI, 0x39
	HUNGARIAN, 0x0e
	ICELANDIC, 0x0f
	IGBO, 0x70
	INDONESIAN, 0x21
	INUKTITUT, 0x5d
	IRISH, 0x3c
	ITALIAN, 0x10
	JAPANESE, 0x11
	KANNADA, 0x4b
	KASHMIRI, 0x60
	KAZAK, 0x3f
	KHMER, 0x53
	KICHE, 0x86
	KINYARWANDA, 0x87
	KONKANI, 0x57
	KOREAN, 0x12
	KYRGYZ, 0x40
	LAO, 0x54
	LATVIAN, 0x26
	LITHUANIAN, 0x27
	LOWER_SORBIAN, 0x2e
	LUXEMBOURGISH, 0x6e
	MACEDONIAN, 0x2f
	MALAY, 0x3e
	MALAYALAM, 0x4c
	MALTESE, 0x3a
	MANIPURI, 0x58
	MAORI, 0x81
	MAPUDUNGUN, 0x7a
	MARATHI, 0x4e
	MOHAWK, 0x7c
	MONGOLIAN, 0x50
	NEPALI, 0x61
	NORWEGIAN, 0x14
	OCCITAN, 0x82
	ODIA, 0x48
	ORIYA, 0x48
	PASHTO, 0x63
	PERSIAN, 0x29
	POLISH, 0x15
	PORTUGUESE, 0x16
	PULAR, 0x67
	PUNJABI, 0x46
	QUECHUA, 0x6b
	ROMANIAN, 0x18
	ROMANSH, 0x17
	RUSSIAN, 0x19
	SAKHA, 0x85
	SAMI, 0x3b
	SANSKRIT, 0x4f
	SCOTTISH_GAELIC, 0x91
	SERBIAN, 0x1a
	SERBIAN_NEUTRAL, 0x7c1a
	SINDHI, 0x59
	SINHALESE, 0x5b
	SLOVAK, 0x1b
	SLOVENIAN, 0x24
	SOTHO, 0x6c
	SPANISH, 0x0a
	SWAHILI, 0x41
	SWEDISH, 0x1d
	SYRIAC, 0x5a
	TAJIK, 0x28
	TAMAZIGHT, 0x5f
	TAMIL, 0x49
	TATAR, 0x44
	TELUGU, 0x4a
	THAI, 0x1e
	TIBETAN, 0x51
	TIGRIGNA, 0x73
	TIGRINYA, 0x73
	TSWANA, 0x32
	TURKISH, 0x1f
	TURKMEN, 0x42
	UIGHUR, 0x80
	UKRAINIAN, 0x22
	UPPER_SORBIAN, 0x2e
	URDU, 0x20
	UZBEK, 0x43
	VALENCIAN, 0x03
	VIETNAMESE, 0x2a
	WELSH, 0x52
	WOLOF, 0x88
	XHOSA, 0x34
	YAKUT, 0x85
	YI, 0x78
	YORUBA, 0x6a
	ZULU, 0x35
}
impl LANG {
	/// [`MAKELANGID`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-makelangid)
	/// macro.
	pub fn MAKELANGID(self, sublang: co::SUBLANG) -> u32 {
		((sublang.0 << 10) | self.0) as u32
	}
}

const_type! { FW, u32,
	/// [`LOGFONT`](crate::LOGFONT) `lfWeight`.

	DONTCARE, 0
	THIN, 100
	EXTRALIGHT, 200
	ULTRALIGHT, Self::EXTRALIGHT.0
	LIGHT, 300
	NORMAL, 400
	REGULAR, 400
	MEDIUM, 500
	SEMIBOLD, 600
	DEMIBOLD, Self::SEMIBOLD.0
	BOLD, 700
	EXTRABOLD, 800
	ULTRABOLD, Self::EXTRABOLD.0
	HEAVY, 900
	BLACK, Self::HEAVY.0
}
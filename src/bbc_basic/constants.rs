use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

pub const TOKEN_MASK: u8 = 0x80;

pub const END_MARKER: [u8; 2] = [0x0d, 0xff];

pub const REM_TOKEN: u8 = 0xf4;

pub const LINE_NUMBER_TOKEN: u8 = 0x8d;

// Note: this list all BBC BASIC V tokens even though we only
// care about BBC BASIC II!
pub const KEYWORD_TOKENS: [(&str, u8); 128] = [
    ("OTHERWISE", 0x7f),
    ("AND", 0x80),
    ("DIV", 0x81),
    ("EOR", 0x82),
    ("MOD", 0x83),
    ("OR", 0x84),
    ("ERROR", 0x85),
    ("LINE", 0x86),
    ("OFF", 0x87),
    ("STEP", 0x88),
    ("SPC", 0x89),
    ("TAB(", 0x8a),
    ("ELSE", 0x8b),
    ("THEN", 0x8c),
    ("<line>", LINE_NUMBER_TOKEN),
    ("OPENIN", 0x8e),
    ("PTR", 0x8f),
    ("PAGE", 0x90),
    ("TIME", 0x91),
    ("LOMEM", 0x92),
    ("HIMEM", 0x93),
    ("ABS", 0x94),
    ("ACS", 0x95),
    ("ADVAL", 0x96),
    ("ASC", 0x97),
    ("ASN", 0x98),
    ("ATN", 0x99),
    ("BGET", 0x9a),
    ("COS", 0x9b),
    ("COUNT", 0x9c),
    ("DEG", 0x9d),
    ("ERL", 0x9e),
    ("ERR", 0x9f),
    ("EVAL", 0xa0),
    ("EXP", 0xa1),
    ("EXT", 0xa2),
    ("FALSE", 0xa3),
    ("FN", 0xa4),
    ("GET", 0xa5),
    ("INKEY", 0xa6),
    ("INSTR(", 0xa7),
    ("INT", 0xa8),
    ("LEN", 0xa9),
    ("LN", 0xaa),
    ("LOG", 0xab),
    ("NOT", 0xac),
    ("OPENUP", 0xad),
    ("OPENOUT", 0xae),
    ("PI", 0xaf),
    ("POINT(", 0xb0),
    ("POS", 0xb1),
    ("RAD", 0xb2),
    ("RND", 0xb3),
    ("SGN", 0xb4),
    ("SIN", 0xb5),
    ("SQR", 0xb6),
    ("TAN", 0xb7),
    ("TO", 0xb8),
    ("TRUE", 0xb9),
    ("USR", 0xba),
    ("VAL", 0xbb),
    ("VPOS", 0xbc),
    ("CHR$", 0xbd),
    ("GET$", 0xbe),
    ("INKEY$", 0xbf),
    ("LEFT$(", 0xc0),
    ("MID$(", 0xc1),
    ("RIGHT$(", 0xc2),
    ("STR$", 0xc3),
    ("STRING$(", 0xc4),
    ("EOF", 0xc5),
    ("<ESCFN>", 0xc6),
    ("<ESCCOM>", 0xc7),
    ("<ESCSTMT>", 0xc8),
    ("WHEN", 0xc9),
    ("OF", 0xca),
    ("ENDCASE", 0xcb),
    //("ELSE", 0xcc),
    ("ENDIF", 0xcd),
    ("ENDWHILE", 0xce),
    ("PTR", 0xcf),
    ("PAGE", 0xd0),
    ("TIME", 0xd1),
    ("LOMEM", 0xd2),
    ("HIMEM", 0xd3),
    ("SOUND", 0xd4),
    ("BPUT", 0xd5),
    ("CALL", 0xd6),
    ("CHAIN", 0xd7),
    ("CLEAR", 0xd8),
    ("CLOSE", 0xd9),
    ("CLG", 0xda),
    ("CLS", 0xdb),
    ("DATA", 0xdc),
    ("DEF", 0xdd),
    ("DIM", 0xde),
    ("DRAW", 0xdf),
    ("END", 0xe0),
    ("ENDPROC", 0xe1),
    ("ENVELOPE", 0xe2),
    ("FOR", 0xe3),
    ("GOSUB", 0xe4),
    ("GOTO", 0xe5),
    ("GCOL", 0xe6),
    ("IF", 0xe7),
    ("INPUT", 0xe8),
    ("LET", 0xe9),
    ("LOCAL", 0xea),
    ("MODE", 0xeb),
    ("MOVE", 0xec),
    ("NEXT", 0xed),
    ("ON", 0xee),
    ("VDU", 0xef),
    ("PLOT", 0xf0),
    ("PRINT", 0xf1),
    ("PROC", 0xf2),
    ("READ", 0xf3),
    ("REM", REM_TOKEN),
    ("REPEAT", 0xf5),
    ("REPORT", 0xf6),
    ("RESTORE", 0xf7),
    ("RETURN", 0xf8),
    ("RUN", 0xf9),
    ("STOP", 0xfa),
    ("COLOUR", 0xfb),
    ("TRACE", 0xfc),
    ("UNTIL", 0xfd),
    ("WIDTH", 0xfe),
    ("OSCLI", 0xff),
];

pub static KEYWORDS_BY_NAME: LazyLock<HashMap<&str, u8>> =
    LazyLock::new(|| KEYWORD_TOKENS.iter().copied().collect());

pub static KEYWORDS_BY_TOKEN: LazyLock<HashMap<u8, &str>> = LazyLock::new(|| {
    KEYWORD_TOKENS
        .iter()
        .map(|(name, token)| (*token, *name))
        .collect()
});

const LINE_NUMBER_KEYWORDS: [&str; 4] = ["ELSE", "GOTO", "GOSUB", "THEN"];

pub static LINE_NUMBER_TOKENS: LazyLock<HashSet<u8>> = LazyLock::new(|| {
    LINE_NUMBER_KEYWORDS
        .map(|name| *KEYWORDS_BY_NAME.get(name).expect("must exist"))
        .into_iter()
        .collect()
});

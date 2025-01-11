pub struct ANSI {
   pub prefix: &'static str,
   pub suffix: &'static str,
}

pub const RED: ANSI = ANSI {
   prefix: "\x1b[31m",
   suffix: "\x1b[0m",
};
pub const BOLD_RED: ANSI = ANSI {
   prefix: "\x1b[1m\x1b[31m",
   suffix: "\x1b[0m",
};
pub const DARK_RED: ANSI = ANSI {
   prefix: "\x1b[91m",
   suffix: "\x1b[0m",
};
pub const BOLD_DARK_RED: ANSI = ANSI {
   prefix: "\x1b[1m\x1b[91m",
   suffix: "\x1b[0m",
};

pub const GREEN: ANSI = ANSI {
   prefix: "\x1b[32m",
   suffix: "\x1b[0m",
};
pub const BOLD_GREEN: ANSI = ANSI {
   prefix: "\x1b[1m\x1b[32m",
   suffix: "\x1b[0m",
};
pub const DARK_GREEN: ANSI = ANSI {
   prefix: "\x1b[92m",
   suffix: "\x1b[0m",
};
pub const BOLD_DARK_GREEN: ANSI = ANSI {
   prefix: "\x1b[1m\x1b[92m",
   suffix: "\x1b[0m",
};

pub const YELLOW: ANSI = ANSI {
   prefix: "\x1b[33m",
   suffix: "\x1b[0m",
};
pub const BOLD_YELLOW: ANSI = ANSI {
   prefix: "\x1b[1m\x1b[33m",
   suffix: "\x1b[0m",
};
pub const DARK_YELLOW: ANSI = ANSI {
   prefix: "\x1b[93m",
   suffix: "\x1b[0m",
};
pub const BOLD_DARK_YELLOW: ANSI = ANSI {
   prefix: "\x1b[1m\x1b[93m",
   suffix: "\x1b[0m",
};

pub const BLUE: ANSI = ANSI {
   prefix: "\x1b[34m",
   suffix: "\x1b[0m",
};
pub const BOLD_BLUE: ANSI = ANSI {
   prefix: "\x1b[1m\x1b[34m",
   suffix: "\x1b[0m",
};
pub const DARK_BLUE: ANSI = ANSI {
   prefix: "\x1b[94m",
   suffix: "\x1b[0m",
};
pub const BOLD_DARK_BLUE: ANSI = ANSI {
   prefix: "\x1b[1m\x1b[94m",
   suffix: "\x1b[0m",
};

pub const MAGENTA: ANSI = ANSI {
   prefix: "\x1b[35m",
   suffix: "\x1b[0m",
};
pub const BOLD_MAGENTA: ANSI = ANSI {
   prefix: "\x1b[1m\x1b[35m",
   suffix: "\x1b[0m",
};
pub const DARK_MAGENTA: ANSI = ANSI {
   prefix: "\x1b[95m",
   suffix: "\x1b[0m",
};
pub const BOLD_DARK_MAGENTA: ANSI = ANSI {
   prefix: "\x1b[1m\x1b[95m",
   suffix: "\x1b[0m",
};

pub const CYAN: ANSI = ANSI {
   prefix: "\x1b[36m",
   suffix: "\x1b[0m",
};
pub const BOLD_CYAN: ANSI = ANSI {
   prefix: "\x1b[1m\x1b[36m",
   suffix: "\x1b[0m",
};
pub const DARK_CYAN: ANSI = ANSI {
   prefix: "\x1b[96m",
   suffix: "\x1b[0m",
};
pub const BOLD_DARK_CYAN: ANSI = ANSI {
   prefix: "\x1b[1m\x1b[96m",
   suffix: "\x1b[0m",
};

pub const BG_RED: ANSI = ANSI {
   prefix: "\x1b[41m\x1b[30m",
   suffix: "\x1b[0m",
};
pub const BOLD_BG_RED: ANSI = ANSI {
   prefix: "\x1b[1m\x1b[41m\x1b[30m",
   suffix: "\x1b[0m",
};
pub const BG_DARK_RED: ANSI = ANSI {
   prefix: "\x1b[101m\x1b[30m",
   suffix: "\x1b[0m",
};
pub const BOLD_BG_DARK_RED: ANSI = ANSI {
   prefix: "\x1b[1m\x1b[101m\x1b[30m",
   suffix: "\x1b[0m",
};

pub const BG_GREEN: ANSI = ANSI {
   prefix: "\x1b[42m\x1b[30m",
   suffix: "\x1b[0m",
};
pub const BOLD_BG_GREEN: ANSI = ANSI {
   prefix: "\x1b[1m\x1b[42m\x1b[30m",
   suffix: "\x1b[0m",
};
pub const BG_DARK_GREEN: ANSI = ANSI {
   prefix: "\x1b[102m\x1b[30m",
   suffix: "\x1b[0m",
};
pub const BOLD_BG_DARK_GREEN: ANSI = ANSI {
   prefix: "\x1b[1m\x1b[102m\x1b[30m",
   suffix: "\x1b[0m",
};

pub const BG_YELLOW: ANSI = ANSI {
   prefix: "\x1b[43m\x1b[30m",
   suffix: "\x1b[0m",
};
pub const BOLD_BG_YELLOW: ANSI = ANSI {
   prefix: "\x1b[1m\x1b[43m\x1b[30m",
   suffix: "\x1b[0m",
};
pub const BG_DARK_YELLOW: ANSI = ANSI {
   prefix: "\x1b[103m\x1b[30m",
   suffix: "\x1b[0m",
};
pub const BOLD_BG_DARK_YELLOW: ANSI = ANSI {
   prefix: "\x1b[1m\x1b[103m\x1b[30m",
   suffix: "\x1b[0m",
};

pub const BG_BLUE: ANSI = ANSI {
   prefix: "\x1b[44m\x1b[30m",
   suffix: "\x1b[0m",
};
pub const BOLD_BG_BLUE: ANSI = ANSI {
   prefix: "\x1b[1m\x1b[44m\x1b[30m",
   suffix: "\x1b[0m",
};
pub const BG_DARK_BLUE: ANSI = ANSI {
   prefix: "\x1b[104m\x1b[30m",
   suffix: "\x1b[0m",
};
pub const BOLD_BG_DARK_BLUE: ANSI = ANSI {
   prefix: "\x1b[1m\x1b[104m\x1b[30m",
   suffix: "\x1b[0m",
};

pub const BG_MAGENTA: ANSI = ANSI {
   prefix: "\x1b[45m\x1b[30m",
   suffix: "\x1b[0m",
};
pub const BOLD_BG_MAGENTA: ANSI = ANSI {
   prefix: "\x1b[1m\x1b[45m\x1b[30m",
   suffix: "\x1b[0m",
};
pub const BG_DARK_MAGENTA: ANSI = ANSI {
   prefix: "\x1b[105m\x1b[30m",
   suffix: "\x1b[0m",
};
pub const BOLD_BG_DARK_MAGENTA: ANSI = ANSI {
   prefix: "\x1b[1m\x1b[105m\x1b[30m",
   suffix: "\x1b[0m",
};

pub const BG_CYAN: ANSI = ANSI {
   prefix: "\x1b[46m\x1b[30m",
   suffix: "\x1b[0m",
};
pub const BOLD_BG_CYAN: ANSI = ANSI {
   prefix: "\x1b[1m\x1b[46m\x1b[30m",
   suffix: "\x1b[0m",
};
pub const BG_DARK_CYAN: ANSI = ANSI {
   prefix: "\x1b[106m\x1b[30m",
   suffix: "\x1b[0m",
};
pub const BOLD_BG_DARK_CYAN: ANSI = ANSI {
   prefix: "\x1b[1m\x1b[106m\x1b[30m",
   suffix: "\x1b[0m",
};

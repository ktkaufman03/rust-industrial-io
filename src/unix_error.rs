#[cfg(windows)]
mod windows {
    use std::fmt;

    extern "C" {
        fn _errno() -> *const i32;
    }

    /// Error-code-containing struct used as a fallback when `nix::Error` is unavailable
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct Error(i32);

    impl Error {
        /// Create a `Error` from an error code.
        pub fn from_i32(code: i32) -> Self {
            Self(code)
        }

        fn name(&self) -> &'static str {
            match self.0 {
                1 => "EPERM",
                2 => "ENOENT",
                3 => "ESRCH",
                4 => "EINTR",
                5 => "EIO",
                6 => "ENXIO",
                7 => "E2BIG",
                8 => "ENOEXEC",
                9 => "EBADF",
                10 => "ECHILD",
                11 => "EAGAIN",
                12 => "ENOMEM",
                13 => "EACCES",
                14 => "EFAULT",
                16 => "EBUSY",
                17 => "EEXIST",
                18 => "EXDEV",
                19 => "ENODEV",
                20 => "ENOTDIR",
                21 => "EISDIR",
                22 => "EINVAL",
                23 => "ENFILE",
                24 => "EMFILE",
                25 => "ENOTTY",
                27 => "EFBIG",
                28 => "ENOSPC",
                29 => "ESPIPE",
                30 => "EROFS",
                31 => "EMLINK",
                32 => "EPIPE",
                33 => "EDOM",
                34 => "ERANGE",
                36 => "EDEADLK",
                38 => "ENAMETOOLONG",
                39 => "ENOLCK",
                40 => "ENOSYS",
                41 => "ENOTEMPTY",
                42 => "EILSEQ",
                80 => "STRUNCATE",
                100 => "EADDRINUSE",
                101 => "EADDRNOTAVAIL",
                102 => "EAFNOSUPPORT",
                103 => "EALREADY",
                104 => "EBADMSG",
                105 => "ECANCELED",
                106 => "ECONNABORTED",
                107 => "ECONNREFUSED",
                108 => "ECONNRESET",
                109 => "EDESTADDRREQ",
                110 => "EHOSTUNREACH",
                111 => "EIDRM",
                112 => "EINPROGRESS",
                113 => "EISCONN",
                114 => "ELOOP",
                115 => "EMSGSIZE",
                116 => "ENETDOWN",
                117 => "ENETRESET",
                118 => "ENETUNREACH",
                119 => "ENOBUFS",
                120 => "ENODATA",
                121 => "ENOLINK",
                122 => "ENOMSG",
                123 => "ENOPROTOOPT",
                124 => "ENOSR",
                125 => "ENOSTR",
                126 => "ENOTCONN",
                127 => "ENOTRECOVERABLE",
                128 => "ENOTSOCK",
                129 => "ENOTSUP",
                130 => "EOPNOTSUPP",
                131 => "EOTHER",
                132 => "EOVERFLOW",
                133 => "EOWNERDEAD",
                134 => "EPROTO",
                135 => "EPROTONOSUPPORT",
                136 => "EPROTOTYPE",
                137 => "ETIME",
                138 => "ETIMEDOUT",
                139 => "ETXTBSY",
                140 => "EWOULDBLOCK",
                _ => "unknown",
            }
        }
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}: {}", self.0, self.name())
        }
    }

    pub fn last_error() -> Error {
        Error::from_i32(unsafe { *_errno() })
    }

    pub fn from_i32(code: i32) -> Error {
        Error::from_i32(code)
    }
}

#[cfg(unix)]
mod unix {
    pub use nix::Error;

    pub fn last_error() -> Error {
        Error::last()
    }

    pub fn from_i32(code: i32) -> Error {
        Error::from_i32(code)
    }
}

#[cfg(unix)]
pub use unix::{from_i32, last_error, Error};

#[cfg(windows)]
pub use windows::{from_i32, last_error, Error};

#[cfg(not(any(unix, windows)))]
compile_error!("No error translation available");

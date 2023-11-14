//! Unix specific network types.
// This module does not currently provide any public API, but it was
// unintentionally defined as a public module. Hide it from the documentation
// instead of changing it to a private module to avoid breakage.
#[doc(hidden)]
#[cfg(unix)]
pub mod datagram;

#[cfg(unix)]
pub(crate) mod listener;

#[cfg(unix)]
mod split;
#[cfg(unix)]
pub use split::{ReadHalf, WriteHalf};

#[cfg(unix)]
mod split_owned;
#[cfg(unix)]
pub use split_owned::{OwnedReadHalf, OwnedWriteHalf, ReuniteError};

mod socketaddr;
pub use socketaddr::SocketAddr;

pub(crate) mod stream;
#[allow(unused)]
pub(crate) use stream::UnixStream;

mod ucred;
pub use ucred::UCred;

pub mod pipe;

/// A type representing process and process group IDs.
#[allow(non_camel_case_types)]
pub type uid_t = u32;

/// A type representing user ID.
#[allow(non_camel_case_types)]
pub type gid_t = u32;

/// A type representing group ID.
#[allow(non_camel_case_types)]
pub type pid_t = i32;

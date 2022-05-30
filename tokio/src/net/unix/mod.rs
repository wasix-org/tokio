//! Unix domain socket utility types.

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
#[cfg(any(unix, all(tokio_wasix, feature = "fs")))]
pub(crate) use stream::UnixStream;

mod ucred;
pub use ucred::UCred;

#![cfg_attr(tokio_wasix, allow(unused, missing_docs))]
use std::fmt;
use std::path::Path;

/// An address associated with a Tokio Unix socket.
#[cfg(unix)]
pub struct SocketAddr(pub(super) mio::net::SocketAddr);

#[cfg(tokio_wasix)]
#[derive(Debug)]
pub struct SocketAddr();

#[cfg(unix)]
impl SocketAddr {
    /// Returns `true` if the address is unnamed.
    ///
    /// Documentation reflected in [`SocketAddr`]
    ///
    /// [`SocketAddr`]: std::os::unix::net::SocketAddr
    pub fn is_unnamed(&self) -> bool {
        self.0.is_unnamed()
    }

    /// Returns the contents of this address if it is a `pathname` address.
    ///
    /// Documentation reflected in [`SocketAddr`]
    ///
    /// [`SocketAddr`]: std::os::unix::net::SocketAddr
    pub fn as_pathname(&self) -> Option<&Path> {
        self.0.as_pathname()
    }
}

#[cfg(tokio_wasix)]
impl SocketAddr {
    pub fn is_unnamed(&self) -> bool {
        true
    }
    pub fn as_pathname(&self) -> Option<&Path> {
        None
    }
}

#[cfg(unix)]
impl fmt::Debug for SocketAddr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(fmt)
    }
}

//! WASI-specific types for signal handling.
//!
//! This module is only defined on WASI platforms and contains the primary
//! `Signal` type for receiving notifications of signals.

#![cfg(tokio_wasi)]
#![cfg_attr(docsrs, doc(cfg(all(tokio_wasi, feature = "signal"))))]
#![allow(unused_variables)]

use crate::signal::registry::{EventInfo, Init, Storage, EventId};
use std::io;
use std::task::{Context, Poll};

pub(crate) mod driver;

#[derive(Debug)]
pub(super) struct Inner(());

pub(crate) type OsStorage = Vec<SignalInfo>;

impl Init for OsStorage {
    fn init() -> Self {
        let possible = 0..=33;
        possible.map(|_| SignalInfo::default()).collect()
    }
}

impl Storage for OsStorage {
    fn event_info(&self, id: EventId) -> Option<&EventInfo> {
        self.get(id).map(|si| &si.event_info)
    }

    fn for_each<'a, F>(&'a self, f: F)
    where
        F: FnMut(&'a EventInfo),
    {
        self.iter().map(|si| &si.event_info).for_each(f)
    }
}

pub(crate) struct SignalInfo {
    event_info: EventInfo,
}

impl Default for SignalInfo {
    fn default() -> SignalInfo {
        SignalInfo {
            event_info: Default::default(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct OsExtraData {
}

impl Init for OsExtraData {
    fn init() -> Self {
        Self { }
    }
}

/// Represents the specific kind of signal to listen for.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct SignalKind(libc::c_int);

impl SignalKind {
    /// Allows for listening to any valid OS signal.
    pub fn from_raw(signum: std::os::raw::c_int) -> Self {
        Self(signum as libc::c_int)
    }

    /// Get the signal's numeric value.
    pub fn as_raw_value(&self) -> std::os::raw::c_int {
        self.0
    }
}

impl From<std::os::raw::c_int> for SignalKind {
    fn from(signum: std::os::raw::c_int) -> Self {
        Self::from_raw(signum as libc::c_int)
    }
}

impl From<SignalKind> for std::os::raw::c_int {
    fn from(kind: SignalKind) -> Self {
        kind.as_raw_value()
    }
}

/// A stream of events for receiving a particular type of OS signal.
///
#[must_use = "streams do nothing unless polled"]
#[derive(Debug)]
pub struct Signal {
}

/// Creates a new stream which will receive notifications when the current
/// process receives the specified signal `kind`.
///
pub fn signal(kind: SignalKind) -> io::Result<Signal> {
    unimplemented!();
}

impl Signal {
    /// Receives the next signal notification event.
    ///
    pub async fn recv(&mut self) -> Option<()> {
        unimplemented!();
    }

    /// Polls to receive the next signal notification event, outside of an
    /// `async` context.
    ///
    pub fn poll_recv(&mut self, cx: &mut Context<'_>) -> Poll<Option<()>> {
        unimplemented!();
    }
}

// Work around for abstracting streams internally
pub(crate) trait InternalStream {
    fn poll_recv(&mut self, cx: &mut Context<'_>) -> Poll<Option<()>>;
}

impl InternalStream for Signal {
    fn poll_recv(&mut self, cx: &mut Context<'_>) -> Poll<Option<()>> {
        self.poll_recv(cx)
    }
}

pub(crate) fn ctrl_c() -> io::Result<Signal> {
    unimplemented!();
}

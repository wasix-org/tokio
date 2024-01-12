#![cfg_attr(target_os = "wasi", allow(unused, dead_code))]

/// Marker for types that are `Sync` but not `Send`
pub(crate) struct SyncNotSend(*mut ());

unsafe impl Sync for SyncNotSend {}

cfg_rt! {
    pub(crate) struct NotSendOrSync(*mut ());
}

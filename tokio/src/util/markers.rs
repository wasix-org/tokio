#![cfg_attr(target_os = "wasi", allow(unused, dead_code))]

/// Marker for types that are `Sync` but not `Send`
#[allow(dead_code)]
pub(crate) struct SyncNotSend(#[allow(dead_code)] *mut ());

unsafe impl Sync for SyncNotSend {}

cfg_rt! {
    pub(crate) struct NotSendOrSync(#[allow(dead_code)] *mut ());
}

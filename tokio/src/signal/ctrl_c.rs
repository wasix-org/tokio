#[cfg(any(unix, target_vendor = "wasmer"))]
use super::unix::{self as os_impl};
#[cfg(windows)]
use super::windows::{self as os_impl};

use std::io;

/// Completes when a "ctrl-c" notification is sent to the process.
///
/// While signals are handled very differently between Unix and Windows, both
/// platforms support receiving a signal on "ctrl-c". This function provides a
/// portable API for receiving this notification.
///
/// Once the returned future is polled, a listener is registered. The future
/// will complete on the first received `ctrl-c` **after** the initial call to
/// either `Future::poll` or `.await`.
///
/// # Caveats
///
/// On Unix platforms, the first time that a `Signal` instance is registered for a
/// particular signal kind, an OS signal-handler is installed which replaces the
/// default platform behavior when that signal is received, **for the duration of
/// the entire process**.
///
/// For example, Unix systems will terminate a process by default when it
/// receives a signal generated by `"CTRL+C"` on the terminal. But, when a
/// `ctrl_c` stream is created to listen for this signal, the time it arrives,
/// it will be translated to a stream event, and the process will continue to
/// execute.  **Even if this `Signal` instance is dropped, subsequent `SIGINT`
/// deliveries will end up captured by Tokio, and the default platform behavior
/// will NOT be reset**.
///
/// Thus, applications should take care to ensure the expected signal behavior
/// occurs as expected after listening for specific signals.
///
/// # Examples
///
/// ```rust,no_run
/// use tokio::signal;
///
/// #[tokio::main]
/// async fn main() {
///     println!("waiting for ctrl-c");
///
///     signal::ctrl_c().await.expect("failed to listen for event");
///
///     println!("received ctrl-c event");
/// }
/// ```
///
/// Listen in the background:
///
/// ```rust,no_run
/// tokio::spawn(async move {
///     tokio::signal::ctrl_c().await.unwrap();
///     // Your handler here
/// });
/// ```
pub async fn ctrl_c() -> io::Result<()> {
    os_impl::ctrl_c()?.recv().await;
    Ok(())
}

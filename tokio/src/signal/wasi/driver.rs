#![cfg_attr(not(feature = "rt"), allow(dead_code))]
#![allow(unused_variables)]

//! Signal driver

use crate::io::driver::{Driver as IoDriver};
use crate::park::Park;

use std::io::{self};
use std::time::Duration;

#[derive(Debug)]
pub(crate) struct Driver {
    /// Thread parker. The `Driver` park implementation delegates to this.
    park: IoDriver,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Handle {
}

// ===== impl Driver =====

impl Driver {
    pub(crate) fn new(park: IoDriver) -> io::Result<Self> {
        Ok(
            Self {
                park
            }
        )
    }

    pub(crate) fn handle(&self) -> Handle {
        Handle {
        }
    }

    fn process(&self) {
        std::thread::yield_now();
    }
}

impl Park for Driver {
    type Unpark = <IoDriver as Park>::Unpark;
    type Error = io::Error;

    fn unpark(&self) -> Self::Unpark {
        self.park.unpark()
    }

    fn park(&mut self) -> Result<(), Self::Error> {
        if let Err(err) = self.park.park() {
            std::thread::yield_now();
        }
        self.process();
        Ok(())
    }

    fn park_timeout(&mut self, duration: Duration) -> Result<(), Self::Error> {
        if let Err(err) = self.park.park_timeout(duration) {
            std::thread::yield_now();
        }
        self.process();
        Ok(())
    }

    fn shutdown(&mut self) {
        self.park.shutdown()
    }
}

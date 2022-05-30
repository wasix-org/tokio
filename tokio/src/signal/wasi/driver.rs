#![cfg_attr(not(feature = "rt"), allow(dead_code))]
#![allow(unused_variables)]

//! Signal driver

use crate::io::driver::{Driver as IoDriver};
use crate::park::Park;

use std::io::{self};
use std::time::Duration;

#[derive(Debug)]
pub(crate) struct Driver {
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Handle {
}

// ===== impl Driver =====

impl Driver {
    pub(crate) fn new(park: IoDriver) -> io::Result<Self> {
        unimplemented!();
    }

    pub(crate) fn handle(&self) -> Handle {
        unimplemented!();
    }
}

impl Park for Driver {
    type Unpark = <IoDriver as Park>::Unpark;
    type Error = io::Error;

    fn unpark(&self) -> Self::Unpark {
        unimplemented!();
    }

    fn park(&mut self) -> Result<(), Self::Error> {
        unimplemented!();
    }

    fn park_timeout(&mut self, duration: Duration) -> Result<(), Self::Error> {
        unimplemented!();
    }

    fn shutdown(&mut self) {
        unimplemented!();
    }
}

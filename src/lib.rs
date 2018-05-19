extern crate embedded_hal as hal;
extern crate nb;
extern crate serial;

use serial::prelude::*;
use std::ffi::OsStr;
use std::io::prelude::*;

pub use serial::BaudRate;
pub use serial::CharSize;
pub use serial::FlowControl;
pub use serial::Parity;
pub use serial::PortSettings;
pub use serial::StopBits;

use std::cell::RefCell;
use std::rc::Rc;

/// Newtype over [`serial-rs`](https://crates.io/crates/serial)'s serial port abstraction.
pub struct Serial {
    inner: Rc<RefCell<serial::SystemPort>>,
}

pub struct Tx {
    inner: Rc<RefCell<serial::SystemPort>>,
}

pub struct Rx {
    inner: Rc<RefCell<serial::SystemPort>>,
}

impl Serial {
    pub fn new<T: AsRef<OsStr> + ?Sized>(
        port: &T,
        settings: &serial::PortSettings,
    ) -> serial::Result<Self> {
        let mut port = serial::open(&port)?;
        port.configure(settings)?;
        Ok(Serial {
            inner: Rc::new(RefCell::new(port)),
        })
    }

    pub fn split(self) -> (Tx, Rx) {
        (
            Tx {
                inner: Rc::clone(&self.inner),
            },
            Rx {
                inner: Rc::clone(&self.inner),
            },
        )
    }
}

impl hal::serial::Read<u8> for Rx {
    type Error = serial::Error;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        let mut buf: [u8; 1] = [0];
        let mut inner = (*self.inner).borrow_mut();
        let _n = match inner.read(&mut buf) {
            Ok(s) => s,
            Err(e) => {
                return Err(nb::Error::Other(serial::Error::new(
                    serial::ErrorKind::Io(e.kind()),
                    "bad read",
                )));
            }
        };
        Ok(buf[0])
    }
}

impl hal::serial::Write<u8> for Tx {
    type Error = serial::Error;

    fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        let mut inner = (*self.inner).borrow_mut();
        match inner.write(&[byte]) {
            Ok(_) => Ok(()),
            Err(e) => Err(nb::Error::Other(serial::Error::new(
                serial::ErrorKind::Io(e.kind()),
                "bad write",
            ))),
        }
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        let mut inner = (*self.inner).borrow_mut();
        match inner.flush() {
            Ok(_) => Ok(()),
            Err(e) => Err(nb::Error::Other(serial::Error::new(
                serial::ErrorKind::Io(e.kind()),
                "bad flush",
            ))),
        }
    }
}

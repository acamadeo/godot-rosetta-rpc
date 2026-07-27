//! The one hand-written seam the generated Bootstrap expects: an
//! implementation of `ServiceImplementations` for every service this
//! language implements (just Clock — GameService is implemented in Kotlin,
//! see ../../kotlin/).

use crate::clock_impl::ClockImpl;
use crate::rosetta::example::{Clock, GeneratedServiceFactory, ServiceImplementations};

pub struct AppServiceImplementations;

impl ServiceImplementations for AppServiceImplementations {
    fn clock(&self, _factory: &GeneratedServiceFactory) -> Option<Box<dyn Clock>> {
        Some(Box::new(ClockImpl))
    }
}

//! The one hand-written seam the generated Bootstrap expects: an
//! implementation of `ServiceImplementations` for every service this
//! language implements (just Clock — GameService is implemented in Kotlin,
//! see ../../kotlin/).

use crate::clock_impl::ClockImpl;
use crate::profiler_impl::ProfilerImpl;
use crate::rosetta::example::{Clock, GeneratedServiceFactory, Profiler, ServiceImplementations};

pub struct AppServiceImplementations;

impl ServiceImplementations for AppServiceImplementations {
    fn clock(&self, _factory: &GeneratedServiceFactory) -> Option<Box<dyn Clock>> {
        Some(Box::new(ClockImpl))
    }

    fn profiler(&self, factory: &GeneratedServiceFactory) -> Option<Box<dyn Profiler>> {
        Some(Box::new(ProfilerImpl::new(factory.clone())))
    }
}

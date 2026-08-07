//! The one hand-written seam the generated Bootstrap expects: an
//! implementation of `ServiceImplementations` for every service this
//! language implements (Clock, Profiler, and BrokenRust).

use crate::broken_rust_impl::BrokenRustImpl;
use crate::clock_impl::ClockImpl;
use crate::profiler_impl::ProfilerImpl;
use crate::rosetta::example::{
    BrokenRust, Clock, GeneratedServiceFactory, Profiler, ServiceImplementations,
};

pub struct AppServiceImplementations;

impl ServiceImplementations for AppServiceImplementations {
    fn clock(&self, _factory: &GeneratedServiceFactory) -> Option<Box<dyn Clock>> {
        Some(Box::new(ClockImpl))
    }

    fn profiler(&self, factory: &GeneratedServiceFactory) -> Option<Box<dyn Profiler>> {
        Some(Box::new(ProfilerImpl::new(factory.clone())))
    }

    fn broken_rust(&self, _factory: &GeneratedServiceFactory) -> Option<Box<dyn BrokenRust>> {
        Some(Box::new(BrokenRustImpl))
    }
}

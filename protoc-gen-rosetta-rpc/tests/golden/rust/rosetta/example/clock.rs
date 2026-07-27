// GENERATED CODE — do not edit. Source: rosetta.example.Clock
// Not `Send + Sync`: implementations commonly hold a `Gd<T>` handle, and
// Godot objects are inherently single-threaded.
pub trait Clock {

    fn current_time(&self, request: protobuf_gen::rosetta::example::CurrentTimeRequest) -> protobuf_gen::rosetta::example::CurrentTimeResponse;

}
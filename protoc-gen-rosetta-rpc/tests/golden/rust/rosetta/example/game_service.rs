// GENERATED CODE — do not edit. Source: rosetta.example.GameService
// Not `Send + Sync`: implementations commonly hold a `Gd<T>` handle, and
// Godot objects are inherently single-threaded.
pub trait GameService {

    fn ping(&self, request: protobuf_gen::rosetta::example::PingRequest) -> protobuf_gen::rosetta::example::PingResponse;

}
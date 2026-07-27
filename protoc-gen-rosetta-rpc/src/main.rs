//! `protoc-gen-rosetta-rpc` — a custom protoc plugin that generates
//! cross-language RPC glue (service interfaces, method descriptors, clients,
//! adapters, a service factory, and a bootstrap) for the godot-rosetta-rpc
//! framework.
//!
//! Invoked by protoc as e.g.
//! `--rosetta-rpc_out=lang=rust,message_crate=protobuf_gen:out_dir`. protoc
//! writes an encoded `CodeGeneratorRequest` to this process's stdin and reads
//! an encoded `CodeGeneratorResponse` from its stdout.

mod generator;
mod ir;
mod kotlin_gen;
mod naming;
mod options;
mod rust_gen;
mod view;

use std::io::{Read, Write};

use prost::Message;
use prost_types::compiler::{CodeGeneratorRequest, CodeGeneratorResponse};

use options::Options;

fn main() {
    if let Err(err) = run() {
        // protoc surfaces `error` to the user and treats the invocation as
        // failed; nothing should ever be written to stdout in that case.
        let response = CodeGeneratorResponse {
            error: Some(err),
            file: Vec::new(),
            supported_features: None,
        };
        write_response(&response).expect("failed to write error response to stdout");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let request = read_request().map_err(|e| format!("failed to read request: {e}"))?;
    let options = Options::parse(request.parameter.as_deref().unwrap_or(""))?;
    let ir = ir::extract(&request);
    let files = generator::generate(&ir, &options)?;

    let response = CodeGeneratorResponse {
        error: None,
        file: files,
        supported_features: None,
    };
    write_response(&response).map_err(|e| format!("failed to write response: {e}"))?;
    Ok(())
}

fn read_request() -> std::io::Result<CodeGeneratorRequest> {
    let mut buf = Vec::new();
    std::io::stdin().read_to_end(&mut buf)?;
    CodeGeneratorRequest::decode(buf.as_slice())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

fn write_response(response: &CodeGeneratorResponse) -> std::io::Result<()> {
    let mut buf = Vec::new();
    response
        .encode(&mut buf)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    std::io::stdout().write_all(&buf)
}

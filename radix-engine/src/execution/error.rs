use std::fmt;

use sbor::*;
use scrypto::types::*;
use wasmi::*;

use crate::model::*;

#[derive(Debug)]
pub enum ExecutionError {
    RuntimeError(Error),

    MemoryAccessError(Error),

    NoValidBlueprintReturn,

    InvalidOpCode(u32),

    InvalidRequest(DecodeError),

    UnknownHostFunction(usize),

    UnableToAllocateMemory,

    ResourceLeak(Vec<BID>),

    BlueprintAlreadyExists(Address),

    ComponentAlreadyExists(Address),

    ResourceAlreadyExists(Address),

    ComponentNotFound(Address),

    ResourceNotFound(Address),

    ImmutableResource,

    NotAuthorizedToMint,

    BucketNotFound,

    BucketRefNotFound,

    BucketOperationError(BucketError),
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl HostError for ExecutionError {}
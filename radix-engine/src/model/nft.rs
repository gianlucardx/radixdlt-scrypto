use sbor::*;
use scrypto::rust::vec::Vec;

/// Represents an error when accessing a NFT.
#[derive(Debug, Clone)]
pub enum NftError {
    UnauthorizedAccess,
}

/// An nft is a peirece of data that is uniquely identified within a resource.
#[derive(Debug, Clone, TypeId, Encode, Decode)]
pub struct Nft {
    immutable_data: Vec<u8>,
    mutable_data: Vec<u8>,
}

impl Nft {
    pub fn new(immutable_data: Vec<u8>, mutable_data: Vec<u8>) -> Self {
        Self {
            immutable_data,
            mutable_data,
        }
    }

    pub fn immutable_data(&self) -> Vec<u8> {
        self.immutable_data.clone()
    }

    pub fn mutable_data(&self) -> Vec<u8> {
        self.mutable_data.clone()
    }

    pub fn set_mutable_data(&mut self, new_mutable_data: Vec<u8>) -> Result<(), NftError> {
        self.mutable_data = new_mutable_data;
        Ok(())
    }
}

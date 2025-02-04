mod bucket;
mod bucket_ref;
mod nft;
mod nft_data;
mod resource_builder;
mod resource_def;
mod vault;

/// Resource feature flags.
pub mod resource_flags;

/// Various resource permissions.
pub mod resource_permissions;

pub use bucket::Bucket;
pub use bucket_ref::BucketRef;
pub use nft::Nft;
pub use nft_data::NftData;
pub use resource_builder::{ResourceBuilder, DIVISIBILITY_MAXIMUM, DIVISIBILITY_NONE};
pub use resource_def::ResourceDef;
pub use resource_flags::*;
pub use resource_permissions::*;
pub use vault::Vault;

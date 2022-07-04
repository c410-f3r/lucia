#![allow(
  // Global types generally used by more than one API
  dead_code
)]

use crate::{
  consts::{MAX_ASSET_ABBR_LEN, MAX_NUMBER_LEN},
  utils::UrlPartsArrayString,
};

pub(crate) type Id = usize;
pub(crate) type MaxAssetAbbr = arrayvec::ArrayString<MAX_ASSET_ABBR_LEN>;
pub(crate) type MaxAssetName = arrayvec::ArrayString<16>;
pub(crate) type MaxPairAbbr = arrayvec::ArrayString<{ 2 * MAX_ASSET_ABBR_LEN + 1 }>;
pub(crate) type MaxUrl = arrayvec::ArrayString<96>;
pub(crate) type MaxUrlParts = UrlPartsArrayString<96>;
pub(crate) type MaxNumberStr = arrayvec::ArrayString<MAX_NUMBER_LEN>;

_create_blockchain_constants!(
  address_hash: MaxAddressHash = 32,
  block_hash: MaxBlockHash = 32,
  signature_hash: MaxSignatureHash = 64,
  transaction_hash: MaxTransactionHash = 64,
  //
  address_hash_str: MaxAddressHashStr = 46,
  block_hash_str: MaxBlockHashStr = 67,
  signature_hash_str: MaxSignatureHashStr = 90,
  transaction_hash_str: MaxTransactionHashStr = 90
);

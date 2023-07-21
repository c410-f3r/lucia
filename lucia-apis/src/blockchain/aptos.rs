//! Aptos is a public blockchain platform with smart contract functionality.
//!
//! <https://fullnode.devnet.aptoslabs.com/v1>
//!
//! ```rust,no_run
//! # async fn fun() -> lucia_apis::Result<()> {
//! use lucia::{dnsn::SerdeJson, network::HttpParams};
//! use lucia_apis::{blockchain::aptos::Aptos, misc::PkgsAux};
//!
//! let mut pkgs_aux =
//!   PkgsAux::from_minimum(Aptos::new(None), SerdeJson, HttpParams::from_url("URL")?);
//! let _ = pkgs_aux.check_basic_node_health().params(None).build();
//! # Ok(()) }
//! ```

#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;
mod pkg;

use lucia::{misc::RequestThrottling, network::HttpResParams, Api};
pub use pkg::*;

#[derive(Debug)]
#[doc = _generic_api_doc!()]
#[lucia_macros::api_types(pkgs_aux(crate::misc::PkgsAux), transport(http))]
pub struct Aptos {
  /// See [FormattedHttpResponseHeaders].
  pub fhrh: FormattedHttpResponseHeaders,
  /// If some, tells that each request must respect calling intervals.
  pub rt: Option<RequestThrottling>,
}

impl Aptos {
  /// If desired, it is possible to instantiate directly instead of using this method.
  pub fn new(rt: Option<RequestThrottling>) -> Self {
    Self { fhrh: <_>::default(), rt }
  }
}

#[cfg_attr(feature = "async-trait", async_trait::async_trait)]
impl Api for Aptos {
  type Error = crate::Error;

  async fn before_sending(&mut self) -> Result<(), Self::Error> {
    self.fhrh.clear();
    if let Some(ref mut rt) = self.rt {
      rt.rc.update_params(&rt.rl).await?;
    }
    Ok(())
  }
}

#[derive(Debug, Default)]
/// Relevant information extracted from HTTP response headers.
pub struct FormattedHttpResponseHeaders {
  /// Current block height of the chain
  pub x_aptos_block_height: u64,
  /// Chain id of the current chain
  pub x_aptos_chain_id: u8,
  /// Current epoch of the chain
  pub x_aptos_epoch: u64,
  /// Oldest non_pruned ledger version of the chain
  pub x_aptos_ledger_oldest_version: u64,
  /// Current timestamp of the chain
  pub x_aptos_ledger_timestampusec: u64,
  /// Current ledger version of the chain
  pub x_aptos_ledger_version: u64,
  /// Oldest non_pruned block height of the chain
  pub x_aptos_oldest_block_height: u64,
  /// Cursor to be used for endpoints that support cursor-based pagination. Pass this to the start field of the endpoint on the next call to get the next page of results.
  pub x_aptos_cursor: Option<String>,
}

impl FormattedHttpResponseHeaders {
  pub(crate) fn clear(&mut self) {
    let Self {
      x_aptos_block_height,
      x_aptos_chain_id,
      x_aptos_epoch,
      x_aptos_ledger_oldest_version,
      x_aptos_ledger_timestampusec,
      x_aptos_ledger_version,
      x_aptos_oldest_block_height,
      x_aptos_cursor,
    } = self;
    *x_aptos_block_height = 0;
    *x_aptos_chain_id = 0;
    *x_aptos_epoch = 0;
    *x_aptos_ledger_oldest_version = 0;
    *x_aptos_ledger_timestampusec = 0;
    *x_aptos_ledger_version = 0;
    *x_aptos_oldest_block_height = 0;
    if let Some(el) = x_aptos_cursor {
      el.clear();
    }
  }

  pub(crate) fn eval(&mut self, hrp: &HttpResParams) -> crate::Result<()> {
    let Self {
      x_aptos_block_height,
      x_aptos_chain_id,
      x_aptos_epoch,
      x_aptos_ledger_oldest_version,
      x_aptos_ledger_timestampusec,
      x_aptos_ledger_version,
      x_aptos_oldest_block_height,
      x_aptos_cursor,
    } = self;
    let mut counter: u8 = 0;
    for (key, value) in hrp.headers.iter() {
      match key {
        "x-aptos-block-height" => {
          *x_aptos_block_height = value.parse().map_err(lucia::Error::from)?;
          counter = counter.wrapping_add(1);
        }
        "x-aptos-chain-id" => {
          *x_aptos_chain_id = value.parse().map_err(lucia::Error::from)?;
          counter = counter.wrapping_add(1);
        }
        "x-aptos-cursor" => {
          if let Some(el) = x_aptos_cursor {
            el.push_str(value);
          } else {
            *x_aptos_cursor = Some(value.to_owned());
          }
        }
        "x-aptos-epoch" => {
          *x_aptos_epoch = value.parse().map_err(lucia::Error::from)?;
          counter = counter.wrapping_add(1);
        }
        "x-aptos-ledger-oldest-version" => {
          *x_aptos_ledger_oldest_version = value.parse().map_err(lucia::Error::from)?;
          counter = counter.wrapping_add(1);
        }
        "x-aptos-ledger-timestampusec" => {
          *x_aptos_ledger_timestampusec = value.parse().map_err(lucia::Error::from)?;
          counter = counter.wrapping_add(1);
        }
        "x-aptos-ledger-version" => {
          *x_aptos_ledger_version = value.parse().map_err(lucia::Error::from)?;
          counter = counter.wrapping_add(1);
        }
        "x-aptos-oldest-block-height" => {
          *x_aptos_oldest_block_height = value.parse().map_err(lucia::Error::from)?;
          counter = counter.wrapping_add(1);
        }
        _ => {}
      }
    }
    if counter != 7 {
      return Err(crate::Error::MandatoryResponseHeadersWereNotFound);
    }
    Ok(())
  }
}

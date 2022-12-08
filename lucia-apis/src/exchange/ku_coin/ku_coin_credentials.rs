use arrayvec::{ArrayString, ArrayVec};
use core::{
  fmt::{Debug, Formatter},
  str,
};
use hmac::{Hmac, Mac};
use lucia::{misc::GenericTime, network::HttpReqParams};
use sha2::Sha256;

// 256 because of SHA-2
const SIGNATURE_MAX_LEN: usize = (256 / 3 + 1) * 4;

/// Used to sign requests in private endpoints.
pub struct KuCoinCredentials {
  api_key: String,
  api_pw: String,
  api_secret: String,
}

impl KuCoinCredentials {
  /// Loads credential values from pre-defined environment variables.
  ///
  /// * KU_COIN_KEY
  /// * KU_COIN_PW
  /// * KU_COIN_SECRET
  #[cfg(feature = "std")]
  #[inline]
  pub fn from_default_env_vars() -> crate::Result<Self> {
    Self::from_env_vars("KU_COIN_KEY", "KU_COIN_PW", "KU_COIN_SECRET")
  }

  /// Loads environment variables that are expected to have credential values.
  #[cfg(feature = "std")]
  #[inline]
  pub fn from_env_vars(api_key: &str, api_pw: &str, api_secret: &str) -> crate::Result<Self> {
    Self::new(std::env::var(api_key)?, std::env::var(api_pw)?, std::env::var(api_secret)?)
  }

  /// Before signing requests, you must create API keys in the KuCoin website and provide them
  /// here to sign requests.
  #[inline]
  pub fn new(api_key: String, api_pw: String, api_secret: String) -> crate::Result<Self> {
    Ok(Self { api_key, api_pw, api_secret })
  }

  #[inline]
  pub(crate) fn push_headers(
    &self,
    body: &[u8],
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    let timestamp: i64 = GenericTime::now()?.timestamp()?.as_millis().try_into()?;
    let sign_value = self.sign(body, req_params, timestamp)?;
    req_params.headers.push_str("KC-API-KEY", self.api_key.as_ref())?;
    req_params.headers.push_str("KC-API-KEY-VERSION", "2")?;
    req_params.headers.push_str("KC-API-PASSPHRASE", self.sign_api_pw()?.as_str())?;
    req_params.headers.push_str("KC-API-SIGN", sign_value.as_str())?;
    req_params
      .headers
      .push_fmt(format_args!("{}", "KC-API-TIMESTAMP"), format_args!("{}", timestamp))?;
    Ok(())
  }

  #[inline]
  fn sign(
    &self,
    body: &[u8],
    req_params: &mut HttpReqParams,
    timestamp: i64,
  ) -> crate::Result<ArrayString<SIGNATURE_MAX_LEN>> {
    let mut mac = Hmac::<Sha256>::new_from_slice(self.api_secret.as_str().as_bytes())?;
    mac.update(ArrayString::<20>::try_from(format_args!("{}", timestamp))?.as_bytes());
    mac.update(<&str>::from(req_params.method).as_bytes());
    mac.update(req_params.url.href().as_bytes());
    mac.update(body);
    let input = mac.finalize().into_bytes();
    encode_to_base64(input.as_slice())
  }

  #[inline]
  fn sign_api_pw(&self) -> crate::Result<ArrayString<SIGNATURE_MAX_LEN>> {
    let mut mac = Hmac::<Sha256>::new_from_slice(self.api_secret.as_str().as_bytes())?;
    mac.update(self.api_pw.as_str().as_bytes());
    let input = mac.finalize().into_bytes();
    encode_to_base64(input.as_slice())
  }
}

impl Debug for KuCoinCredentials {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("KuCoinCredentials").finish()
  }
}

#[inline(always)]
fn encode_to_base64<const N: usize>(bytes: &[u8]) -> crate::Result<ArrayString<N>> {
  let mut buffer = ArrayVec::from([0; N]);
  let len = base64::encode_config_slice(bytes, base64::STANDARD, &mut buffer);
  buffer.truncate(len);
  let rslt = str::from_utf8(&buffer)
    .ok()
    .and_then(|str| ArrayString::try_from(str).ok())
    .ok_or(arrayvec::CapacityError::new(()))?;
  Ok(rslt)
}

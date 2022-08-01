use crate::{RequestParamsModifier, RequestResponse};

/// All communication between the parties of all APIs happens through the structures that implement
/// this trait.
///
/// Can have multiple request parameters as well as multiple response parameters.
///
/// # Types
///
/// * `CP`: **C**ommon **P**arameters
/// * `REQP`: **REQ**uest **P**arameters
/// * `RESP`: **RES**ponse **P**arameters
pub trait Request<CP, REQP, RESP>: RequestParamsModifier<CP, REQP> + RequestResponse<RESP> {}

impl<CP, REQP, RESP, T> Request<CP, REQP, RESP> for T where
  T: RequestParamsModifier<CP, REQP> + RequestResponse<RESP>
{
}

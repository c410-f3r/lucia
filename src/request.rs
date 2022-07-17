mod request_data;
mod request_manager;
mod request_params;
mod request_with_aux;
mod requests;

pub use request_data::*;
pub use request_manager::*;
pub use request_params::*;
pub use request_with_aux::*;
pub use requests::*;

/// All communication between the parties of all APIs happens through the structures that implement
/// this trait.
///
/// Composed by the union of [RequestData] and [RequestParams].
pub trait Request<CP, RPD>: RequestData + RequestParams<CP, RPD> {}

impl<CP, RPD, T> Request<CP, RPD> for T where T: RequestData + RequestParams<CP, RPD> {}

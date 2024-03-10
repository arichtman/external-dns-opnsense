use serde::Serialize;

#[derive(Debug, Clone)]
pub enum InternalDataError {
    NotFound,
    AmbiguousResult,
    GenericError,
}

#[derive(Debug, Clone, Serialize)]
pub enum OPNSenseError {
    GenericFailure,
    UnconvertibleData,
    NoData,
}

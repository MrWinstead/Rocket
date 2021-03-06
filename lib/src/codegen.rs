use handler::{Handler, ErrorHandler};
use http::{Method, ContentType};

pub struct StaticRouteInfo {
    pub method: Method,
    pub path: &'static str,
    pub format: Option<ContentType>,
    pub handler: Handler,
    pub rank: Option<isize>,
}

pub struct StaticCatchInfo {
    pub code: u16,
    pub handler: ErrorHandler,
}

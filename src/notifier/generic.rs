// Vigil
//
// Microservices Status Page
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::time::SystemTime;

use prober::status::Status;

pub const DISPATCH_TIMEOUT_SECONDS: u64 = 10;

pub struct Notification<'a> {
    pub status: &'a Status,
    pub time: SystemTime,
    pub replicas: Vec<&'a str>,
}

pub trait GenericNotifier {
    fn dispatch(notification: &Notification) -> Result<(), bool>;
    fn is_enabled() -> bool;
}
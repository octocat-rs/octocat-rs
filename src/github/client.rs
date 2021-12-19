use crate::github::{handler::EventHandler, util::Authorization};

// TODO: Client trait, method impls
/// Where the magic happens.
pub struct Client<T>
where
    T: std::fmt::Debug + EventHandler,
{
    handler: T,
    authorization: Authorization,
}

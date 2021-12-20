use crate::github::{handler::EventHandler, util::Authorization, DefaultEventHandler};

// TODO: Client trait, method impls
/// Where the magic happens.
#[allow(dead_code)]
pub struct Client<T>
where
    T: std::fmt::Debug + EventHandler,
{
    handler: T,
    authorization: Authorization,
}

impl<T> Client<T>
where
    T: std::fmt::Debug + EventHandler,
{
    pub fn new(handler: T, auth: Authorization) -> Self {
        Self {
            handler,
            authorization: auth,
        }
    }

    pub fn set_auth(self, auth: Authorization) -> Self {
        Self {
            handler: self.handler,
            authorization: auth,
        }
    }
}

impl Default for Client<DefaultEventHandler> {
    fn default() -> Client<DefaultEventHandler> {
        Client {
            handler: DefaultEventHandler,
            authorization: Authorization::default(),
        }
    }
}

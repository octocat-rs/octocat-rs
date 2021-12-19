use futures::future;

pub type BoxFuture<T> = future::BoxFuture<'static, T>;

pub struct Command<T>
where
    T: std::fmt::Debug + Send,
{
    to_be_performed: Vec<BoxFuture<T>>,
}

impl<T> Command<T>
where
    T: std::fmt::Debug + Send,
{
    pub fn none() -> Self {
        Self {
            to_be_performed: Vec::new(),
        }
    }
}

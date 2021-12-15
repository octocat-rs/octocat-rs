use std::{panic, process};

pub struct GitHub {
    #[allow(dead_code)]
    api_key: String,
}

impl GitHub {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_owned()
        }
    }

    pub async fn run(&self) {
        pretty_env_logger::init();

        panic::set_hook(Box::new(|msg| {
            match msg.payload().downcast_ref::<&str>() {
                Some(msg) => error!("Panicked at: {}", msg),
                _ => error!("Error occurred")
            }

            if let Some(loc) = msg.location() { error!("Location: {}:{}:{}", loc.file(), loc.line(), loc.column()) }

            process::exit(1);
        }));

        todo!()
    }
}
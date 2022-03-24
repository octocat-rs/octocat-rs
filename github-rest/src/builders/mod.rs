//! This module includes various builders for Methods that require a lot of
//! information

use crate::{GithubRestError, Requester};
use async_trait::async_trait;
pub use commits::*;
pub use gists::*;
pub use issues::*;
pub use pull_requests::*;
pub use reactions::*;
use serde::de::DeserializeOwned;

mod commits;
mod gists;
mod issues;
mod pull_requests;
mod reactions;

#[async_trait]
pub trait Builder {
    type Response: DeserializeOwned;

    async fn execute<T>(self, client: &T) -> Result<Self::Response, GithubRestError>
    where
        T: Requester;
}

/// Define a struct. This expands to:
///
///     #[derive(Default, Clone)]
///     pub struct S {
///         a: Type,
///         b: Type,
///         c: Type,
///     }
///
///     impl S {
///         pub fn new() -> Self {
///             Self::default()
///         }
///     }
macro_rules! builder {
    (
        $(#[$attr:meta])*
        $name:ident { $($field:ident: $field_type:ty),* }
    ) => {

        $(#[$attr])*
        #[derive(Default, Clone)]
        pub struct $name {
            $(
                $field: $field_type,
            )*
        }

        impl $name {
            pub fn new() -> Self { Self::default() }
        }
    }
}

/// Build an impl block with setters. This expands to:
///
///     impl S {
///         pub fn a(mut self, v: A) -> Self {
///             self.a = v;
///             self
///         }
///     }
macro_rules! builder_setters {
    ($name:ident { $($field:ident: $field_type:ty),* }) => {
        paste::paste! {
            impl $name {
                $(
                    pub fn $field(mut self, $field: $field_type) -> Self {
                        self.$field = $field;
                        self
                    }
                )*
            }
        }
    }
}

/// Build an impl block with setters. This expands to:
///
///     impl S {
///         pub fn a<T: Into<String>>(mut self, v: T) -> Self {
///             self.a = v.into();
///             self
///         }
///     }
macro_rules! builder_string_setters {
    ($name:ident { $($field:ident),* }) => {
        paste::paste! {
            impl $name {
                $(
                    pub fn $field<T: Into<String>>(mut self, $field: T) -> Self {
                        self.$field = $field.into();
                        self
                    }
                )*
            }
        }
    }
}

/// Build an impl block with setters. This expands to:
///
///     impl S {
///         pub fn a<T: Into<String>>(mut self, v: T) -> Self {
///             self.f.a = Some(v.into());
///             self
///         }
///     }
macro_rules! builder_nested_string_setters {
    ($name:ident { $field:ident { $($subfield:ident),* } }) => {
        paste::paste! {
            impl $name {
                $(
                    pub fn $subfield<T: Into<String>>(mut self, $subfield: T) -> Self {
                        self.$field.$subfield = Some($subfield.into());
                        self
                    }
                )*
            }
        }
    }
}

/// Build an impl block with setters. This expands to:
///
///     impl S {
///         pub fn a<T: Into<String>>(mut self, v: T) -> Self {
///             self.f.a = v.into();
///             self
///         }
///     }
macro_rules! builder_nested_string_setters_required {
    ($name:ident { $field:ident { $($subfield:ident),* } }) => {
        paste::paste! {
            impl $name {
                $(
                    pub fn $subfield<T: Into<String>>(mut self, $subfield: T) -> Self {
                        self.$field.$subfield = $subfield.into();
                        self
                    }
                )*
            }
        }
    }
}

/// Build an impl block with setters. This expands to:
///
///     impl S {
///         pub fn a(mut self, v: A) -> Self {
///             self.f.a = Some(v);
///             self
///         }
///     }
macro_rules! builder_nested_setters {
    ($name:ident { $field:ident { $($subfield:ident: $subfield_type:ty),* } }) => {
        paste::paste! {
            impl $name {
                $(
                    pub fn $subfield(mut self, $subfield: $subfield_type) -> Self {
                        self.$field.$subfield = Some($subfield);
                        self
                    }
                )*
            }
        }
    }
}

/// Build an impl block with setters. This expands to:
///
///     impl S {
///         pub fn a(mut self, v: A) -> Self {
///             self.f.a = v;
///             self
///         }
///     }
macro_rules! builder_nested_setters_non_optional {
    ($name:ident { $field:ident { $($subfield:ident: $subfield_type:ty),* } }) => {
        paste::paste! {
            impl $name {
                $(
                    pub fn $subfield(mut self, $subfield: $subfield_type) -> Self {
                        self.$field.$subfield = $subfield;
                        self
                    }
                )*
            }
        }
    }
}

pub(crate) use builder;
pub(crate) use builder_nested_setters;
pub(crate) use builder_nested_setters_non_optional;
pub(crate) use builder_nested_string_setters;
pub(crate) use builder_nested_string_setters_required;
pub(crate) use builder_setters;
pub(crate) use builder_string_setters;

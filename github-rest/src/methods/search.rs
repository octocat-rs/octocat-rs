use crate::{methods::prelude::*, model::search::RepoSearchResultItem};
use std::ops::Range;

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchRepositoriesResponse {
    pub total_count: usize,
    pub incomplete_results: bool,
    pub items: Vec<RepoSearchResultItem>,
}

#[derive(Debug, Default, Clone)]
pub struct SearchRepositoriesBody {
    /// Size of repository in kilobytes.
    pub size: Option<Range<usize>>,
    pub followers: Option<Range<usize>>,
    pub forks: Option<Range<usize>>,
    pub stars: Option<Range<usize>>,
}

impl SearchRepositoriesBody {
    pub(crate) fn into_query(self) -> String {
        let mut ret = "".to_owned();

        macro_rules! push_or_skip {
            ($($field:ident),*) => {
                paste::paste! {
                    $(
                        match &self.$field {
                            Some(range) => {
                                let mut val = stringify!($field).to_owned();
                                val.push_str(format!(":{range:?}").as_str());

                                ret.push_str(val.as_str())
                            },
                            None => {}
                        }

                        ret.push_str(" ");
                    )*
                }
            }
        }

        push_or_skip!(size, followers, forks, stars);

        ret
    }
}

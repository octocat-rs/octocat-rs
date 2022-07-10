use crate::{
    methods::prelude::*,
    model::search::{IssueSearchResultItem, RepoSearchResultItem},
};
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
    /// Number of repository topics.
    pub topics: Option<Range<usize>>,
    /// Number of issues with the "help wanted" label.
    pub help_wanted_issues: Option<Range<usize>>,
    /// Number of issues with the "good first issue" label.
    pub good_first_issues: Option<Range<usize>>,
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
                                let mut val = format!("&{}", stringify!($field));
                                val.push_str(format!(":{range:?}").as_str());

                                ret.push_str(val.as_str())
                            },
                            None => {}
                        }
                    )*
                }
            }
        }

        push_or_skip!(
            size,
            followers,
            forks,
            stars,
            topics,
            help_wanted_issues,
            good_first_issues
        );

        ret
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchIssuesResponse {
    pub total_count: usize,
    pub incomplete_results: bool,
    pub items: Vec<IssueSearchResultItem>,
}

#[derive(Debug, Default, Clone)]
pub struct SearchIssuesBody {
    pub comments: Option<Range<usize>>,
    pub interactions: Option<Range<usize>>,
    pub reactions: Option<Range<usize>>,
}

impl SearchIssuesBody {
    pub(crate) fn into_query(self) -> String {
        let mut ret = "".to_owned();

        macro_rules! push_or_skip {
            ($($field:ident),*) => {
                paste::paste! {
                    $(
                        match &self.$field {
                            Some(range) => {
                                let mut val = format!("&{}", stringify!($field));
                                val.push_str(format!(":{}..{}", range.start, range.end).as_str());

                                ret.push_str(val.as_str())
                            },
                            None => {}
                        }
                    )*
                }
            }
        }

        push_or_skip!(comments, interactions, reactions);

        ret
    }
}

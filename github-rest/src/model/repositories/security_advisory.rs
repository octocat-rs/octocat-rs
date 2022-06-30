use crate::model::prelude::*;

pub mod events {
    use crate::model::{prelude::*, repositories::security_advisory::SecurityAdvisory};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct SecurityAdvisoryEvent {
        pub action: SecurityAdvisoryAction,
        pub security_advisory: SecurityAdvisory,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumString, EnumVariantNames)]
    #[serde(rename_all = "snake_case")]
    pub enum SecurityAdvisoryAction {
        Published,
        Updated,
        Performed,
        Withdrawn,
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecurityAdvisory {
    pub ghsa_id: String,
    pub summary: String,
    pub description: String,
    pub severity: String,
    pub identifiers: Vec<Identifier>,
    pub references: Vec<Reference>,
    pub published_at: String,
    pub updated_at: String,
    pub withdrawn_at: Value,
    pub vulnerabilities: Vec<Vulnerability>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Identifier {
    pub value: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reference {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Vulnerability {
    pub package: Package,
    pub severity: String,
    pub vulnerable_version_range: String,
    pub first_patched_version: FirstPatchedVersion,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Package {
    pub ecosystem: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FirstPatchedVersion {
    pub identifier: String,
}

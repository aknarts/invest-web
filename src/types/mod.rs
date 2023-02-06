use serde::{Deserialize, Serialize, Serializer};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use yew_hooks::UseCounterHandle;

pub mod auth;

/// Conduit api error info for Unprocessable Entity error
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}

#[derive(Default, Clone)]
pub struct WrapCounter(pub Option<UseCounterHandle>);

impl Debug for WrapCounter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            None => {
                write!(f, "No Counter")
            }
            Some(c) => {
                write!(f, "Counter [{:?}]", *c)
            }
        }
    }
}

impl PartialEq<Self> for WrapCounter {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Eq for WrapCounter {}

impl Serialize for WrapCounter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_i8(0)
    }
}

impl Ord for WrapCounter {
    fn cmp(&self, _: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl PartialOrd for WrapCounter {
    fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

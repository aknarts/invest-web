use serde::{Deserialize, Serialize, Serializer};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use yew::Callback;
use yew_hooks::UseCounterHandle;

pub mod auth;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
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

#[derive(Default, Clone)]
pub struct WrapCallback(pub Option<Callback<String>>);

impl Debug for WrapCallback {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            None => {
                write!(f, "No callback")
            }
            Some(_) => {
                write!(f, "Callback")
            }
        }
    }
}

impl PartialEq<Self> for WrapCallback {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Eq for WrapCallback {}

impl Serialize for WrapCallback {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i8(0)
    }
}

impl Ord for WrapCallback {
    fn cmp(&self, _: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl PartialOrd for WrapCallback {
    fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrap_counter_debug() {
        let wrap = WrapCounter(None);
        assert_eq!(format!("{wrap:?}"), "No Counter")
    }

    #[test]
    fn wrap_counter_eq() {
        let wrap = WrapCounter(None);
        let wrap2 = WrapCounter(None);
        assert_eq!(wrap.eq(&wrap2), true)
    }

    #[test]
    fn wrap_counter_cmp() {
        let wrap = WrapCounter(None);
        let wrap2 = WrapCounter(None);
        assert_eq!(wrap.cmp(&wrap2), Ordering::Equal)
    }

    #[test]
    fn wrap_counter_partial_cmp() {
        let wrap = WrapCounter(None);
        let wrap2 = WrapCounter(None);
        assert_eq!(wrap.partial_cmp(&wrap2), Some(Ordering::Equal))
    }

    #[test]
    fn wrap_callback_debug() {
        let wrap = WrapCallback(None);
        assert_eq!(format!("{wrap:?}"), "No callback")
    }

    #[test]
    fn wrap_callback_eq() {
        let wrap = WrapCallback(None);
        let wrap2 = WrapCallback(None);
        assert_eq!(wrap.eq(&wrap2), true)
    }

    #[test]
    fn wrap_callback_cmp() {
        let wrap = WrapCallback(None);
        let wrap2 = WrapCallback(None);
        assert_eq!(wrap.cmp(&wrap2), Ordering::Equal)
    }

    #[test]
    fn wrap_callback_partial_cmp() {
        let wrap = WrapCallback(None);
        let wrap2 = WrapCallback(None);
        assert_eq!(wrap.partial_cmp(&wrap2), Some(Ordering::Equal))
    }
}

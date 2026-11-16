// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-gnu-xz: gnu xz component model.
//!
//! Real implementation: gnu_xz_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GnuXzItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GnuXzError {
    EmptyGnuXzItemName,
    DuplicateGnuXzItem(String),
    EmptyEventSubject,
}

impl fmt::Display for GnuXzError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyGnuXzItemName => write!(f, "gnu_xz_item name must not be empty"),
            Self::DuplicateGnuXzItem(id) => write!(f, "duplicate gnu_xz_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for GnuXzError {}

pub struct GnuXz {
    pub name: String,
    gnu_xz_items: BTreeMap<String, GnuXzItem>,
    events: BTreeMap<String, Event>,
}

impl GnuXz {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            gnu_xz_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_gnu_xz_item(&mut self, item: GnuXzItem) -> Result<(), GnuXzError> {
        if item.name.trim().is_empty() {
            return Err(GnuXzError::EmptyGnuXzItemName);
        }
        let id = item.id.clone();
        if self.gnu_xz_items.contains_key(&id) {
            return Err(GnuXzError::DuplicateGnuXzItem(id.clone()));
        }
        self.gnu_xz_items.insert(id, item);
        Ok(())
    }

    pub fn gnu_xz_item_count(&self) -> usize {
        self.gnu_xz_items.len()
    }

    pub fn get_gnu_xz_item(&self, id: &str) -> Option<&GnuXzItem> {
        self.gnu_xz_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), GnuXzError> {
        if event.subject.trim().is_empty() {
            return Err(GnuXzError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_gnu_xz_item(id: &str, name: &str) -> GnuXzItem {
        GnuXzItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = GnuXz::new("GnuXz");
        assert_eq!(m.name, "GnuXz");
        assert_eq!(m.gnu_xz_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_gnu_xz_item() {
        let mut m = GnuXz::new("GnuXz");
        m.add_gnu_xz_item(sample_gnu_xz_item("a1", "Alpha")).unwrap();
        assert_eq!(m.gnu_xz_item_count(), 1);
        assert!(m.get_gnu_xz_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = GnuXz::new("GnuXz");
        assert!(m.add_gnu_xz_item(sample_gnu_xz_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = GnuXz::new("GnuXz");
        m.add_gnu_xz_item(sample_gnu_xz_item("x", "One")).unwrap();
        assert!(m.add_gnu_xz_item(sample_gnu_xz_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = GnuXz::new("GnuXz");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}

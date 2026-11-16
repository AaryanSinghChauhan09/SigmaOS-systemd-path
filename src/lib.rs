// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-systemd-path: systemd path component model.
//!
//! Real implementation: systemd_path_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemdPathItem {
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
pub enum SystemdPathError {
    EmptySystemdPathItemName,
    DuplicateSystemdPathItem(String),
    EmptyEventSubject,
}

impl fmt::Display for SystemdPathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySystemdPathItemName => write!(f, "systemd_path_item name must not be empty"),
            Self::DuplicateSystemdPathItem(id) => write!(f, "duplicate systemd_path_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SystemdPathError {}

pub struct SystemdPath {
    pub name: String,
    systemd_path_items: BTreeMap<String, SystemdPathItem>,
    events: BTreeMap<String, Event>,
}

impl SystemdPath {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            systemd_path_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_systemd_path_item(&mut self, item: SystemdPathItem) -> Result<(), SystemdPathError> {
        if item.name.trim().is_empty() {
            return Err(SystemdPathError::EmptySystemdPathItemName);
        }
        let id = item.id.clone();
        if self.systemd_path_items.contains_key(&id) {
            return Err(SystemdPathError::DuplicateSystemdPathItem(id.clone()));
        }
        self.systemd_path_items.insert(id, item);
        Ok(())
    }

    pub fn systemd_path_item_count(&self) -> usize {
        self.systemd_path_items.len()
    }

    pub fn get_systemd_path_item(&self, id: &str) -> Option<&SystemdPathItem> {
        self.systemd_path_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SystemdPathError> {
        if event.subject.trim().is_empty() {
            return Err(SystemdPathError::EmptyEventSubject);
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

    fn sample_systemd_path_item(id: &str, name: &str) -> SystemdPathItem {
        SystemdPathItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SystemdPath::new("SystemdPath");
        assert_eq!(m.name, "SystemdPath");
        assert_eq!(m.systemd_path_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_systemd_path_item() {
        let mut m = SystemdPath::new("SystemdPath");
        m.add_systemd_path_item(sample_systemd_path_item("a1", "Alpha")).unwrap();
        assert_eq!(m.systemd_path_item_count(), 1);
        assert!(m.get_systemd_path_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SystemdPath::new("SystemdPath");
        assert!(m.add_systemd_path_item(sample_systemd_path_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SystemdPath::new("SystemdPath");
        m.add_systemd_path_item(sample_systemd_path_item("x", "One")).unwrap();
        assert!(m.add_systemd_path_item(sample_systemd_path_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SystemdPath::new("SystemdPath");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}

//! Web core library

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageData {
    pub title: String,
    pub description: String,
    pub path: String,
}

impl PageData {
    pub fn new(title: &str, description: &str, path: &str) -> Self {
        Self {
            title: title.to_string(),
            description: description.to_string(),
            path: path.to_string(),
        }
    }

    pub fn home() -> Self {
        Self::new("Home", "Welcome to the web app", "/")
    }

    pub fn about() -> Self {
        Self::new("About", "Learn more about us", "/about")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavItem {
    pub label: String,
    pub href: String,
    pub active: bool,
}

impl NavItem {
    pub fn new(label: &str, href: &str) -> Self {
        Self {
            label: label.to_string(),
            href: href.to_string(),
            active: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_data() {
        let page = PageData::home();
        assert_eq!(page.title, "Home");
        assert_eq!(page.path, "/");
    }

    #[test]
    fn test_nav_item() {
        let item = NavItem::new("Home", "/");
        assert_eq!(item.label, "Home");
        assert!(!item.active);
    }
}

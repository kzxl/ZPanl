//! # Embedded Web UI Assets
//!
//! Embedded professional single-page application and static assets
//! for the ZPanl sovereign web control panel.

/// Embedded HTML shell for the ZPanl dashboard.
pub const INDEX_HTML: &str = include_str!("../assets/index.html");

/// Embedded CSS stylesheet for styling and dark/light themes.
pub const STYLE_CSS: &str = include_str!("../assets/style.css");

/// Embedded JavaScript client-side application logic and I18N.
pub const APP_JS: &str = include_str!("../assets/app.js");

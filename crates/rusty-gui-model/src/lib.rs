//! Portable GUI descriptor contracts for Rusty Morphospace.

use serde::{Deserialize, Serialize};

/// A renderer-neutral panel descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PanelDescriptor {
    /// Schema id for the descriptor.
    pub schema: String,
    /// Stable panel id.
    pub panel_id: String,
    /// Human-facing panel title.
    pub title: String,
    /// Layout hint for the panel.
    pub layout: LayoutDescriptor,
    /// Widgets contained by the panel.
    pub widgets: Vec<WidgetDescriptor>,
}

/// Toolkit-neutral layout hints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayoutDescriptor {
    /// Layout kind, such as stack or grid.
    pub kind: String,
    /// Gap between child widgets in logical pixels.
    pub gap: u32,
}

/// A GUI widget descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WidgetDescriptor {
    /// Stable widget id.
    pub widget_id: String,
    /// Widget kind, such as toggle, slider, select, button, or graph_canvas.
    pub kind: String,
    /// Human-facing label.
    pub label: String,
    /// Command binding for state-changing widgets.
    pub binding: CommandBinding,
}

/// Binding from a GUI gesture to an external command authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandBinding {
    /// External command id. GUI does not own command authority.
    pub command_id: String,
    /// Canonical setting id or subject id affected by the command.
    pub setting_id: String,
}

impl PanelDescriptor {
    /// Validate the basic descriptor invariants that adapters depend on.
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != "rusty.gui.panel_descriptor.v1" {
            return Err(format!("unsupported panel schema {}", self.schema));
        }
        if self.panel_id.trim().is_empty() {
            return Err("panel_id must not be empty".to_string());
        }
        if self.widgets.is_empty() {
            return Err("panel must contain at least one widget".to_string());
        }
        for widget in &self.widgets {
            if widget.widget_id.trim().is_empty() {
                return Err("widget_id must not be empty".to_string());
            }
            if widget.binding.command_id.trim().is_empty() {
                return Err(format!(
                    "widget {} has no command binding",
                    widget.widget_id
                ));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::PanelDescriptor;

    #[test]
    fn fixture_panel_descriptor_validates() {
        let fixture = include_str!("../../../fixtures/descriptors/basic-panel.json");
        let panel: PanelDescriptor = serde_json::from_str(fixture).expect("valid JSON");
        panel.validate().expect("valid panel descriptor");
    }
}

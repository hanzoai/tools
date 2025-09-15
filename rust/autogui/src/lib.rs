use anyhow::Result;
use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD, Engine};
use enigo::{Enigo, Key, KeyboardControllable, MouseButton, MouseControllable};
use image::ImageOutputFormat;
use log::{debug, info};
use screenshots::Screen;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::Cursor;

/// Computer control tool for MCP
pub struct ComputerControlTool {
    enigo: Enigo,
}

impl ComputerControlTool {
    pub fn new() -> Self {
        Self {
            enigo: Enigo::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
enum ComputerAction {
    #[serde(rename = "screenshot")]
    Screenshot,
    
    #[serde(rename = "mouse_move")]
    MouseMove { x: i32, y: i32 },
    
    #[serde(rename = "mouse_click")]
    MouseClick { 
        button: String,
        #[serde(default)]
        double: bool,
    },
    
    #[serde(rename = "key_press")]
    KeyPress { key: String },
    
    #[serde(rename = "type_text")]
    TypeText { text: String },
    
    #[serde(rename = "scroll")]
    Scroll { 
        direction: String,
        amount: i32,
    },
}

#[derive(Debug, Serialize)]
pub struct ToolResult {
    pub success: bool,
    pub content: Value,
    pub error: Option<String>,
}

#[async_trait]
impl MCPTool for ComputerControlTool {
    fn name(&self) -> &str {
        "computer_control"
    }
    
    fn description(&self) -> &str {
        "Control computer with screenshot, mouse, and keyboard actions"
    }
    
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["screenshot", "mouse_move", "mouse_click", "key_press", "type_text", "scroll"],
                    "description": "The action to perform"
                },
                "x": {
                    "type": "integer",
                    "description": "X coordinate for mouse operations"
                },
                "y": {
                    "type": "integer",
                    "description": "Y coordinate for mouse operations"
                },
                "button": {
                    "type": "string",
                    "enum": ["left", "right", "middle"],
                    "description": "Mouse button to click"
                },
                "double": {
                    "type": "boolean",
                    "description": "Whether to double-click"
                },
                "key": {
                    "type": "string",
                    "description": "Key to press (e.g., 'enter', 'escape', 'tab')"
                },
                "text": {
                    "type": "string",
                    "description": "Text to type"
                },
                "direction": {
                    "type": "string",
                    "enum": ["up", "down"],
                    "description": "Scroll direction"
                },
                "amount": {
                    "type": "integer",
                    "description": "Scroll amount"
                }
            },
            "required": ["action"]
        })
    }
    
    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let action: ComputerAction = serde_json::from_value(params)?;
        
        match action {
            ComputerAction::Screenshot => {
                info!("Taking screenshot");
                let screens = Screen::all()?;
                
                if screens.is_empty() {
                    return Ok(ToolResult {
                        success: false,
                        content: json!({"error": "No screens found"}),
                        error: Some("No screens found".to_string()),
                    });
                }
                
                let screen = &screens[0];
                let image = screen.capture()?;
                
                // Convert to PNG and base64
                let mut buffer = Cursor::new(Vec::new());
                image.write_to(&mut buffer, ImageOutputFormat::Png)?;
                let base64_image = STANDARD.encode(buffer.into_inner());
                
                Ok(ToolResult {
                    success: true,
                    content: json!({
                        "screenshot": format!("data:image/png;base64,{}", base64_image),
                        "width": image.width(),
                        "height": image.height(),
                    }),
                    error: None,
                })
            },
            
            ComputerAction::MouseMove { x, y } => {
                info!("Moving mouse to ({}, {})", x, y);
                self.enigo.mouse_move_to(x, y);
                
                Ok(ToolResult {
                    success: true,
                    content: json!({"moved_to": {"x": x, "y": y}}),
                    error: None,
                })
            },
            
            ComputerAction::MouseClick { button, double } => {
                let mouse_button = match button.as_str() {
                    "left" => MouseButton::Left,
                    "right" => MouseButton::Right,
                    "middle" => MouseButton::Middle,
                    _ => MouseButton::Left,
                };
                
                info!("Clicking mouse button: {:?}, double: {}", button, double);
                
                if double {
                    self.enigo.mouse_click(mouse_button);
                    std::thread::sleep(std::time::Duration::from_millis(50));
                    self.enigo.mouse_click(mouse_button);
                } else {
                    self.enigo.mouse_click(mouse_button);
                }
                
                Ok(ToolResult {
                    success: true,
                    content: json!({"clicked": button, "double": double}),
                    error: None,
                })
            },
            
            ComputerAction::KeyPress { key } => {
                info!("Pressing key: {}", key);
                
                let enigo_key = match key.to_lowercase().as_str() {
                    "enter" | "return" => Key::Return,
                    "tab" => Key::Tab,
                    "escape" | "esc" => Key::Escape,
                    "space" => Key::Space,
                    "backspace" => Key::Backspace,
                    "delete" => Key::Delete,
                    "up" => Key::UpArrow,
                    "down" => Key::DownArrow,
                    "left" => Key::LeftArrow,
                    "right" => Key::RightArrow,
                    "home" => Key::Home,
                    "end" => Key::End,
                    "pageup" => Key::PageUp,
                    "pagedown" => Key::PageDown,
                    _ => {
                        return Ok(ToolResult {
                            success: false,
                            content: json!({"error": format!("Unknown key: {}", key)}),
                            error: Some(format!("Unknown key: {}", key)),
                        });
                    }
                };
                
                self.enigo.key_click(enigo_key);
                
                Ok(ToolResult {
                    success: true,
                    content: json!({"key_pressed": key}),
                    error: None,
                })
            },
            
            ComputerAction::TypeText { text } => {
                info!("Typing text: {}", text);
                self.enigo.key_sequence(&text);
                
                Ok(ToolResult {
                    success: true,
                    content: json!({"typed": text}),
                    error: None,
                })
            },
            
            ComputerAction::Scroll { direction, amount } => {
                info!("Scrolling {} by {}", direction, amount);
                
                let scroll_amount = if direction == "up" {
                    amount
                } else {
                    -amount
                };
                
                self.enigo.mouse_scroll_y(scroll_amount);
                
                Ok(ToolResult {
                    success: true,
                    content: json!({"scrolled": {"direction": direction, "amount": amount}}),
                    error: None,
                })
            },
        }
    }
}

// MCP Tool trait (will be imported from the MCP crate in practice)
#[async_trait]
pub trait MCPTool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters(&self) -> Value;
    async fn execute(&self, params: Value) -> Result<ToolResult>;
}
mod claude_code;
mod codex;
mod hermes;
mod openclaw;
mod util;

pub use claude_code::ClaudeCodeAdapter;
pub use codex::CodexAdapter;
pub use hermes::{HermesAdapter, HermesSettings};
pub use openclaw::OpenClawAdapter;

use crate::adapter::RuntimeAdapter;

pub fn all_adapters() -> Vec<Box<dyn RuntimeAdapter>> {
    vec![
        Box::new(OpenClawAdapter),
        Box::new(HermesAdapter),
        Box::new(ClaudeCodeAdapter),
        Box::new(CodexAdapter),
    ]
}

pub fn adapter_by_id(id: &str) -> Option<Box<dyn RuntimeAdapter>> {
    all_adapters()
        .into_iter()
        .find(|adapter| adapter.id() == id)
}

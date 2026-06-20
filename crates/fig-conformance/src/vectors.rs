use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConformanceSuite {
    pub version: u32,
    pub suite: String,
    pub vectors: Vec<ConformanceVector>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConformanceVector {
    pub id: String,
    pub category: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub message_type: String,
    #[serde(default)]
    pub expected_hex: String,
    #[serde(default)]
    pub frame: Option<FrameSpec>,
    #[serde(default)]
    pub payload: Option<serde_json::Value>,
    #[serde(default)]
    pub channel: Option<ChannelSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameSpec {
    pub frame_type: String,
    pub channel_id: u16,
    pub stream_seq: u32,
    pub schema_id: u8,
    #[serde(default)]
    pub extensions: Vec<ExtensionSpec>,
    #[serde(default)]
    pub payload_hex: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionSpec {
    pub tag: String,
    pub value: ExtensionValueSpec,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExtensionValueSpec {
    Text(String),
    U16(u16),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelSpec {
    pub channel_id: u16,
    pub client_stream_id: u64,
    pub server_stream_id: u64,
}

pub fn load_suite(path: &Path) -> Result<ConformanceSuite> {
    let data = fs::read_to_string(path)
        .with_context(|| format!("read {}", path.display()))?;
    serde_json::from_str(&data).context("parse conformance JSON")
}

pub fn write_suite(path: &Path, suite: &ConformanceSuite) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(suite)?;
    fs::write(path, json)?;
    Ok(())
}

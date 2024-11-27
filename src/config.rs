use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use tokio::{fs, io};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct ManagerConfig {
    // in bytes
    pub max_size: usize,
    pub listen: SocketAddr,
    // corresponds to hyper::Uri
    pub storage_nodes: Vec<String>,
    pub deduplicate: bool,
    pub rebalance: bool,
    pub rebalance_threshold: f32,
    pub persistence: bool,
    pub log_file: PathBuf,
}

impl Default for ManagerConfig {
    fn default() -> Self {
        Self {
            max_size: 5 * 1000 * 1000 * 1000,
            listen: ([0, 0, 0, 0], 3000).into(),
            storage_nodes: vec!["http://0.0.0.0:3001".to_string()],
            deduplicate: true,
            rebalance: false,
            rebalance_threshold: 0.0,
            persistence: false,
            log_file: "manager.log".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct StorageConfig {
    allocated_size: usize,
    listen: SocketAddr,
    // corresponds to hyper::Uri
    manager_nodes: Vec<String>,
    region: String,
    file_dir: PathBuf,
    log_file: PathBuf,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            allocated_size: 15 * 1000 * 1000 * 1000,
            listen: ([0, 0, 0, 0], 3001).into(),
            manager_nodes: vec!["http://0.0.0.0:3000".to_string()],
            region: "mars".to_string(),
            file_dir: "files".into(),
            log_file: "storage.log".into(),
        }
    }
}

impl ManagerConfig {
    pub async fn load<P: AsRef<Path>>(file: P) -> Result<Self, io::Error> {
        let config = match fs::read_to_string(&file).await {
            Ok(contents) => toml::from_str(&contents).map_err(io::Error::other)?,
            Err(err) => match err.kind() {
                io::ErrorKind::NotFound => {
                    let default_config = Self::default();
                    fs::write(
                        &file,
                        toml::to_string_pretty(&default_config).map_err(io::Error::other)?,
                    )
                    .await?;
                    default_config
                }
                _ => return Err(err),
            },
        };
        Ok(config)
    }
}
impl StorageConfig {
    pub async fn load<P: AsRef<Path>>(file: P) -> Result<Self, io::Error> {
        let config = match fs::read_to_string(&file).await {
            Ok(contents) => toml::from_str(&contents).map_err(io::Error::other)?,
            Err(err) => match err.kind() {
                io::ErrorKind::NotFound => {
                    let default_config = Self::default();
                    fs::write(
                        &file,
                        toml::to_string_pretty(&default_config).map_err(io::Error::other)?,
                    )
                    .await?;
                    default_config
                }
                _ => return Err(err),
            },
        };
        Ok(config)
    }
}

use serde::{Deserialize, Serialize};
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LocalSettings {
    #[serde(default)]
    pub recent_projects: Vec<RecentProjectRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentProjectRecord {
    pub path: String,
    pub display_name: String,
    pub last_opened_at: String,
}

pub struct LocalSettingsStorage {
    settings_path: PathBuf,
}

impl LocalSettingsStorage {
    pub fn new(app_data_dir: &Path) -> Self {
        Self {
            settings_path: app_data_dir.join("hotsas_settings.json"),
        }
    }

    pub fn load(&self) -> Result<LocalSettings, io::Error> {
        if !self.settings_path.exists() {
            return Ok(LocalSettings::default());
        }
        let content = std::fs::read_to_string(&self.settings_path)?;
        let settings: LocalSettings = serde_json::from_str(&content).unwrap_or_default();
        Ok(settings)
    }

    pub fn save(&self, settings: &LocalSettings) -> Result<(), io::Error> {
        if let Some(parent) = self.settings_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(settings)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        std::fs::write(&self.settings_path, content)
    }

    pub fn add_recent_project(&self, path: &str, display_name: &str) -> Result<(), io::Error> {
        let mut settings = self.load()?;
        let now = chrono::Local::now().to_rfc3339();
        settings.recent_projects.retain(|r| r.path != path);
        settings.recent_projects.insert(
            0,
            RecentProjectRecord {
                path: path.to_string(),
                display_name: display_name.to_string(),
                last_opened_at: now,
            },
        );
        const MAX_RECENT: usize = 20;
        if settings.recent_projects.len() > MAX_RECENT {
            settings.recent_projects.truncate(MAX_RECENT);
        }
        self.save(&settings)
    }

    pub fn remove_recent_project(&self, path: &str) -> Result<(), io::Error> {
        let mut settings = self.load()?;
        settings.recent_projects.retain(|r| r.path != path);
        self.save(&settings)
    }

    pub fn clear_missing_recent_projects(&self) -> Result<usize, io::Error> {
        let mut settings = self.load()?;
        let before = settings.recent_projects.len();
        settings
            .recent_projects
            .retain(|r| Path::new(&r.path).exists());
        let after = settings.recent_projects.len();
        self.save(&settings)?;
        Ok(before - after)
    }
}

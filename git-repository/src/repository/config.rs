use crate::config;

impl crate::Repository {
    /// Return a snapshot of the configuration as seen upon opening the repository.
    pub fn config_snapshot(&self) -> config::Snapshot<'_> {
        config::Snapshot { repo: self }
    }
}

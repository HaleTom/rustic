//! `config` example
use rustic_core::{ConfigOpts, Repository, RepositoryOptions};
use simplelog::{Config, LevelFilter, SimpleLogger};

fn main() {
    // Display info logs
    let _ = SimpleLogger::init(LevelFilter::Info, Config::default());

    // Open repository
    let repo_opts = RepositoryOptions {
        repository: Some("/tmp/repo".to_string()),
        password: Some("test".to_string()),
        ..Default::default()
    };

    let repo = Repository::new(&repo_opts).unwrap().open().unwrap();

    // Set Config, e.g. Compression level
    let config_opts = ConfigOpts {
        set_compression: Some(22),
        ..Default::default()
    };
    repo.apply_config(&config_opts).unwrap();
}

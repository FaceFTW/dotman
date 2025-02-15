use anyhow::{Context, Result};
use watchexec::sources::fs::Watcher;
use watchexec::{Config, Watchexec};
use watchexec_filterer_globset::GlobsetFilterer;

use super::display_error;
use crate::args::Options;
use crate::deploy;

pub(crate) async fn watch(opt: Options) -> Result<()> {
    let config = Config::default();

    config.file_watcher(Watcher::Native);
    config.pathset(["."]);

    let filter = GlobsetFilterer::new(
        ".",
        [],
        [
            (format!("{}/**", opt.cache_directory.display()), None),
            (opt.cache_file.to_string_lossy().into(), None),
            (".git/**".into(), None),
            ("DOTTER_SYMLINK_TEST".into(), None),
        ],
        [],
        [],
        [],
    )
    .await?;
    config.filterer(filter);

    config.on_action(move |mut action| {
        if action.signals().next().is_some() {
            action.quit();
            return action;
        }

        println!("[Dotter] Deploying...");
        if let Err(e) = deploy::deploy(&opt) {
            display_error(e);
        }

        action
    });

    config.on_error(move |e| {
        log::error!("Watcher error: {e:#?}");
    });

    let we = Watchexec::with_config(config)?;
    we.main().await.context("run watchexec main loop")??;
    Ok(())
}

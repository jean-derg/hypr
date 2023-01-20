use std::{fs, path::Path};
use notify::{Watcher, RecommendedWatcher, Config};

use notify::{Event, EventKind, RecursiveMode};

pub fn protect(path: &'static Path) {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())
    .unwrap();
    watcher.watch(path, RecursiveMode::Recursive).unwrap();
    for e in rx {
        match e {
            Ok(e) => check_event(e, path),
            Err(e) => eprintln!("error: {e}")
        }
    }
}

fn check_event(ev: Event, path: &Path) {
    println!("ev {ev:?}");
    match ev.kind {
        EventKind::Modify(_) | EventKind::Remove(_) | EventKind::Create(_) => {
            println!("modification detected");
            fs::remove_dir_all(path).unwrap();
        }
        _ => {}
    }
}

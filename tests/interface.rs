use anyhow::Result;
use is_data_interface::{error::VecResult, fs::FileContentInterface, interface};
use std::fs;
use std::path::PathBuf;
use std::collections::HashSet;
// #[test]
fn test_interface() {
    const WRITE_TEXT_1: &str = "test_1";
    const WRITE_TEXT_2: &str = "test_2";
    let write_text_path_1: String = format!("tests/{WRITE_TEXT_1}.txt");
    let write_text_path_2: String = format!("tests/{WRITE_TEXT_2}.txt");

    let _ = fs::write(&write_text_path_1, "1");
    let _ = fs::write(&write_text_path_2, "2");

    let _ = || -> Result<()> {
        let paths = vec![
            (&write_text_path_1, notify::RecursiveMode::NonRecursive),
            (&write_text_path_2, notify::RecursiveMode::NonRecursive),
        ];
        // let VecResult { ok, err } = FileContentInterface::new(&paths)?;
        // if !err.is_empty() {
        //     panic!("err vec not empty")
        
        let set = HashSet::from_iter(paths.clone().into_iter().map(|x|(PathBuf::from(x.0) )));
        let mut fi = FileContentInterface::new(set);
        drop(paths);
        // let rx = fi.get().share_reciver();
        let write_text_path_1_c_1 = write_text_path_1.clone();
        let write_text_path_1_c_2 = write_text_path_1_c_1.clone();
        // std::thread::scope(|_| {
        //     std::thread::spawn(move || {
        //         let _ = fs::read(&write_text_path_1_c_1);
        //     });
        //     if let Ok(read) = rx.recv_timeout(std::time::Duration::from_secs(1)) {
        //         assert_eq!(
        //             read.unwrap(),
        //             notify::Event {
        //                 kind: notify::EventKind::Access(notify::event::AccessKind::Open(
        //                     notify::event::AccessMode::Any
        //                 )),
        //                 paths: vec!(fs::canonicalize(write_text_path_1_c_2).unwrap()),
        //                 attrs: notify::event::EventAttributes::new(),
        //             }
        //         );
        //     } else {
        //         panic!("no event recived");
        //     }
        // });
        // let di = DataInterface::new(fi);
        use crate::interface::{InterfaceRead, InterfaceReadManager};
        let data = fi.read_all()?;
        if !data.err.is_empty() {
            panic!("err vec not empty");
        }
        use std::collections::HashMap;
        let set: HashMap<PathBuf, Vec<u8>> = HashMap::from_iter(data.ok);
        assert_eq!(
            set.get(std::path::Path::new(&write_text_path_1)),
            Some(&Vec::from("1"))
        );
        // let lock = fi.lock();

        Ok(())
    }();
    let _ = fs::remove_file(&write_text_path_1);
    let _ = fs::remove_file(&write_text_path_2);
}
// #[test]
// fn test_lock() {
//     const WRITE_TEXT: &str = "test_watch_file";
//     let write_text_path: String = format!("tests/{WRITE_TEXT}.txt");
//     let _ = fs::write(&write_text_path, WRITE_TEXT);
//     let paths = vec![(&write_text_path, notify::RecursiveMode::NonRecursive)];
//
//     let _ = || -> anyhow::Result<()> {
//         let fsi = FsInterface::new(&paths)?;
//         drop(paths);
//         let rx = fsi.share_reciver();
//         let fi = FileInterface::new(fsi);
//         use notify::Watcher;
//         // fi.0.watcher.watcher.watch();
//         // let di = DataInterface::new(fi);
//         use interface::Interface;
//         // assert_eq!(str::from_utf8(&fi.read()?)?, WRITE_TEXT);
//         Ok(())
//     }();
//     let _ = fs::remove_file(&write_text_path);
// }

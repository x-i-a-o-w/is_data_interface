// #[test]
// fn test_watch() {
//     use is_data_interface::{FsInterface, interface};
//     use std::fs;
// 
//     pub struct FileInterface<F = FsInterface>(F);
// 
//     impl interface::Interface for FileInterface {
//         type Output = Vec<u8>;
//         fn read(&self) -> anyhow::Result<Self::Output> {
//             Ok(fs::read(&self.0.path)?)
//         }
//     }
//     const WRITE_TEXT: &str = "test_watch_file";
//     let write_text_path: String = format!("tests/{WRITE_TEXT}.txt");
//     use anyhow::Result;
//     let _ = fs::write(&write_text_path, WRITE_TEXT);
//     let _ = || -> Result<()> {
//         let fi = FileInterface(FsInterface::new(&write_text_path, false)?);
//         use notify::Watcher;
//         fi.0.watcher.unwrap().watcher.watch();
//         // let di = DataInterface::new(fi);
//         use interface::Interface;
//         assert_eq!(str::from_utf8(&fi.read()?)?, WRITE_TEXT);
//         Ok(())
//     }();
//     let _ = fs::remove_file(&write_text_path);
// }

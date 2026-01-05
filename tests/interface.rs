use anyhow::Result;
use is_data_interface::{fs::FsInterface, interface};
use std::fs;
use std::path::PathBuf;
#[test]
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
        let fsi = FsInterface::new(&paths)?;
        drop(paths);
        let rx = fsi.share_reciver();
        let fi = FileInterface(fsi);
        let write_text_path_1_c_1 = write_text_path_1.clone();
        let write_text_path_1_c_2 = write_text_path_1_c_1.clone();
        std::thread::scope(|s| {
            std::thread::spawn(move || {
                let _ = fs::read(&write_text_path_1_c_1);
            });
            if let Ok(read) = rx.recv_timeout(std::time::Duration::from_secs(1)) {
                assert_eq!(
                    read.unwrap(),
                    notify::Event {
                        kind: notify::EventKind::Access(notify::event::AccessKind::Open(
                            notify::event::AccessMode::Any
                        )),
                        paths: vec!(fs::canonicalize(write_text_path_1_c_2).unwrap()),
                        attrs: notify::event::EventAttributes::new(),
                    }
                );
            } else {
                panic!("no event recived");
            }
        });
        // let di = DataInterface::new(fi);
        use interface::Interface;
        let data = fi.read()?;
        assert_eq!(data[0].0, PathBuf::from(&write_text_path_1));
        assert_eq!(data[0].1.as_ref().unwrap(), &Vec::from("1"));
        Ok(())
    }();
    let _ = fs::remove_file(&write_text_path_1);
    let _ = fs::remove_file(&write_text_path_2);
}
#[test]
fn test_watch() {
    const WRITE_TEXT: &str = "test_watch_file";
    let write_text_path: String = format!("tests/{WRITE_TEXT}.txt");
    let _ = fs::write(&write_text_path, WRITE_TEXT);
    let paths = vec![(&write_text_path, notify::RecursiveMode::NonRecursive)];

    let _ = || -> anyhow::Result<()> {
        let fsi = FsInterface::new(&paths)?;
        drop(paths);
        let rx = fsi.share_reciver();
        let fi = FileInterface(fsi);
        use notify::Watcher;
        // fi.0.watcher.watcher.watch();
        // let di = DataInterface::new(fi);
        use interface::Interface;
        // assert_eq!(str::from_utf8(&fi.read()?)?, WRITE_TEXT);
        Ok(())
    }();
    let _ = fs::remove_file(&write_text_path);
}
pub struct FileInterface<F = FsInterface>(F);
use interface::Interface;
impl<'a> interface::Interface for FileInterface {
    // TODO this Output type should be flattened somehow to reduce Vec and Result enum based overhead
    type Output = Vec<(PathBuf, std::io::Result<Vec<u8>>)>;
    fn read(&self) -> anyhow::Result<Self::Output> {
        Ok(self
            .0
            .paths()
            .iter()
            .map(|x| {
                let content = fs::read(&x);
                (x.clone(), content)
            })
            .collect())
    }
}
impl<F> FileInterface<F>
where
    FileInterface<F>: interface::Interface,
{
    //      pub fn read_ignore(&self) -> anyhow::Result<<FileInterface<F> as Interface>::Output> {
    // let locked=self.0.paths().iter().map(|x|{
    //
    // });
    //      	fs::rename()?;
    //          interface::Interface::read(self)
    //      }
}

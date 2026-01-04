#[test]
fn test_interface() {
    use is_data_interface::{FsInterface, interface};
    use std::path::PathBuf;
    use std::fs;

    pub struct FileInterface<F = FsInterface>(F);

    impl<'a> interface::Interface for FileInterface {
        // type Output = Vec<(std::path::PathBuf,Vec<u8>)>;
        type Output = Vec<(PathBuf,std::io::Result<Vec<u8>>)>;
        fn read(&self) -> anyhow::Result<Self::Output> {
			// use notify::Watcher;
			Ok(
				self.0.paths.iter().map(|x|
					{
						let content=fs::read(&x);
						(x.clone(),content)
					}
				).collect()
			)
			// Ok(())
        }
    }
    const WRITE_TEXT_1: &str = "test_1";
    const WRITE_TEXT_2: &str = "test_2";
    let write_text_path_1: String = format!("tests/{WRITE_TEXT_1}.txt");
    let write_text_path_2: String = format!("tests/{WRITE_TEXT_2}.txt");

    let _=fs::write(&write_text_path_1,"1");
    let _=fs::write(&write_text_path_2,"2");

    use anyhow::Result;
    let _ = || -> Result<()> {
    	let paths=vec!(
    			(&write_text_path_1,notify::RecursiveMode::NonRecursive),
    			(&write_text_path_2,notify::RecursiveMode::NonRecursive)
    		);
        let fi = FileInterface(FsInterface::new(&paths)?);
        // let di = DataInterface::new(fi);
        use interface::Interface;
        let data=fi.read()?;
        assert_eq!(data[0].0,PathBuf::from(&write_text_path_1));
        assert_eq!(data[0].1.as_ref().unwrap(),&Vec::from("1"));
        // assert_eq!(str::from_utf8(&fi.read()?)?, "1");
        Ok(())
    }();
    let _ = fs::remove_file(&write_text_path_1);
    let _ = fs::remove_file(&write_text_path_2);
}

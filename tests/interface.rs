#[test]
fn test_interface() {
    use is_data_interface::{DataInterface, FsInterface, interface};
    use std::fs;
    
    pub struct FileInterface<F = FsInterface>(F);

    impl interface::Interface for FileInterface {
        type Output = Vec<u8>;
        fn read(&self) -> anyhow::Result<Self::Output> {
            Ok(fs::read(&self.0.path)?)
        }
    }
    const WRITE_TEXT:&str="test_interface_file";
    use anyhow::Result;
    let _ = fs::write("tests/test_interface_file.txt",WRITE_TEXT);
    let _=||->Result<()>{	
	    let fi = FileInterface(FsInterface::new("tests/test_interface_file.txt",false,)?);
	    // let di = DataInterface::new(fi);
	    use interface::Interface;
	    assert_eq!(str::from_utf8(&fi.read()?)?,WRITE_TEXT);
	    Ok(())
    }();
    let _ = fs::remove_file("tests/test_interface_file.txt");
}

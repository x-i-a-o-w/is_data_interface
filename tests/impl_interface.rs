use is_data_interface::{
    error::{self, VecResult},
    interface,
};
#[test]
fn impl_interface() {
    // all examples of imlementation shown here

    pub struct TestBorrowInputBorrowOutput<S: AsRef<str> = String>(Vec<S>);
    impl<'input, S: AsRef<str> + 'input> interface::InterfaceRead<'input>
        for TestBorrowInputBorrowOutput<S>
    {
        type Input = &'input S;
        type Output = &'input str;
        type OutputError = ();
        fn read(input: Self::Input) -> std::result::Result<Self::Output, Self::OutputError> {
            Ok(input.as_ref())
        }
    }

    impl<'input, S: AsRef<str> + 'input> interface::InterfaceReadManager<'input>
        for TestBorrowInputBorrowOutput<S>
    {
        type OutputCollection<O: 'input> = Vec<O>;
        fn read_all(&'input self) -> anyhow::Result<Self::OutputCollection<Self::Output>> {
            use interface::InterfaceRead;
            Ok(self.0.iter().map(|x| Self::read(x).unwrap()).collect())
        }
    }
    // error
    // let _=TestBorrowInputBorrowOutput(vec!(1));
    let _ = TestBorrowInputBorrowOutput(vec!["d"]);
    let _ = TestBorrowInputBorrowOutput(vec!["d".to_owned()]);

    pub struct TestOwnedInputOwnedOutput<S: Clone = String>(Vec<S>);
    impl<'input, S: Clone + 'input> interface::InterfaceRead<'input> for TestOwnedInputOwnedOutput<S> {
        type Input = S;
        type Output = S;
        type OutputError = ();
        fn read(input: Self::Input) -> std::result::Result<Self::Output, Self::OutputError> {
            Ok(input)
        }
    }

    impl<'input, S: Clone + 'input> interface::InterfaceReadManager<'input>
        for TestOwnedInputOwnedOutput<S>
    {
        type OutputCollection<O: 'input> = Vec<S>;
        fn read_all(&'input self) -> anyhow::Result<Self::OutputCollection<Self::Output>> {
            use interface::InterfaceRead;
            Ok(self
                .0
                .iter()
                .map(|x| Self::read(x.clone()).unwrap())
                .collect())
        }
    }

    let _ = TestOwnedInputOwnedOutput(vec![1]);
    let _ = TestOwnedInputOwnedOutput(vec!["d"]);
    let _ = TestOwnedInputOwnedOutput(vec!["d".to_owned()]);

    pub struct TestOwnedInputBorrowOutput<S: Clone = String>(Vec<S>);
    impl<'input, S: Clone + 'input> interface::InterfaceRead<'input> for TestOwnedInputBorrowOutput<S> {
        // borrowed in input but will be borrowed from owned in manager
        type Input = &'input S;
        type Output = &'input S;
        type OutputError = ();
        fn read(input: Self::Input) -> std::result::Result<Self::Output, Self::OutputError> {
            Ok(&input)
        }
    }

    impl<'input, S: Clone + 'input> interface::InterfaceReadManager<'input>
        for TestOwnedInputBorrowOutput<S>
    {
        type OutputCollection<O: 'input> = Vec<&'input S>;
        fn read_all(&'input self) -> anyhow::Result<Self::OutputCollection<Self::Output>> {
            use interface::InterfaceRead;
            Ok(self.0.iter().map(|x| Self::read(&x).unwrap()).collect())
        }
    }

    let _ = TestOwnedInputBorrowOutput(vec![1]);
    let _ = TestOwnedInputBorrowOutput(vec!["d"]);
    let _ = TestOwnedInputBorrowOutput(vec!["d".to_owned()]);

    pub struct TestBorrowInputOwnedOutput<S: Clone = String>(Vec<S>);
    impl<'input, S: Clone + 'input> interface::InterfaceRead<'input> for TestBorrowInputOwnedOutput<S> {
        // borrowed in input but will be borrowed from owned in manager
        type Input = &'input S;
        type Output = S;
        type OutputError = ();
        fn read(input: Self::Input) -> std::result::Result<Self::Output, Self::OutputError> {
            Ok(input.clone())
        }
    }

    impl<'input, S: Clone + 'input> interface::InterfaceReadManager<'input>
        for TestBorrowInputOwnedOutput<S>
    {
        type OutputCollection<O: 'input> = Vec<S>;
        fn read_all(&'input self) -> anyhow::Result<Self::OutputCollection<Self::Output>> {
            use interface::InterfaceRead;
            Ok(self.0.iter().map(|x| Self::read(x).unwrap()).collect())
        }
    }

    let _ = TestBorrowInputOwnedOutput(vec![1]);
    let _ = TestBorrowInputOwnedOutput(vec!["d"]);
    let _ = TestBorrowInputOwnedOutput(vec!["d".to_owned()]);
}

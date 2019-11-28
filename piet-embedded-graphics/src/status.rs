/* ////
    use piet::{
        ////new_error, 
        Error, 
        ////ErrorKind, 
    };

    #[derive(Debug)]
    struct WrappedStatus(Status);

    impl fmt::Display for WrappedStatus {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Embed error: {:?}", self.0)
        }
    }

    impl std::error::Error for WrappedStatus {}

    trait WrapError<T> {
        fn wrap(self) -> Result<T, Error>;
    }

    // Discussion question: a blanket impl here should be pretty doable.

    impl<T> WrapError<T> for Result<T, BorrowError> {
        fn wrap(self) -> Result<T, Error> {
            self.map_err(|e| {
                let e: Box<dyn std::error::Error> = Box::new(e);
                e.into()
            })
        }
    }

    impl<T> WrapError<T> for Result<T, Status> {
        fn wrap(self) -> Result<T, Error> {
            self.map_err(|e| {
                let e: Box<dyn std::error::Error> = Box::new(WrappedStatus(e));
                e.into()
            })
        }
    }
*/ ////
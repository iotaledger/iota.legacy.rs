	error_chain!{
        types {
            Error, ErrorKind, ResultExt, Result;
        }
		links {
	    }
        foreign_links {
                Fmt(::std::fmt::Error);
                Io(::std::io::Error) #[cfg(unix)];
        }
        errors {
            InvalidInputSize(i: usize) {
                description("Input trinary is not of `TRANSACTION_LENGTH`")
                display("Invalid trinary Size: '{}'", i)
            }
            InvalidMinWeightMagnitude(i: usize) {
                description("Min weight magnitude exceeds `HASH_LENGT`")
                display("Invalid Weight Size: '{}'", i)                
            }
        }
    }

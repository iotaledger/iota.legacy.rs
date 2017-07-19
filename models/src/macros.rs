macro_rules! typed_view {
    ($name: ident, $error: ident, $len_name: ident, $len: expr) => {

        pub const $len_name: usize = $len;

        #[derive(Eq, PartialEq)]
        pub struct $name<'a>(&'a [Trit]);

        #[derive(Debug, Eq, PartialEq)]
        pub enum $error {
            InvalidLength,
        }

        impl<'a> fmt::Debug for $name<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str(stringify!($name))
                    .and_then(|_| f.write_str("("))
                    .and_then(|_| fmt::Display::fmt(self, f))
                    .and_then(|_| f.write_str(")"))
            }
        }


        impl<'a> $name<'a> {
            pub fn from_trits(base: &'a [Trit]) -> Result<Self, $error> {
                if base.len() != $len_name{
                    return Err($error::InvalidLength);
                }
                Ok($name(base))
            }
        }

        impl<'a> Deref for $name<'a> {
            type Target = &'a [Trit];
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<'a> fmt::Display for $name<'a> {

            #[cfg(feature = "alloc")]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let s = trits_to_string(self.0).unwrap();
                f.write_str(&s)
            }

            #[cfg(not(feature = "alloc"))]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                use core::fmt::Debug;

                self.0.fmt(f)
            }
        }
    }
}

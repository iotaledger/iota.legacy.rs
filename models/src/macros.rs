macro_rules! typed_own {
    ($name: ident, $viewname: ident, $len_name: ident) => {
        pub struct $name([Trit; $len_name]);

        impl Default for $name {
            fn default() -> $name{
                $name([0; $len_name])
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &$name) -> bool {
                &self.0[..$len_name] == &other.0[..$len_name]
            }
        }

        impl $name {
            pub fn from_trits(base: &[Trit]) -> Result<Self, ::ModelParseError> {
                if base.len() != $len_name{
                    return Err(::ModelParseError::InvalidLength);
                }

                let mut inst = $name::default();
                inst.0.clone_from_slice(base);
                Ok(inst)
            }

            pub fn from_view(view: &$viewname) -> Self {
                let mut inst = $name::default();
                inst.0.clone_from_slice(view);
                inst
            }

            pub fn view(&self) -> $viewname {
                $viewname(&self.0)
            }
        }

        impl ::core::ops::Deref for $name {
            type Target = [Trit; $len_name];
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl ::core::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                f.write_str(stringify!($name))
                    .and_then(|_| f.write_str("("))
                    .and_then(|_| ::core::fmt::Display::fmt(self, f))
                    .and_then(|_| f.write_str(")"))
            }
        }

        impl ::core::fmt::Display for $name {

            #[cfg(feature = "alloc")]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let s = trits_to_string(&self.0).unwrap();
                f.write_str(&s)
            }

            #[cfg(not(feature = "alloc"))]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                use ::core::fmt::Debug;
                self.0.fmt(f)
            }
        }
    }
}


macro_rules! typed_view {
    ($name: ident, $len_name: ident) => {
        #[derive(Eq, PartialEq)]
        pub struct $name<'a>(&'a [Trit]);

        impl<'a> $name<'a> {
            pub fn from_trits(base: &'a [Trit]) -> Result<Self, ::ModelParseError> {
                if base.len() != $len_name{
                    return Err(::ModelParseError::InvalidLength);
                }
                Ok($name(base))
            }

            #[inline]
            pub unsafe fn from_trits_raw(base: &'a [Trit]) -> Self {
                $name(base)
            }
        }

        impl<'a> ::core::ops::Deref for $name<'a> {
            type Target = [Trit];
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<'a> ::core::fmt::Debug for $name<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                f.write_str(stringify!($name))
                    .and_then(|_| f.write_str("("))
                    .and_then(|_| ::core::fmt::Display::fmt(self, f))
                    .and_then(|_| f.write_str(")"))
            }
        }

        impl<'a> ::core::fmt::Display for $name<'a> {
            #[cfg(feature = "alloc")]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let s = trits_to_string(self.0).unwrap();
                f.write_str(&s)
            }

            #[cfg(not(feature = "alloc"))]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                use ::core::fmt::Debug;
                self.0.fmt(f)
            }
        }
    }
}

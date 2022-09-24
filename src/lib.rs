#[macro_export]
macro_rules! enum_array {
    (
        $(#[$attr:meta])*
        $visibility:vis enum $name:ident {
            $($id:ident),*$(,)?
        }
    ) => {
        $(#[$attr])*
        $visibility enum $name {
            $($id),*
        }
        impl $name {
            $visibility const ENTRIES: [$name; <[$name]>::len(&[$($name::$id),*])] = [$($name::$id),*];
        }
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn empty() {
        enum_array!{
            enum Empty {}
        }
        assert!(Empty::ENTRIES.is_empty())
    }
    #[test]
    fn full() {
        enum_array!{
            #[derive(Debug, PartialEq, Eq)]
            pub enum Example {
                A,
                B,
            }
        }
        assert_eq!(Example::ENTRIES, [Example::A, Example::B])
    }
}
#![macro_use]

/// A macro to generate HTML class names from enums.
macro_rules! prop_enum {
    ($name:ident {$($choice:ident => $string:literal),* $(,)?}) => {
        #[derive(Copy, Clone, PartialEq, Eq)]
        pub enum $name {
            $($choice),*
        }

        impl $name {
            fn class(self, class: &str) -> String {
                format!(
                    "{}--{}",
                    class,
                    match self {
                        $($name::$choice => $string),*
                    }
                )
            }
        }
    };
}

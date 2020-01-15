#![macro_use]

macro_rules! prop_enum {
    ($name:ident {$($choice:ident => $string:literal),* $(,)?}) => {
        #[derive(Copy, Clone)]
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

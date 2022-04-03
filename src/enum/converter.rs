#[macro_export]
macro_rules! enum_convertable {
    ($literal_type:ident $(#[$enum_meta:meta])? $enum_vis:vis enum $enum_name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? {
        DEFAULT => $default_value_literal:literal,
        $($(#[$value_meta:meta])? $value_name:ident => $value_literal:literal,)*
    }) => {
        $(#[$enum_meta])?
        $enum_vis enum $enum_name$(<$($lt$(:$clt$(+$dlt)*)?),+>)? {
            DEFAULT,
            $(#[allow(non_camel_case_types)] $(#[$value_meta])? $value_name,)+
        }
        impl From<$literal_type> for $enum_name$(<$($lt$(:$clt$(+$dlt)*)?),+>)? {
            #[inline]
            fn from(val: $literal_type) -> Self {
                return match val {
                    $($value_literal => $enum_name::$value_name,)*
                    _ => $enum_name::DEFAULT,
                };
            }
        }
        impl Into<$literal_type> for $enum_name$(<$($lt$(:$clt$(+$dlt)*)?),+>)? {
            #[inline]
            fn into(self) -> $literal_type {
                match self {
                    $($enum_name::$value_name => $value_literal,)*
                    $enum_name::DEFAULT => $default_value_literal,
                }
            }
        }
    }
}
/// Creates an enum with various traits.
/// The first key-value pair is the default used if any conversion would fail.
#[macro_export]
macro_rules! extended_enum {
    ( $name:ident, $ty:ty, $( $var:ident => $val:expr ),+ $(,)* ) => (

        #[derive(Clone,Debug,Eq,PartialEq)]
        pub enum $name {
            $($var,)*
        }

        impl From<$name> for $ty {
            fn from(v: $name) -> Self {
                match v {
                    $( $name::$var => $val, )*
                }
            }
        }

        impl PartialEq<$name> for $ty {
            fn eq(&self, other: &$name) -> bool {
                match *other {
                    $( $name::$var => *self == $val, )*
                }
            }
        }

        impl TryFrom<$ty> for $name {
            type Error = crate::error::Error;

            fn try_from(value: $ty) -> Result<Self, Self::Error> {
                match value {
                    $( $val => Ok($name::$var),)*
                    _ => Err(Self::Error::from(crate::error::HciError::new(crate::error::HciErrorKind::InvalidValue))),
                }
            }
        }
    );
}

#[macro_export]
macro_rules! extended_enum_other {
    ( $name:ident, $ty:ty,
      $( $var:ident => $val:expr ),+ $(,)* ) => (

        #[derive(Clone,Debug,Eq,PartialEq)]
        pub enum $name {
            $($var,)*
            Other($ty),
        }

        impl From<$ty> for $name {
            fn from(v: $ty) -> Self {
                match v {
                    $( $val => $name::$var,)*
                    _ => $name::Other(v),
                }
            }
        }

        impl From<$name> for $ty {
            fn from(v: $name) -> Self {
                match v {
                    $( $name::$var => $val, )*
                    $name::Other(v) => v,
                }
            }
        }

        impl PartialEq<$name> for $ty {
            fn eq(&self, other: &$name) -> bool {
                match *other {
                    $( $name::$var => *self == $val, )*
                    $name::Other(v) => *self == v,
                }
            }
        }
    );
}

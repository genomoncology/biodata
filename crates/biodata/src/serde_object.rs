//! Keep record boundaries map-only while retaining derived field checks.
#[cfg(feature = "serde")]
use std::{fmt, marker::PhantomData};

#[cfg(feature = "serde")]
pub(crate) fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: serde::Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    if !deserializer.is_human_readable() {
        return T::deserialize(deserializer);
    }
    struct Object<T>(PhantomData<T>);
    impl<'de, T: serde::Deserialize<'de>> serde::de::Visitor<'de> for Object<T> {
        type Value = T;
        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("an object")
        }
        fn visit_map<M: serde::de::MapAccess<'de>>(self, map: M) -> Result<T, M::Error> {
            T::deserialize(serde::de::value::MapAccessDeserializer::new(map))
        }
    }
    deserializer.deserialize_map(Object(PhantomData))
}

#[cfg(feature = "serde")]
macro_rules! object_deserialize {
    ($value:ty, $fields:ty) => {
        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for $value {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let fields = crate::serde_object::deserialize::<$fields, D>(deserializer)?;
                Self::try_from(fields).map_err(serde::de::Error::custom)
            }
        }
    };
}
#[cfg(feature = "serde")]
pub(crate) use object_deserialize;

// Keep the public enum and its private field helper on one declaration.
macro_rules! object_enum {
    ($helper:ident, $tag:literal, $content:literal, $rename:literal;
     $(#[$meta:meta])* $visibility:vis enum $name:ident {
        $($(#[$variant_meta:meta])* $variant:ident($payload:ty)),* $(,)?
    }) => {
        $(#[$meta])* $visibility enum $name {
            $($(#[$variant_meta])* $variant($payload)),*
        }
        #[cfg(feature = "serde")]
        #[derive(serde::Deserialize)]
        #[serde(tag = $tag, content = $content, rename_all = $rename)]
        enum $helper {
            $($(#[$variant_meta])* $variant($payload)),*
        }
        #[cfg(feature = "serde")]
        impl From<$helper> for $name {
            fn from(fields: $helper) -> Self {
                match fields { $($helper::$variant(value) => Self::$variant(value)),* }
            }
        }
        #[cfg(feature = "serde")]
        crate::serde_object::object_deserialize!($name, $helper);
    };
}
pub(crate) use object_enum;

// Declare fields once for the public value and its derived reader. All serde
// attributes apply to both, and non-human-readable readers retain their layout.
macro_rules! object_record {
    ($(#[$meta:meta])* $visibility:vis struct $name:ident
     $(<$($generic:ident $(= $default:ty)?),*>)? {
        $($(#[$field_meta:meta])* $field_visibility:vis $field:ident: $ty:ty),* $(,)?
    }) => {
        $(#[$meta])* $visibility struct $name $(<$($generic $(= $default)?),*>)? {
            $($(#[$field_meta])* $field_visibility $field: $ty),*
        }
        #[cfg(feature = "serde")]
        impl<'de $(, $($generic: serde::Deserialize<'de>),*)?> serde::Deserialize<'de>
            for $name $(<$($generic),*>)? {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                $(#[$meta])*
                #[derive(serde::Deserialize)]
                struct $name $(<$($generic $(= $default)?),*>)? {
                    $($(#[$field_meta])* $field_visibility $field: $ty),*
                }
                let fields = crate::serde_object::deserialize::<$name $(<$($generic),*>)?, D>(deserializer)?;
                Ok(Self { $($field: fields.$field),* })
            }
        }
    };
}
pub(crate) use object_record;

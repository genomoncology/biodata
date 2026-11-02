// JSON strings use the public parser, so serde cannot bypass validation.
/// Implement validated string serialization and schema using BioData's features.
#[macro_export]
macro_rules! text_value {
    ($ty:ty, $name:expr, $pattern:expr) => {
        $crate::text_value!(
            $ty,
            $name,
            $pattern,
            concat!("A validated canonical ", $name, " string.")
        );
    };
    ($ty:ty, $schema_name:expr, $pattern:expr, $description:expr) => {
        $crate::__biodata_text_serde!($ty, $schema_name);
        $crate::__biodata_text_schema!($ty, $schema_name, $pattern, $description);
    };
}
#[doc(hidden)]
#[cfg(feature = "serde")]
#[macro_export]
macro_rules! __biodata_text_serde {
    ($ty:ty, $schema_name:expr) => {
        impl $crate::__serde::Serialize for $ty {
            fn serialize<S: $crate::__serde::Serializer>(
                &self,
                serializer: S,
            ) -> Result<S::Ok, S::Error> {
                serializer.collect_str(self)
            }
        }

        impl<'de> $crate::__serde::Deserialize<'de> for $ty {
            fn deserialize<D: $crate::__serde::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Self, D::Error> {
                struct Visitor;
                impl $crate::__serde::de::Visitor<'_> for Visitor {
                    type Value = $ty;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        formatter.write_str(concat!("canonical ", $schema_name, " string"))
                    }
                    fn visit_str<E: $crate::__serde::de::Error>(
                        self,
                        value: &str,
                    ) -> Result<$ty, E> {
                        value.parse().map_err(E::custom)
                    }
                }
                deserializer.deserialize_str(Visitor)
            }
        }
    };
}
#[doc(hidden)]
#[cfg(not(feature = "serde"))]
#[macro_export]
macro_rules! __biodata_text_serde {
    ($($tokens:tt)*) => {};
}
#[doc(hidden)]
#[cfg(feature = "schema")]
#[macro_export]
macro_rules! __biodata_text_schema {
    ($ty:ty, $schema_name:expr, $pattern:expr, $description:expr) => {
        impl $ty {
            /// Schema pattern for the parser's accepted spelling.
            pub fn pattern() -> String { ($pattern).into() }
        }
        impl $crate::__schemars::JsonSchema for $ty {
            fn schema_name() -> std::borrow::Cow<'static, str> {
                $schema_name.into()
            }
            fn json_schema(_: &mut $crate::__schemars::SchemaGenerator) -> $crate::__schemars::Schema {
                $crate::__schemars::json_schema!({"description":$description, "type":"string", "pattern":Self::pattern()})
            }
        }    };
}
#[doc(hidden)]
#[cfg(not(feature = "schema"))]
#[macro_export]
macro_rules! __biodata_text_schema {
    ($($tokens:tt)*) => {};
}

/// Define one closed wire vocabulary.
#[macro_export]
macro_rules! wire_enum {
    (described $(#[$($doc:tt)*])* $name:ident { $($(#[doc = $vdoc:literal])* $variant:ident => $literal:literal),+ $(,)? }) => {
        $crate::wire_enum! { @attributes [schemars(extend("x-enum-descriptions" = [$(concat!($($vdoc),*)),+]))] [] []
            $(#[$($doc)*])* ; $name { $($(#[doc = $vdoc])* $variant => $literal),+ }
        }
    };
    ($(#[$($doc:tt)*])* $name:ident { $($(#[$vdoc:meta])* $variant:ident => $literal:literal),+ $(,)? }) => {
        $crate::wire_enum! { @attributes [] [] [] $(#[$($doc)*])* ; $name { $($(#[$vdoc])* $variant => $literal),+ } }
    };
    (@attributes [$($schema:meta)*] [$($docs:tt)*] [$($schema_docs:tt)*]
        #[cfg_attr(feature = "schema", schemars($($extension:tt)*))] $($rest:tt)*) => {
        $crate::wire_enum! { @attributes [$($schema)* schemars($($extension)*)] [$($docs)*] [$($schema_docs)*] $($rest)* }
    };
    (@attributes [$($schema:meta)*] [$($docs:tt)*] [$($schema_docs:tt)*]
        #[schemars($($extension:tt)*)] $($rest:tt)*) => {
        $crate::wire_enum! { @attributes [$($schema)* schemars($($extension)*)] [$($docs)*] [$($schema_docs)*] $($rest)* }
    };
    (@attributes [$($schema:meta)*] [$($docs:tt)*] [$($schema_docs:tt)*] #[doc = $doc:expr] $($rest:tt)*) => {
        $crate::wire_enum! { @attributes [$($schema)*] [$($docs)* #[doc = $doc]] [$($schema_docs)* #[doc = $doc]] $($rest)* }
    };
    (@attributes [$($schema:meta)*] [$($docs:tt)*] [$($schema_docs:tt)*] #[$doc:meta] $($rest:tt)*) => {
        $crate::wire_enum! { @attributes [$($schema)*] [$($docs)* #[$doc]] [$($schema_docs)*] $($rest)* }
    };
    (@attributes [$($schema:meta)*] [$(#[$doc:meta])*] [$($schema_docs:tt)*] ; $name:ident { $($(#[$vdoc:meta])* $variant:ident => $literal:literal),+ }) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        $(#[$doc])*
        pub enum $name { $($(#[$vdoc])* $variant,)+ }
        $crate::__biodata_wire_serde!($name { $($variant => $literal),+ });
        $crate::__biodata_wire_schema! { [$($schema)*] $($schema_docs)* $name { $($variant => $literal),+ } }
        impl $name {
            /// Canonical wire spelling from the enum declaration.
            pub const fn as_str(self) -> &'static str { match self { $(Self::$variant => $literal,)+ } }
        }
        impl std::str::FromStr for $name {
            type Err = $crate::Error;
            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match value {
                    $($literal => Ok(Self::$variant),)+
                    _ => Err($crate::Error::WireEnumUnknown),
                }
            }
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
    };
}

#[doc(hidden)]
#[cfg(feature = "serde")]
#[macro_export]
macro_rules! __biodata_wire_serde {
    ($name:ident { $($variant:ident => $literal:literal),+ }) => {
        impl $crate::__serde::Serialize for $name {
            fn serialize<S: $crate::__serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                match self { $(Self::$variant => serializer.serialize_unit_variant(stringify!($name), Self::$variant as u32, $literal),)+ }
            }
        }
        impl<'de> $crate::__serde::Deserialize<'de> for $name {
            fn deserialize<D: $crate::__serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                struct Visitor;
                impl<'input> $crate::__serde::de::Visitor<'input> for Visitor {
                    type Value = $name;
                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str(concat!("a declared ", stringify!($name), " value"))
                    }
                    fn visit_str<E: $crate::__serde::de::Error>(self, value: &str) -> Result<$name, E> {
                        match value {
                            $($literal => Ok($name::$variant),)+
                            _ => Err(E::custom($crate::Error::WireEnumUnknown)),
                        }
                    }
                    fn visit_bool<E: $crate::__serde::de::Error>(self, _: bool) -> Result<$name, E> {
                        Err(E::custom($crate::Error::WireEnumUnknown))
                    }
                    fn visit_i64<E: $crate::__serde::de::Error>(self, _: i64) -> Result<$name, E> {
                        Err(E::custom($crate::Error::WireEnumUnknown))
                    }
                    fn visit_u64<E: $crate::__serde::de::Error>(self, _: u64) -> Result<$name, E> {
                        Err(E::custom($crate::Error::WireEnumUnknown))
                    }
                    fn visit_i128<E: $crate::__serde::de::Error>(self, _: i128) -> Result<$name, E> {
                        Err(E::custom($crate::Error::WireEnumUnknown))
                    }
                    fn visit_u128<E: $crate::__serde::de::Error>(self, _: u128) -> Result<$name, E> {
                        Err(E::custom($crate::Error::WireEnumUnknown))
                    }
                    fn visit_f64<E: $crate::__serde::de::Error>(self, _: f64) -> Result<$name, E> {
                        Err(E::custom($crate::Error::WireEnumUnknown))
                    }
                    fn visit_char<E: $crate::__serde::de::Error>(self, _: char) -> Result<$name, E> {
                        Err(E::custom($crate::Error::WireEnumUnknown))
                    }
                    fn visit_bytes<E: $crate::__serde::de::Error>(self, _: &[u8]) -> Result<$name, E> {
                        Err(E::custom($crate::Error::WireEnumUnknown))
                    }
                    fn visit_unit<E: $crate::__serde::de::Error>(self) -> Result<$name, E> {
                        Err(E::custom($crate::Error::WireEnumUnknown))
                    }
                    fn visit_seq<A: $crate::__serde::de::SeqAccess<'input>>(self, _: A) -> Result<$name, A::Error> {
                        Err($crate::__serde::de::Error::custom($crate::Error::WireEnumUnknown))
                    }
                    fn visit_map<A: $crate::__serde::de::MapAccess<'input>>(self, _: A) -> Result<$name, A::Error> {
                        Err($crate::__serde::de::Error::custom($crate::Error::WireEnumUnknown))
                    }
                }
                deserializer.deserialize_any(Visitor)
            }
        }
    };
}
#[doc(hidden)]
#[cfg(not(feature = "serde"))]
#[macro_export]
macro_rules! __biodata_wire_serde {
    ($($tokens:tt)*) => {};
}

#[doc(hidden)]
#[cfg(feature = "schema")]
#[macro_export]
macro_rules! __biodata_wire_schema {
    ([$($schema:meta)*] $(#[$doc:meta])* $name:ident { $($variant:ident => $literal:literal),+ }) => {
        impl $crate::__schemars::JsonSchema for $name {
            fn schema_name() -> std::borrow::Cow<'static, str> { stringify!($name).into() }
            fn schema_id() -> std::borrow::Cow<'static, str> { concat!(module_path!(), "::", stringify!($name)).into() }
            fn json_schema(generator: &mut $crate::__schemars::SchemaGenerator) -> $crate::__schemars::Schema {
                use $crate::__schemars as schemars;
                $(#[$doc])*
                #[derive(schemars::JsonSchema)]
                #[schemars(with = "str", extend("enum" = [$($literal),+]))]
                $(#[$schema])*
                struct VocabularySchema;
                <VocabularySchema as schemars::JsonSchema>::json_schema(generator)
            }
        }
    };
}
#[doc(hidden)]
#[cfg(not(feature = "schema"))]
#[macro_export]
macro_rules! __biodata_wire_schema {
    ($($tokens:tt)*) => {};
}

// Define each canonical spelling once for parsing, display and JSON.
/// Define one canonical string vocabulary.
#[macro_export]
macro_rules! canonical_enum {
    ($(#[$($doc:tt)*])* $name:ident, $error:ident { $($(#[doc = $variant_doc:literal])* $variant:ident => $text:literal),+ $(,)? }) => {
        $crate::canonical_enum! { @attributes [] [] $(#[$($doc)*])* ; $name, $error { $($(#[doc = $variant_doc])* $variant => $text),+ } }
    };
    (@attributes [$($attrs:tt)*] [$($schema_docs:tt)*]
        #[cfg_attr(feature = "schema", schemars($($extension:tt)*))] $($rest:tt)*) => {
        $crate::canonical_enum! { @attributes [$($attrs)*] [$($schema_docs)* #[schemars($($extension)*)]] $($rest)* }
    };
    (@attributes [$($attrs:tt)*] [$($schema_docs:tt)*] #[schemars($($extension:tt)*)] $($rest:tt)*) => {
        $crate::canonical_enum! { @attributes [$($attrs)*] [$($schema_docs)* #[schemars($($extension)*)]] $($rest)* }
    };
    (@attributes [$($attrs:tt)*] [$($schema_docs:tt)*] #[doc = $doc:expr] $($rest:tt)*) => {
        $crate::canonical_enum! { @attributes [$($attrs)* #[doc = $doc]] [$($schema_docs)* #[doc = $doc]] $($rest)* }
    };
    (@attributes [$($attrs:tt)*] [$($schema_docs:tt)*] #[$attr:meta] $($rest:tt)*) => {
        $crate::canonical_enum! { @attributes [$($attrs)* #[$attr]] [$($schema_docs)*] $($rest)* }
    };
    (@attributes [$($attrs:tt)*] [$($schema_docs:tt)*] ; $name:ident, $error:ident { $($(#[doc = $variant_doc:literal])* $variant:ident => $text:literal),+ }) => {
        $($attrs)*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum $name {
            $($(#[doc = $variant_doc])* $variant),+
        }

        $crate::__biodata_enum_serde!($name);
        $crate::__biodata_enum_schema! { $($schema_docs)* $name { $($(#[doc = $variant_doc])* $variant => $text),+ } }

        impl $name {
            /// Every declared value, in declaration order.
            pub const ALL: &'static [Self] = &[$(Self::$variant),+];
        }

        impl From<$name> for &'static str {
            fn from(value: $name) -> Self {
                match value { $($name::$variant => $text),+ }
            }
        }

        impl std::str::FromStr for $name {
            type Err = $crate::Error;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match value {
                    $($text => Ok(Self::$variant),)+
                    _ => Err($crate::Error::$error),
                }
            }
        }

        impl TryFrom<String> for $name {
            type Error = $crate::Error;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                value.parse()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str((*self).into())
            }
        }
    };
}

#[doc(hidden)]
#[cfg(feature = "serde")]
#[macro_export]
macro_rules! __biodata_enum_serde {
    ($name:ident) => {
        impl $crate::__serde::Serialize for $name {
            fn serialize<S: $crate::__serde::Serializer>(
                &self,
                serializer: S,
            ) -> Result<S::Ok, S::Error> {
                serializer.serialize_str((*self).into())
            }
        }
        impl<'de> $crate::__serde::Deserialize<'de> for $name {
            fn deserialize<D: $crate::__serde::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Self, D::Error> {
                let value = <String as $crate::__serde::Deserialize>::deserialize(deserializer)?;
                Self::try_from(value).map_err($crate::__serde::de::Error::custom)
            }
        }
    };
}
#[doc(hidden)]
#[cfg(not(feature = "serde"))]
#[macro_export]
macro_rules! __biodata_enum_serde {
    ($($tokens:tt)*) => {};
}
#[doc(hidden)]
#[cfg(feature = "schema")]
#[macro_export]
macro_rules! __biodata_enum_schema {
    ($(#[$($attr:tt)*])* $name:ident { $($(#[doc = $vdoc:literal])* $variant:ident => $text:literal),+ }) => {
        $crate::__biodata_enum_schema! { @attributes [] $(#[$($attr)*])* ; $name { $($(#[doc = $vdoc])* $variant => $text),+ } }
    };
    (@attributes [$($docs:tt)*] #[doc = $doc:expr] $($rest:tt)*) => {
        $crate::__biodata_enum_schema! { @attributes [$($docs)* #[doc = $doc]] $($rest)* }
    };
    (@attributes [$($docs:tt)*] #[cfg_attr(feature = "schema", schemars($($extension:tt)*))] $($rest:tt)*) => {
        $crate::__biodata_enum_schema! { @attributes [$($docs)* #[schemars($($extension)*)]] $($rest)* }
    };
    (@attributes [$($docs:tt)*] #[schemars($($extension:tt)*)] $($rest:tt)*) => {
        $crate::__biodata_enum_schema! { @attributes [$($docs)* #[schemars($($extension)*)]] $($rest)* }
    };
    (@attributes [$($docs:tt)*] #[$other:meta] $($rest:tt)*) => {
        $crate::__biodata_enum_schema! { @attributes [$($docs)*] $($rest)* }
    };
    (@attributes [$($docs:tt)*] ; $name:ident { $($(#[doc = $vdoc:literal])* $variant:ident => $text:literal),+ }) => {
        impl $crate::__schemars::JsonSchema for $name {
            fn schema_name() -> std::borrow::Cow<'static, str> { stringify!($name).into() }
            fn schema_id() -> std::borrow::Cow<'static, str> { concat!(module_path!(), "::", stringify!($name)).into() }
            fn json_schema(generator: &mut $crate::__schemars::SchemaGenerator) -> $crate::__schemars::Schema {
                use $crate::__schemars as schemars;
                #[derive(schemars::JsonSchema)]
                $($docs)*
                #[schemars(with = "str", extend("x-enum-open" = true, "enum" = [$($text),+], "x-enum-descriptions" = [$(concat!($($vdoc),*)),+]))]
                struct VocabularySchema;
                <VocabularySchema as schemars::JsonSchema>::json_schema(generator)
            }
        }
    };
}
#[doc(hidden)]
#[cfg(not(feature = "schema"))]
#[macro_export]
macro_rules! __biodata_enum_schema {
    ($($tokens:tt)*) => {};
}

/// Define a publisher vocabulary that retains new source values unchanged.
#[macro_export]
macro_rules! native_enum {
    ($(#[$doc:meta])* $name:ident { $($(#[$vdoc:meta])* $variant:ident => $literal:literal),+ $(,)? }) => {
        $(#[$doc])*
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub enum $name {
            $($(#[$vdoc])* $variant,)+
            /// A new publisher value, retained without interpretation.
            Unknown($crate::SourceText),
        }
        impl $name {
            /// Borrow the exact publisher spelling.
            pub fn as_str(&self) -> &str {
                match self { $(Self::$variant => $literal,)+ Self::Unknown(value) => value.as_str() }
            }
            /// Whether this value falls outside the published vocabulary.
            pub fn is_unknown(&self) -> bool { matches!(self, Self::Unknown(_)) }
        }
        impl From<String> for $name {
            fn from(value: String) -> Self {
                match value.as_str() { $($literal => Self::$variant,)+ _ => Self::Unknown(value.into()) }
            }
        }
        $crate::__biodata_native_serde!($name);
        $crate::__biodata_native_schema!($name { $($literal),+ });
    }
}

#[doc(hidden)]
#[cfg(feature = "serde")]
#[macro_export]
macro_rules! __biodata_native_serde {
    ($name:ident) => {
        impl<'de> $crate::__serde::Deserialize<'de> for $name {
            fn deserialize<D: $crate::__serde::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Self, D::Error> {
                <String as $crate::__serde::Deserialize>::deserialize(deserializer).map(Self::from)
            }
        }
        impl $crate::__serde::Serialize for $name {
            fn serialize<S: $crate::__serde::Serializer>(
                &self,
                serializer: S,
            ) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(self.as_str())
            }
        }
    };
}
#[doc(hidden)]
#[cfg(not(feature = "serde"))]
#[macro_export]
macro_rules! __biodata_native_serde {
    ($($tokens:tt)*) => {};
}

#[doc(hidden)]
#[cfg(feature = "schema")]
#[macro_export]
macro_rules! __biodata_native_schema {
    ($name:ident { $($literal:literal),+ }) => {
        impl $crate::__schemars::JsonSchema for $name {
            fn schema_name() -> std::borrow::Cow<'static, str> { stringify!($name).into() }
            fn json_schema(_: &mut $crate::__schemars::SchemaGenerator) -> $crate::__schemars::Schema {
                $crate::__schemars::json_schema!({"type":"string", "description":concat!(
                    "Native publisher vocabulary; new values are retained unchanged. Known values: ",
                    $($literal, "; ",)+
                )})
            }
        }
    };
}
#[doc(hidden)]
#[cfg(not(feature = "schema"))]
#[macro_export]
macro_rules! __biodata_native_schema {
    ($($tokens:tt)*) => {};
}

/// Define a checked identifier whose source spelling is its identity.
#[macro_export]
macro_rules! checked_id {
    ($name:ident, $error:ident, $check:expr, $pattern:expr, $doc:expr) => {
        #[doc = $doc]
        #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(Box<str>);
        impl $name {
            /// Check the complete spelling without normalization.
            pub fn new(value: &str) -> Result<Self, $crate::Error> {
                if ($check)(value) {
                    Ok(Self(value.into()))
                } else {
                    Err($crate::Error::$error)
                }
            }
            /// Borrow the unchanged source identifier.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
        impl std::str::FromStr for $name {
            type Err = $crate::Error;
            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        $crate::text_value!($name, stringify!($name), $pattern, $doc);
    };
}

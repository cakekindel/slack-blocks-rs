#[macro_use]
extern crate validator_derive;

pub mod block_elements;
pub mod blocks;
pub mod compose;
pub mod val_helpr;

#[macro_export]
macro_rules! impl_from_contents {
    ($enum_name:ident, $variant:ident, $contents_type:ty) => {
        impl From<$contents_type> for $enum_name {
            fn from(contents: $contents_type) -> Self {
                $enum_name::$variant(contents)
            }
        }
    }
}


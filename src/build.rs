//! Module containing helper types for static builder structs

use std::marker::PhantomData;

/// Type indicating that some required marker `T` has been set
pub struct Set<T>(PhantomData<T>);

/// Type indicating that some required marker `T` has not been set
pub struct Unset<T>(PhantomData<T>);

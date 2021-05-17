//! Module containing helper types for static builder structs

use std::marker::PhantomData;

/// Type indicating that some marker `T` has been set
pub struct Set<T>(PhantomData<T>);

/// Type indicating that some optional marker `T` has not been set
pub struct OptionalMethodNotCalled<T>(PhantomData<T>);

/// Type indicating that some required marker `T` has not been set
pub struct RequiredMethodNotCalled<T>(PhantomData<T>);

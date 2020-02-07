// Trait that is automatically implemented for all types that implement `Drop`.
//
// Used to cause a conflicting trait impl if a type implements `Drop` to forbid implementing `Drop`.
#[doc(hidden)]
pub trait MustNotImplDrop {}

impl<T: Drop> MustNotImplDrop for T {}

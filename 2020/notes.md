# Rustlang Notes

`.unwrap()` accesses the inner `Ok` value in a `Result`, but can panic so in general it's safer to pattern match on the result and handle the `Err` case or use `unwrap_or`, `unwrap_or_else`, or `unwrap_or_deafult`.

`_` can be used as a non-specific/generic type placeholder (need more info on this)

`.collect()` is useful for type-casting (e.g. into `Vec<_>`)

variable decls and their associated types determine which methods can be called and how functions like `.into()` behave.

`where` guards can put extra constraints on types (e.g. `where Self: Sized` in a `trait` that returns an `Option` wrapping the type that implements the trait - because `Option` requires a fixed-sized inner type) and can be used within `traits` to propagate those constraints on to types that implement those traits

`Iterable::filter_map()` is useful for filtering while enumerating an iterable (and will remove `None`s automatically)

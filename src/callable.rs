// A synonym for `Fn`.
//
// Having to write `call` everywhere is a bit annoying, but I couldn't get rid of it in `memoize`.
// When I tried to replace `Callable` with `Fn` one of the two things always happend: either it
// ended up requiring `'static`, which made it impossible to use closures that capture variables; or
// Rust compiler was complaining about recursive type definitions, and even `Box`ing didn't help.
// I'm not sure why a new trait would help with these problems. Either there is some intrinsic
// limitation to the `Fn` trait, or I just didn't figure out how to use it properly.
//
pub trait Callable<T, U> {
    fn call(&self, arg: T) -> U;
}

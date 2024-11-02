This small crate is an adapter crate. It provides no functionality itself,
only an interface for functionality.

This allows you to make other crates that can all work together without
having dependencies on one another. You're probably familiar with "interfaces"
and this is a similar idea.

[SizeLimitedCache](./src/cache_trait.rs) is a simplistic cache trait that assumes
a fixed size policy.

[ShareableCache](./src/shareable_cache.rs) is a mirror of the SizeLimitedCache
trait, but with the ownership requirements relaxed so that you can use it directly
with multiple threads.

These traits are used by the [benchmarking](../benchmarking/README.md) crate to test
different cache implementations on a level playing field.

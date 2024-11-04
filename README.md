Welcome to the Rust workshop!

Clone this repository and start working through the modules at your own pace.

**Modules:**
1. [Intro to Rust](./intro/README.md): Start here if you are new to Rust. Read the readme either way to see if you'd like a quick refresher.
1. [Sieve Cache](./sieve_cache/README.md): Learn applied rust by writing a modern cache eviction algorithm.
1. [Benchmarking](./benchmarking/README.md): Learn tools and techniques for measuring performance of Rust code.

### Prerequisites
* MacOS, Linux, or WSL. You might be able to do this with Windows, but I've not tried!
* An IDE. VS Code is a good choice, but you can use vim, emacs, or some Jetbrains thing if you prefer.
* Rust stable toolchain must be installed. [rustup](https://rustup.rs/) is the usual way.
* gcc and gnuplot for visualizations, perf for detailed profiling.
  * On MacOS `brew install gnuplot` will get you there. If you miss this, you'll just have less smooth graphs :shrug:
  * `perf` is more for linux - it's really for going deep on latency measurements with flamegraphs. We'll be using another means for flamegraphs in this workshop so MacOS people can play along, but `perf`-backed `cargo flamegraph` is an option if you like.

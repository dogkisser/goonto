# Goonto

If you have questions, please read the [FAQ](https://github.com/zoomasochist/goonto/wiki/FAQ/).

## Building

Debug builds (`cargo build`) differ slightly from release builds (`cargo build --release`). Notably, you can run Goonto without elevated permissions. This may break some features.

### macOS / Windows

```
$ cargo build --release
$ target/release/goonto
```

### Linux

Goonto has a few additional dependencies on Linux. They're listed in the [Release build workflow](https://github.com/zoomasochist/goonto/blob/master/.github/workflows/release.yml#L32).

It's just standard X11 libs, `pkg-config`, and `cmake`. You probably have them installed already.

Then:

```
$ cargo build --release
$ target/release/goonto
```

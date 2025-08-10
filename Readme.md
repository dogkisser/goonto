# Goonto

If you have questions, please read the [FAQ](https://github.com/dogkisser/goonto/wiki/FAQ/).

## Building

**You don't need to do this.** If you just want to _use_ Goonto, read the FAQ entry [How do I run Goonto?](https://github.com/dogkisser/goonto/wiki/FAQ#how-do-i-run-goonto)

Debug builds (`cargo build`) differ slightly from release builds (`cargo build --release`).
Notably, you can run Goonto without elevated permissions. This may break some
features.

### macOS / Windows

```
$ cargo build --release
$ target/release/goonto
```

### Linux

Goonto has a few additional dependencies on Linux. They're listed in the
[Release build workflow](https://github.com/dogkisser/goonto/blob/master/.github/workflows/release.yml#L32).

It's just standard X11 libs, `pkg-config`, and `cmake`. You probably have them
installed already.

Then:

```
$ cargo build --release
$ target/release/goonto
```

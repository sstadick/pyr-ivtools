# pyr-lapper
A Python warapper around rust-lapper


## Build
Add to ~/.cargo/config

```toml
[target.x86_64-apple-darwin]
rustflags = [
  "-C", "link-arg=-undefined",
  "-C", "link-arg=dynamic_lookup",
]
```

## Notes
https://github.com/PyO3/maturin


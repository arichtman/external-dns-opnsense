# external-dns-opnsense

External-DNS webhook extension for OPNsense

Very much a work in progress and a learning experience.

```
cargo watch --watch src/ --quiet --clear --exec run
cargo watch --watch src/ --quiet --clear --shell bacon
cargo watch --watch src/ --quiet --clear --exec test
```

## Notes

- Tried to add `rust-toolchain.toml` to get nightly `rustfmt` so I could set module granularity.
  I didn't particularly want everything nightly.
  It may be possible to just add `+nightly` argument to the format call.
  However all the discussion I turned up was for VSCode,
  and I can't locate any options for in-repo Helix config.
  Nightly also broke compilation,
  [issue](https://github.com/tokio-rs/axum/issues/2407)

## References

- [EDNS FAQ](https://github.com/kubernetes-sigs/external-dns/blob/master/docs/faq.md)
- [EDNS webhook tutorial](https://github.com/kubernetes-sigs/external-dns/blob/master/docs/tutorials/webhook-provider.md)
- [EDNS OpenAPI spec issue](https://github.com/kubernetes-sigs/external-dns/issues/4138)
- [OPNsense Unbound API reference](https://docs.opnsense.org/development/api/core/unbound.html)
- [OPNsense forum post](https://forum.opnsense.org/index.php?topic=25823.0)
- [Mo8it's similar project](https://codeberg.org/mo8it/git-webhook-client/src/commit/61bcd61399570fdb67a535cd47ee7a19445f6360)
- [Jeremy Chone's Axum course](https://github.com/jeremychone-channel/rust-axum-course)

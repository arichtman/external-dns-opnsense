# external-dns-opnsense

External-DNS webhook extension for OPNsense

Very much a work in progress and a learning experience.

```
bacon
cargo watch --watch src/ --quiet --clear --exec run
cargo watch --watch src/ --quiet --clear --exec test
```

## References

- [EDNS FAQ](https://github.com/kubernetes-sigs/external-dns/blob/master/docs/faq.md)
- [EDNS webhook tutorial](https://github.com/kubernetes-sigs/external-dns/blob/master/docs/tutorials/webhook-provider.md)
- [EDNS OpenAPI spec issue](https://github.com/kubernetes-sigs/external-dns/issues/4138)
- [OPNsense Unbound API reference](https://docs.opnsense.org/development/api/core/unbound.html)
- [OPNsense forum post](https://forum.opnsense.org/index.php?topic=25823.0)
- [Mo8it's similar project](https://codeberg.org/mo8it/git-webhook-client/src/commit/61bcd61399570fdb67a535cd47ee7a19445f6360)
- [Jeremy Chone's Axum course](https://github.com/jeremychone-channel/rust-axum-course)

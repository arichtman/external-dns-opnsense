# external-dns-opnsense

External-DNS webhook extension for OPNsense

Very much a work in progress and a learning experience.

Live-watch dev environment

```sh
cargo watch --watch src/ --quiet --clear --exec run
cargo watch --watch src/ --quiet --clear --shell bacon
cargo watch --watch src/ --quiet --clear --exec test
```

Reflection snippet - not for prod

```rust
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

print_type_of(&override_list[0]);
```

## Features

- Pooled HTTP clients.
  Client is held in shared state but seems to be releasing connections reasonably quickly.
  It may turn out that our HTTP use just isn't intensive enough to justify pooling.
- OpenTelemetry Prometheus metrics
- Blazingly fast 🚀 (not to write lol, I'm slow as shit)

## Notes

- There are quite a few Rust questions marked with `// TODO`, I should find a mentor.
- See `opnsense-api.md` for some captured calls from the GUI.
  See `openapi.yaml` for a draft OpenAPI v3 spec for the webservice.
- I wish there was something better for git hook management than `rusty-hook`.
- I'm some way towards putting the openapi fuzzer into the development environment.
  It's been a tremendous pain and the flake looks like shit now.
  Oh and of course it doesn't work anyways and to just install it unmanaged would have been seconds.
- I'm concerned search is pretty greedy with results.
  I'll have to think about a non-naieve implementation for this.
  We can keep uuids in state after creation, but what happens when:
  a) service restarts and loses that state, records are still present,
  b) record is modified or removed, state not updated.
  Hitting the service to search every time, getting too many results, and having to filter them
  for every record is way too much.
  Also it looks like there's no rejection of addHostOverride with same host+domain+type.
  That's no bueno cause we can't even rely on stubbing our toe and recovering.
  Perhaps we pull all overrides on boot and filter against our domain list to populate state.
- It may be nice to compare our domain filter against `/unbound/diagnostic/listlocalzones`.
  But this is a nice-to-have.

## References

- [EDNS FAQ](https://github.com/kubernetes-sigs/external-dns/blob/master/docs/faq.md)
- [EDNS webhook tutorial](https://github.com/kubernetes-sigs/external-dns/blob/master/docs/tutorials/webhook-provider.md)
- [EDNS OpenAPI spec issue](https://github.com/kubernetes-sigs/external-dns/issues/4138)
- [OPNsense Unbound API reference](https://docs.opnsense.org/development/api/core/unbound.html)
- [OPNsense forum post](https://forum.opnsense.org/index.php?topic=25823.0)
- [Mo8it's similar project](https://codeberg.org/mo8it/git-webhook-client/src/commit/61bcd61399570fdb67a535cd47ee7a19445f6360)
- [Jeremy Chone's Axum course](https://github.com/jeremychone-channel/rust-axum-course)

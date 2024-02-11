# external-dns-opnsense

External-DNS webhook extension for OPNsense

Very much a work in progress and a learning experience.

See `testing` directory for some supplementary stuff about the upstream API.

## Features

- Pooled HTTP clients.
  Client is held in shared state but seems to be releasing connections reasonably quickly.
  It may turn out that our HTTP use just isn't intensive enough to justify pooling.
- OpenTelemetry Prometheus metrics
- Blazingly fast 🚀 (not to write lol, I'm slow as shit)

## Notes

Concrete TODOs

- Destructure in params using data structure
- Add CI like jithub actions
- Think about publishing, crates? docker hub? WASM?
- Add info, trace and warn debug logging
- Pull business logic out into it's own module
- Use with_state to limit state only to routes that need it
- Review all `// TODO` with mentor
- See about git hooks and some CI checks.
  `rusty-hook` isn't greaaat but works.
- Review all uses of `unwrap`
- Add tracing / "proper" o11y stuff
- Add OpenAPI spec generation?
  Maybe better to add verification/contract testing
- Find a nice way to strip all quotes on Clap arguments
- Consider caching health checks
- Use a trait object to decouple access from handlers
- Put OPNsense CRUD logic into the client struct implementation
- Set derive macros to only apply during test/debug builds
- Create custome error codes for data access, then use a mapper to translate them into HTTP error codes for the response

Testing:

- Mock out opnsense access (using traits)
- Use a mock trait object to conduct unit tests
- Test using Arc for mutable shared state
- Add the openapi fuzzer into the development environment or test suite
  It's been a tremendous pain and the flake looks like shit now.
  Oh and of course it doesn't work anyways and to just install it unmanaged would have been seconds.
- Work out a shared app object so tests don't all instantiate their own "server"
- Add tests for at least all endpoints and methods
- See if `.body()` is actually required for `builder()`

I'll have to think about a non-naieve implementation for this.
I'm concerned search is pretty greedy with results.
We can keep uuids in state after creation, but what happens when:
a) service restarts and loses that state, records are still present,
b) record is modified or removed, state not updated.
Hitting the service to search every time, getting too many results, and having to filter them
for every record is way too much.

Also it looks like there's no rejection of addHostOverride with same host+domain+type.
That's no bueno cause we can't even rely on stubbing our toe and recovering.
Perhaps we pull all overrides on boot and filter against our domain list to populate state.

Also, for EDNS a record has `targets`, so possibly multiple overrides when implemented.
I think we need a separate step that pulls all overrides, filters to domains we care about, and populates the records list.
We also need an apply function that looks the records list and reconciles it with unbound - i.e. the multi-target translation.
Possibly also a deduplication thread that removes anything same record type, host, and domain.
It's probably also wise to have a configurable threshold for
a) total count of unbound records to stop errors wrecking the service, and
b) total count of records considered under management, to stop other blowouts

## Snippets

Live-watch dev environment

```sh
cargo watch --watch src/ --quiet --clear --exec run
bacon
cargo watch --watch src/ --quiet --clear --exec test
```

Reflection snippet - not for prod

```rust
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

print_type_of(&override_list[0]);
```

## References

- [EDNS FAQ](https://github.com/kubernetes-sigs/external-dns/blob/master/docs/faq.md)
- [EDNS webhook tutorial](https://github.com/kubernetes-sigs/external-dns/blob/master/docs/tutorials/webhook-provider.md)
- [EDNS OpenAPI spec issue](https://github.com/kubernetes-sigs/external-dns/issues/4138)
- [OPNsense Unbound API reference](https://docs.opnsense.org/development/api/core/unbound.html)
- [OPNsense forum post](https://forum.opnsense.org/index.php?topic=25823.0)
- [Mo8it's similar project](https://codeberg.org/mo8it/git-webhook-client/src/commit/61bcd61399570fdb67a535cd47ee7a19445f6360)
- [Jeremy Chone's Axum course](https://github.com/jeremychone-channel/rust-axum-course)
- [Rainer Stropek on unit testing Axum](https://www.youtube.com/watch?v=_cYIhG_3qSo), [repo](https://github.com/rstropek/rust-samples/tree/master/axum-di-testing)

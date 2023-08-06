**CAUTION: highly experimental, the Database implementation is likely horribly broken**

# Fedimint Client built with [Leptos](https://github.com/leptos-rs/leptos)

This repo contains a proof-of-concept of how to integrate `fedimint-client` with the Leptos web framework to build a
PWA. Nothing really works yet but it compiles.

To run it enter the `nix develop` shell, run `trunk serve` and open `http://127.0.0.1:8080` in your browser:

```
fedimint-leptos-test$ nix develop
💡 Run 'just' for a list of available 'just ...' helper recipes
fedimint-leptos-test$ trunk serve
2023-08-06T12:00:41.373844Z  INFO 📦 starting build
2023-08-06T12:00:41.374062Z  INFO spawning asset pipelines
2023-08-06T12:00:41.522959Z  INFO building fedimint-leptos-test
    Finished dev [unoptimized + debuginfo] target(s) in 0.14s
2023-08-06T12:00:41.673972Z  INFO fetching cargo artifacts
2023-08-06T12:00:41.834730Z  INFO processing WASM for fedimint-leptos-test
2023-08-06T12:00:41.933395Z  INFO calling wasm-bindgen for fedimint-leptos-test
2023-08-06T12:00:42.907392Z  INFO copying generated wasm-bindgen artifacts
2023-08-06T12:00:42.919447Z  INFO applying new distribution
2023-08-06T12:00:42.919736Z  INFO ✅ success
2023-08-06T12:00:42.919871Z  INFO 📡 serving static assets at -> /
2023-08-06T12:00:42.919899Z  INFO 📡 server listening at http://127.0.0.1:8080
```

You should see "Starting client" in the browser and the JS console should be logging a lot.
# Reproducing the Rexie panic

This is a minimal example that reproduces the panic in Rexie which occurs when transactions are held open across await points. To run it I used the nix shell of the parent project (but it's not optimized and downloads wayyyy too much data, if you still want to use it: `nix develop`), you can probably make do with a lighter set of tools:

* rust with wasm32-unknown-unknown target
* trunk
* npm (maybe?)

To run the example:

```sh
# maybe npm i first? I hope I removed tailwind but I don't understand this web stuff …
trunk serve
```

Then open the browser to http://localhost:8080 and open the console. You should see the following output in the browser console:

```
rexie-repro-79530fe8cd7a95b7.js:331 panicked at 'Could not get IndexedDB store: ObjectStoreOpenFailed(JsValue(InvalidStateError: Failed to execute 'objectStore' on 'IDBTransaction': The transaction has finished.
Error: Failed to execute 'objectStore' on 'IDBTransaction': The transaction has finished.
    at http://127.0.0.1:8080/rexie-repro-79530fe8cd7a95b7.js:439:33
    at handleError (http://127.0.0.1:8080/rexie-repro-79530fe8cd7a95b7.js:227:18)
    at imports.wbg.__wbg_objectStore_5a858a654147f96f (http://127.0.0.1:8080/rexie-repro-79530fe8cd7a95b7.js:437:70)
    at web_sys::features::gen_IdbTransaction::IdbTransaction::object_store::h16ae0fbe68d05c9b (http://127.0.0.1:8080/rexie-repro-79530fe8cd7a95b7_bg.wasm:wasm-function[746]:0x84b1f)
    at rexie::transaction::Transaction::store::h3f7dd814e2188c51 (http://127.0.0.1:8080/rexie-repro-79530fe8cd7a95b7_bg.wasm:wasm-function[867]:0x8e799)
    at rexie_repro::main::{{closure}}::h35cb0f02ccfff3a5 (http://127.0.0.1:8080/rexie-repro-79530fe8cd7a95b7_bg.wasm:wasm-function[69]:0x4a8d)
    at wasm_bindgen_futures::task::singlethread::Task::run::h5cb0297e09d1c303 (http://127.0.0.1:8080/rexie-repro-79530fe8cd7a95b7_bg.wasm:wasm-function[386]:0x5ef0a)
    at wasm_bindgen_futures::queue::QueueState::run_all::h831582888a8da0cf (http://127.0.0.1:8080/rexie-repro-79530fe8cd7a95b7_bg.wasm:wasm-function[293]:0x50b24)
    at wasm_bindgen_futures::queue::Queue::new::{{closure}}::h5e511fb7f5bca41e (http://127.0.0.1:8080/rexie-repro-79530fe8cd7a95b7_bg.wasm:wasm-function[2408]:0xdcb7b)
    at <dyn core::ops::function::FnMut<(A,)>+Output = R as wasm_bindgen::closure::WasmClosure>::describe::invoke::hbd2728dad75a2adf (http://127.0.0.1:8080/rexie-repro-79530fe8cd7a95b7_bg.wasm:wasm-function[1604]:0xbc4c9)))', src/main.rs:34:18

Stack:

Error
    at imports.wbg.__wbg_new_abda76e883ba8a5f (http://127.0.0.1:8080/rexie-repro-79530fe8cd7a95b7.js:334:17)
    at console_error_panic_hook::Error::new::hf81a258f4050326c (http://127.0.0.1:8080/rexie-repro-79530fe8cd7a95b7_bg.wasm:wasm-function[3986]:0xfe787)
    at console_error_panic_hook::hook_impl::hcfd90e524f6e0b60 (http://127.0.0.1:8080/rexie-repro-79530fe8cd7a95b7_bg.wasm:wasm-function[633]:0x7a25c)
    at console_error_panic_hook::hook::hbc56661f8a993ea0 (http://127.0.0.1:8080/rexie-repro-79530fe8cd7a95b7_bg.wasm:wasm-function[4357]:0x10387b)
    at core::ops::function::Fn::call::hb0151f217d44442e (http://127.0.0.1:8080/rexie-repro-79530fe8cd7a95b7_bg.wasm:wasm-function[3821]:0xfbe5d)
    at std::panicking::rust_panic_with_hook::hd000e9fb43b5781d (http://127.0.0.1:8080/rexie-repro-79530fe8cd7a95b7_bg.wasm:wasm-function[1555]:0xb9bb8)
    at std::panicking::begin_panic_handler::{{closure}}::he16e52e9a7dddeb1 (http://127.0.0.1:8080/rexie-repro-79530fe8cd7a95b7_bg.wasm:wasm-function[2045]:0xd006c)
    at std::sys_common::backtrace::__rust_end_short_backtrace::h227361e053771d9e (http://127.0.0.1:8080/rexie-repro-79530fe8cd7a95b7_bg.wasm:wasm-function[4606]:0x106b09)
    at rust_begin_unwind (http://127.0.0.1:8080/rexie-repro-79530fe8cd7a95b7_bg.wasm:wasm-function[3207]:0xf09b0)
    at core::panicking::panic_fmt::h9d972fcdb087ce21 (http://127.0.0.1:8080/rexie-repro-79530fe8cd7a95b7_bg.wasm:wasm-function[4221]:0x101c5c)
```


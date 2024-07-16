# red4ext-rs-hot-reload
Basic setup for hot reloading red4ext-rs. Experimental!

## Instructions:

Add the code you want to hot reload into the reload/src/lib.rs file.

```rust
extern "C" fn interact() -> i32 {
    // Add your code here!
    return ret;
}

```


Compile the main crate (and the subcrate) with `cargo build`

Then copy the `redext-hot-reload.dll` from your `targets/$(BUILD_TYPE)` to `${GAME_DIR}/red4ext/plugins/hot-reload/*`

Trigger the plugin in-game by typing in `print(ReloadTest())` in the CET console.

To update, compile the subcrate with `cargo build -p reload` 

Then copy the `reload.dll` from your `targets/$(BUILD_TYPE)` to `${GAME_DIR}/red4ext/plugins/hot-reload/*`

Whenever you rebuild and then copy `reload.dll` the `print(ReloadTest())` will be updated with the new function.

You can write a little script to do this for you on file change.
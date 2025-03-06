# `waybar-cffi`

This provides Rust bindings to create [CFFI modules for
Waybar][cffi].

Waybar CFFI modules are shared libraries that provide modules that can be
included in a Waybar. These can use the full capabilities of Gtk 3 and, more
generally, native code.

## Quick start

Creating and using a CFFI module is (relatively) easy.

1. Create a `cdylib` crate:

   ```toml
   [package]
   name = "my-module-name"
   version = "0.1.0"
   edition = "2024"

   [lib]
   crate-type = ["cdylib"]

   [dependencies]
   waybar-cffi = "0.1.0"
   ```

2. Implement the `waybar_cffi::Module` trait on a type, and use the
   `waybar_cffi::waybar_module` macro to export the required symbols. (See [the
   hello world](waybar-cffi/examples/hello-world.rs) example or the
   documentation for more detail.)

3. Build in the normal way with `cargo build`.

4. Configure `waybar` per [the CFFI instructions][cffi]:

   ```json
   {
     "modules-left": ["cffi/my-module-name"],
     "cffi/my-module-name": {
       "module-path": "target/debug/libmy_module_name.so"
     }
   }
   ```

5. Profit!

Refer to [the `waybar-cffi` documentation][docs] for more details.

## Development

Honestly, I probably won't be devoting a tonne of time to this in the near
future, but I'm definitely open to PRs.

### Layout

This workspace contains two crates:

- [`waybar-cffi`](waybar-cffi/): the main entry point into the CFFI
  functionality.
- [`waybar-cffi-sys`](waybar-cffi-sys/): the low level bindings based on [the
  CFFI header][header].

### Updating bindings

New Waybar versions will likely require the bindings to be updated. This can be
done by running `make clean && make ffi WAYBAR_ROOT=path/to/Waybar`.

Note that a full Waybar checkout is currently required as the CFFI header isn't
shipped in the release packages.

### Smoke tests

`make hello-world` will build [the hello world
example](waybar-cffi/examples/hello-world.rs) and run Waybar with two instances
of it configured.

[cffi]: https://github.com/Alexays/Waybar/wiki/Module:-CFFI
[docs]: https://docs.rs/waybar-cffi
[header]: https://github.com/Alexays/Waybar/tree/master/resources/custom_modules/cffi_example/waybar_cffi_module.h

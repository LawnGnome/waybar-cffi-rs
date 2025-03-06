BINDGEN := bindgen
CARGO := cargo
JQ := jq
MESON := meson
WAYBAR := waybar

clean:
	rm -f waybar-cffi-sys/src/raw.rs waybar-cffi-sys/WAYBAR_VERSION

ffi: waybar-cffi-sys/src/raw.rs waybar-cffi-sys/WAYBAR_VERSION

.PHONY: hello-world
hello-world:
	$(CARGO) build -p waybar-cffi --example hello-world
	$(WAYBAR) -c waybar-cffi/examples/hello-world.json

waybar-cffi-sys/src/raw.rs: waybar-cffi-sys/src/wrapper.h
	@if [ -z "$(WAYBAR_ROOT)" ]; then echo "Must provide the Waybar source tree root via the WAYBAR_ROOT environment variable."; exit 1; fi
	$(BINDGEN) \
		--allowlist-function=wbcffi_init \
		--allowlist-function=wbcffi_deinit \
		--allowlist-function=wbcffi_update \
		--allowlist-function=wbcffi_refresh \
		--allowlist-function=wbcffi_doaction \
		--allowlist-var=wbcffi_version \
		-o $@ \
		$(WAYBAR_ROOT)/resources/custom_modules/cffi_example/waybar_cffi_module.h \
		-- \
		$$(pkg-config --cflags-only-I gtk+-3.0)

waybar-cffi-sys/WAYBAR_VERSION:
	@if [ -z "$(WAYBAR_ROOT)" ]; then echo "Must provide the Waybar source tree root via the WAYBAR_ROOT environment variable."; exit 1; fi
	$(MESON) introspect $(WAYBAR_ROOT)/meson.build --projectinfo | $(JQ) -j '.version' > $@

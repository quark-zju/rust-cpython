# `bindgen` Output vs Handwritten Bindings

This document compares the current handwritten `python3-sys` bindings with the
`bindgen`-generated file produced by:

```bash
cargo check -p python3-sys --features bindgen
```

Comparison snapshot:

- Local Python: `3.14.3`
- Generated file used for inspection:
  `target/debug/build/python3-sys-e344b26e25676c8e/out/bindings.rs`
- Handwritten entry point:
  [`src/manual_bindings.rs`](../src/manual_bindings.rs)

The generated path is build-output-specific and will change across machines and
build directories. The observations below are about the current local snapshot,
not a promise about every future `bindgen` run.

## Summary

The `bindgen` version is useful as a raw header translation and as an upgrade
discovery tool, but it is not yet a drop-in replacement for the handwritten
surface of this crate.

The largest differences are:

- `bindgen` exposes a much larger, Python-3.14-specific API surface.
- The handwritten bindings include Rust convenience wrappers and initializer
  constants that are not preserved as-is by `bindgen`.
- Several core struct layouts differ because the generated file is tied to the
  exact CPython headers on the local machine, while the handwritten bindings
  intentionally smooth over version differences with `cfg` gates.

## What `bindgen` Adds

The generated file is materially larger than the handwritten API surface.

- `bindgen` snapshot counts:
  `123` public structs, `115` public type aliases, `388` public constants,
  `1150` exported `Py*` functions.
- The handwritten entry point re-exports `55` internal modules and exposes a
  curated subset of Rust-friendly declarations from those files.

Examples of items present in the generated file and not represented in the
current handwritten bindings include:

- Newer type flags such as `Py_TPFLAGS_INLINE_VALUES` and
  `Py_TPFLAGS_MANAGED_DICT` in the generated file.
- Newer helper constants like `Py_CONSTANT_NONE`.
- Newer APIs such as `PyMapping_GetOptionalItem`,
  `PyObject_CallOneArg`, and `PyCode_AddWatcher`.
- Newer runtime structs such as `PyFrameLocalsProxyObject`.

These are real wins for upgrade discovery: the generated file makes it obvious
when newer CPython releases add APIs that this crate does not currently model.

## What the Handwritten Bindings Still Provide

The handwritten bindings contain a compatibility layer that `bindgen` does not
recreate directly.

Examples:

- [`src/object.rs`](../src/object.rs) defines `PyObject_HEAD_INIT` and inline
  access helpers `Py_REFCNT`, `Py_TYPE`, and `Py_SIZE`.
- [`src/moduleobject.rs`](../src/moduleobject.rs) defines
  `PyModuleDef_HEAD_INIT`, plus helper predicates like `PyModule_Check` and
  `PyModule_CheckExact`.
- [`src/pycapsule.rs`](../src/pycapsule.rs) defines `PyCapsule_CheckExact`.
- [`src/bytearrayobject.rs`](../src/bytearrayobject.rs) defines
  `PyByteArray_Check` and `PyByteArray_CheckExact`.

Some of these helpers are wrapper logic around fields or macros rather than raw
linkable C symbols. `bindgen` preserves exported declarations from the headers,
but it does not automatically recreate this crate's Rust-side convenience API.

In practice, this means the `bindgen` feature currently changes the public Rust
surface substantially, even when the underlying C API overlaps.

## Layout Differences

### `PyObject`

The handwritten `PyObject` in [`src/object.rs`](../src/object.rs) assumes the
traditional layout:

- optional `Py_TRACE_REFS` links
- `ob_refcnt`
- `ob_type`

The generated 3.14 snapshot models `_object` differently:

- a union containing `ob_refcnt_full`
- split fields `ob_refcnt`, `ob_overflow`, and `ob_flags`
- then `ob_type`

That is an important signal: generated bindings track the current CPython
headers exactly, while the handwritten bindings intentionally model a narrower
cross-version contract.

### `PyTypeObject`

The handwritten bindings make `PyTypeObject` opaque under `Py_LIMITED_API`, and
otherwise define a version-gated Rust layout in
[`src/object.rs`](../src/object.rs).

The generated snapshot emits the concrete local 3.14 `_typeobject`, including
fields such as:

- `tp_watched`
- `tp_versions_used`

This is accurate for the local headers, but it also means the generated API is
much more tightly coupled to one CPython release line.

### `PyConfig`

The handwritten `PyConfig` in [`src/initconfig.rs`](../src/initconfig.rs) is
explicitly version-gated with `#[cfg(Py_3_x)]` fields. It is trying to support
multiple Python minors from one checked-in Rust source.

The generated 3.14 snapshot emits one concrete `PyConfig` layout containing
fields such as:

- `remote_debug`
- `thread_inherit_context`
- `context_aware_warnings`
- `cpu_count`
- `sys_path_0`

This is simpler mechanically, but it only matches the local headers used for
generation.

## Build and API Stability Implications

The current `bindgen` path is tied to:

- the local Python installation
- the local Python header set
- the local platform ABI
- the exact CPython minor version used during the build

That has two consequences:

- The generated Rust API can change when the build machine's Python changes.
- A crate user selecting `features = ["bindgen"]` may get a meaningfully
  different public API than a user building the default handwritten path.

This is fine for experimentation and upgrade work, but it is not yet a stable
substitute for the handwritten bindings.

## Practical Conclusion

The current `bindgen` feature is best understood as:

- a parallel raw-FFI path
- a way to inspect new CPython headers quickly
- a source of truth for spotting missing declarations during upgrades

It is not yet feature-parity with the handwritten bindings. To get closer, the
next step should be a shim layer on top of generated output that restores the
Rust-facing conveniences and compatibility helpers relied on by existing users.

# Cannoli Programming Language
![](https://github.com/joncatanio/cannoli/blob/master/resources/logo/cannoli_logo_212x118.png)

Cannolib provides standard library support for the [Cannoli Programming Language](https://github.com/joncatanio/cannoli). Various types and modules are implemented in Cannolib that offload a substantial amount of work from the compiler.

### Types
The encapsulating type is a Rust enum called `Value` and is defined in [`value.rs`](/src/value.rs). More complex types may
be defined as structs or enums in their own module, these can be located in the [`types`](/src/types) directory.

### Built-in Functions and Modules
Python 3.6.5 has a variety of built-in functions and modules, Cannolib provides these features through the [`builtin`](/src/builtin) module. Cannolib currently only supports a subset of the built-in functions and a portion of the `math` and `sys` libraries.

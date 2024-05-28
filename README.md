# zed-rego

Zed extension for Rego and [Open Policy Agent](https://github.com/open-policy-agent/opa/) (OPA).

**This is work in progress and not yet ready for general use.**

## Status

### Basic Rego Support

Basic features like syntax highlighting works well, with the following exceptions:

- [ ] No automatic indentation in rule bodies, objects, etc

### Regal Language Server Support

This extension uses the [Regal](https://github.com/styrainc/regal) language server for Rego to provide most of its
functionality. The following features of the language server are currently supported:

- [x] Diagnostics (yes, **but does not show links to docs**)
- [x] Hover (inline docs on built-in functions)
- [x] Go to definition (cmd + click on a reference to go to definition)
- [x] Folding ranges (expand/collapse blocks, imports, comments)
- [x] Document and workspace symbols (cmd+t to navigate to rules, functions, packages)
- [x] Inlay hints (yes, but needs to be enabled in settings "Toggle inlay hints")
- [x] Formatting
- [x] Code actions
- [x] Code completion

## Installation

To install the extension in development mode, first make sure to have [Rust](https://www.rust-lang.org/tools/install) installed.

Next, clone this repository. Now from the Zed "Extensions" menu, choose "Install Dev Extension" and select the
directory where you cloned this repository.

## Depends on

- https://github.com/FallenAngel97/tree-sitter-rego
- https://github.com/StyraInc/regal

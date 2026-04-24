## Context

The benchmark report now includes stable median-oriented metrics. The next
measurable hot path is rule metadata construction. `options_from_config`,
`RuleCatalog::build`, config validation, and performance benchmarks repeatedly
call owned rule factory methods that allocate `Vec<Box<dyn MarkdownRule>>`.

The actual rule objects are zero-sized structs or immutable rule implementations.
Benchmarking showed that changing lint dispatch to cached trait-object or
function-pointer registries can be slower than the current owned dispatch. This
cache targets metadata and catalog paths, while lint evaluation preserves the
fastest measured dispatch.

## Goals / Non-Goals

### Goals

This design removes repeated rule metadata allocation from CLI config
validation, rule catalog construction, and benchmark config validation. It keeps
existing public methods that return owned vectors so downstream callers do not
break.

### Non-Goals

This change does not rewrite rule evaluation into a single pass. It does not
force lint evaluation through a cached runtime registry when benchmarks show a
regression. It does not parallelize per-file CLI processing. It does not change
lint diagnostics, fix output, CLI exit codes, or JSON output shape.

## Decisions

### Decision: add cached metadata registries next to owned compatibility APIs

`MarkdownLinterOps` will expose cached metadata slices for official rules and
user-configurable rules. Existing `get_official_rules()` and
`get_user_configurable_rules()` will remain as owned compatibility APIs.

Alternative considered: change the public APIs to borrowed slices directly.
That would be faster but risks breaking downstream code, so it is out of scope.

### Decision: keep lint dispatch on the fastest measured path

`evaluate_all` will keep using the existing owned rule vector path because the
rule structs are zero-sized and measured cached-dispatch variants regressed
large-document lint. This keeps runtime behavior stable while still caching
metadata paths that showed measurable gains.

### Decision: cache `RuleCatalog::build`

`RuleCatalog::build()` will return a clone of a cached catalog. This keeps the
public API owned and mutation-safe while avoiding repeated rule metadata
reconstruction.

### Decision: add cached config validation

`MarkdownLintConfig` will keep `validate(&[Box<dyn MarkdownRule>])` for
compatibility and add an internal cached-rule validation path. CLI and benchmark
code will use the cached path.

## Risks / Trade-offs

Adding `Sync` to `MarkdownRule` constrains future rules to be thread-safe. That
is acceptable because cached global metadata registries and CLI parallelization
require thread-safe immutable rule definitions.

Cached metadata registries are global for the process lifetime. That is
acceptable because rule metadata is static and configuration stays outside rule
instances.

## Migration Plan

Add cached metadata registry accessors and switch internal metadata hot paths to
them. Keep owned compatibility APIs intact. Refresh the performance baseline and
verify behavior with existing tests, dogfood, and upstream golden comparison.

## Open Questions

None.

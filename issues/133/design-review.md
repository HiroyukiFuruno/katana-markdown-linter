# v0.20.1 Shared Boundary Review

The four audit findings are addressed at their shared authorities: inline source
interpretation, common configuration metadata, formatter layout policy, and
manifest identity. The existing FixEngine remains the only iterative repair
engine for library, CLI, and formatter entrypoints.

## Additional findings addressed before publication

- Valid emphasis candidates now use cached backend source ranges. Shared
  CommonMark flanking checks protect literal intraword markers and Unicode
  boundaries. Ambiguous replacements compare typed parser events before being
  offered as safe fixes.
- External review identified a reference-binding gap in the initial closed-frame
  shortcut and quadratic full-document parsing for repeated nested styles. The
  shortcut that skipped meaning comparison was removed. Shared source scopes and
  original event boundaries now select local comparisons only where the original
  subtree and isolated delimiter boundaries agree; reference-dependent or
  ambiguous contexts retain full-document validation.
- Reference regression includes shortcut/collapsed links and images, previously
  unresolved labels that would gain a new binding, and positive explicit/inline
  link and image repairs. Expected unsafe diagnostics are matched exactly by rule
  and count in the eighth matrix case.
- A later batch review found that individually safe style changes can jointly
  activate unresolved references. The shared application plan now validates the
  combined edits against the intentional-repair baseline. A failed combined
  comparison splits rejected groups to select a deterministic safe subset;
  independent edits still batch. A real-parser regression bounds comparisons to
  15 for 128 candidates with one conflicting reference pair.
  Policy and semantic signatures each have one private authority, also consumed by
  the single-pass API.
- File and stdin formatter convergence failures use the existing structured rule
  error contract. A failed file remains unchanged and other batch files continue.
- Parallel formatter tests use process, nonce, and atomic identity to avoid
  accidental configuration sharing.
- Shared link escape checks run at structural marker candidates, avoiding
  repeated backslash-run scans for every byte.
- Independent harness review found that matrix coverage could silently shrink.
  All seven required IDs are now asserted while additional cases remain allowed.

- Git's local automatic CRLF normalization would have weakened the fixture axis
  after checkout. Fixture-wide byte preservation is now explicit in attributes,
  and matrix line-ending metadata is verified against both input and answer.

- Operation-scoped configuration preparation removes repeated discovery, parsing,
  validation and option conversion. Stateless loading stays fresh, and consecutive
  workflow invocations observe configuration changes. Formatting caches its
  derived layout policy only within the same operation.
- Temporary execution filenames now use a shared ordinal allocator instead of
  logical IDs. Real CLI comparison reproduced case-fold overwrites in the old
  runner and independent exact answers in the revised runner.

## Independent evidence and limits

The original 258 manifest records and their 516 fixture files are unchanged.
Twelve independently reviewed originals bring the corpus to 270 documents.
Eight configuration/structure cases compare explicit expected outputs across
library, file, and stdin entrypoints with three chained passes. A separate public
API regression compares meaning and idempotence for 1,372 delimiter context
combinations. The parser-event oracle shares a backend with production; reviewed
expected bytes, upstream comparison, and literal guard fixtures provide additional
independent checks. These bounded checks do not claim exhaustive Markdown
conformance.

An exploratory 2,620-transformation comparison found no mismatch but missed the
reference binding issue found by external review. Those measurements were not
accepted as a proof. The revised design always compares meaning and adds
reference-specific independent answers plus deterministic scope-selection checks
for repeated nested frames with unrelated global definitions.

Performance baseline, strict ratio 1.4, coverage baseline, and all existing release
gates are retained. Final results and publication references are recorded in
`verification-summary.json` after each gate is collected.

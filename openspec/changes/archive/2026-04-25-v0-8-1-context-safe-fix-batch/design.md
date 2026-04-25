# Design

## MD048 Evaluation

`MD048` must evaluate `DocumentContext::code_blocks()` instead of scanning all
lines for marker strings. The expected style is resolved as follows:

- `style = "backtick"` requires all fenced code blocks to use backtick fences.
- `style = "tilde"` requires all fenced code blocks to use tilde fences.
- `style = "consistent"` or absent uses the first fenced code block style as
  the expected style.

Each mismatched block produces a diagnostic whose range covers the block source
range. This makes diagnostics actionable and gives fix application one source
range for the whole block.

## Safe Fix Policy

A fix is safe only when all of the following are true:

- The block has both an opening and a closing fence line.
- The opening and closing fence lines use the parsed block fence kind.
- Rewriting to the target marker cannot terminate the code block early.

The collision guard checks inner lines between the opening and closing fences.
When converting to backtick fences, any inner line starting with a backtick
fence marker blocks the fix. When converting to tilde fences, any inner line
starting with a tilde fence marker blocks the fix.

The replacement preserves:

- Leading indentation.
- Fence marker length.
- Info strings and trailing text on fence lines.
- Inner code content.
- The original trailing newline of the block range.

## Compatibility

The public API shape remains unchanged for `v0.8.1`. `available_rules()` should
mark `MD048` as fixable once a safe subset exists. The CLI continues to apply
safe fixes only.

## ADDED Requirements

### Requirement: MD034 detects ftp, ftps, and mailto bare URLs

MD034 SHALL detect bare URLs with the schemes `ftp://`, `ftps://`, and `mailto:` in addition to the existing `http://` and `https://`.
For each detected bare URL the rule SHALL emit a diagnostic with `fix_info` that wraps the URL in angle brackets (e.g., `ftp://example.com` → `<ftp://example.com>`).
Existing suppression logic (inside inline code spans, inline links, reference definitions, HTML attributes) SHALL apply equally to the new schemes.

#### Scenario: ftp:// bare URL is flagged

- **WHEN** a line contains `ftp://example.com` outside a code span, link, or HTML attribute
- **THEN** MD034 emits a diagnostic for the bare URL
- **THEN** `fix_info.replacement` is `<ftp://example.com>`

#### Scenario: mailto: bare URL is flagged

- **WHEN** a line contains `mailto:user@example.com` outside a code span or link
- **THEN** MD034 emits a diagnostic for the bare URL
- **THEN** `fix_info.replacement` is `<mailto:user@example.com>`

#### Scenario: ftp:// inside inline code span is ignored

- **WHEN** a line contains a backtick-fenced `ftp://example.com` code span
- **THEN** MD034 does not emit a diagnostic for the URL inside the code span

#### Scenario: Already-bracketed ftp:// is ignored

- **WHEN** a line contains `<ftp://example.com>`
- **THEN** MD034 does not emit a diagnostic

### Requirement: MD034 is_ignored_url uses binary search for span lookup

MD034's `is_ignored_url` helper SHALL use `partition_point` to locate relevant `inline_code_spans`, `inline_links`, and `reference_definitions` entries by line index before checking position containment, reducing per-URL check cost from O(n) to O(log n + k) where k is the number of spans on the same line.
The behavioral output of the rule SHALL be identical to the previous linear-scan implementation.

#### Scenario: URL in document with many links is correctly suppressed

- **WHEN** content has 100+ inline links and a bare `http://example.com` URL inside one of them
- **THEN** MD034 emits no diagnostic for the URL inside the link
- **THEN** check result is identical to the O(n) implementation

#### Scenario: Performance regression gate passes for url-heavy document

- **WHEN** `make bench` is run after the optimization
- **THEN** the url-heavy benchmark case does not exceed 1.40× the recorded baseline

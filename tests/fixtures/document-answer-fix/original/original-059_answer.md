# Escaped punctuation in prose

A user guide may discuss Markdown syntax literally, including the escaped marker \* which must remain visible as punctuation, and a link-like example \(draft\)[https://example.invalid] that is not intended to become a link. The examples appear beside ordinary _emphasis_ so scanners must distinguish a quoted delimiter from an active delimiter. Documentation tools should preserve every backslash and bracket when a safe formatting pass handles the surrounding paragraph.

The final paragraph keeps a second escaped marker \\* and a Unicode note about 日本語 so odd and even escape parity can be reviewed in context.

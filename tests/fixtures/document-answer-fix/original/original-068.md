# Link Normalization

The reference sentence contains [ spaced label ](#link-normalization), and the intentional extra spaces inside that link text are the only mutation in this document. A nearby code sample `[literal]` remains code, while the escaped bracket \[quoted\] remains plain punctuation. The hand-reviewed answer removes the link-text padding without changing its destination, label wording, or either literal example.

The surrounding text describes a small release checklist in complete prose so the fix is evaluated with real Markdown context. Unicode labels such as 東京 and naïve remain exact, and a final sentence confirms that unrelated brackets and code spans are preserved by the document-level pass.

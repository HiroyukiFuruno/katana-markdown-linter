# Dependency update evidence

upstream release: `pulldown-cmark` 0.13.6, `rmcp` 3.9.0, and
`taiki-e/install-action` 2.90.1. VS Code development dependencies were updated
within their supported runtime. The development glob matcher resolves to
`minimatch` 11.0.5 with the published security fix.

api migration note: The Markdown and MCP feature sets remain compatible.
TypeScript and the editor language client retain compatible versions.
The Python package has no runtime dependencies and builds with
`setuptools` 84.2.1. Zed and the refactoring utility were checked separately.

verification evidence: The pre-release implementation passed 272 document
answers, 20 strict performance cases, 801 uncovered lines against the
881-line limit, npm audit with zero findings, VS Code 1.75 activation,
MCP stdio and remote smoke tests, and actionlint with ShellCheck.
The version-synchronized release is checked separately before merge.

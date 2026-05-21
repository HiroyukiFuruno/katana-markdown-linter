#!/usr/bin/env node
const { KmlLauncher } = require("../lib/launcher");

process.exit(new KmlLauncher("kml-mcp-remote", process.argv.slice(2)).run());

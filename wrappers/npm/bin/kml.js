#!/usr/bin/env node
const { spawnSync } = require("node:child_process");
const { KmlInstaller } = require("../lib/installer");

const installer = new KmlInstaller();
const binaryPath = installer.ensureBinary();
const result = spawnSync(binaryPath, process.argv.slice(2), { stdio: "inherit" });

if (result.error) {
  throw result.error;
}

process.exit(result.status === null ? 1 : result.status);

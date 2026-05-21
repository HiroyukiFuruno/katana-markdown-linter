#!/usr/bin/env node
const { dispatchPackageCommand } = require("../lib/launcher");

process.exit(dispatchPackageCommand(process.argv.slice(2)));

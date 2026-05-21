const { spawnSync } = require("node:child_process");
const { KmlInstaller } = require("./installer");

class KmlLauncher {
  constructor(binaryRole, args) {
    this.binaryRole = binaryRole;
    this.args = args;
  }

  run() {
    const installer = new KmlInstaller(this.binaryRole);
    const binaryPath = installer.ensureBinary();
    const result = spawnSync(binaryPath, this.args, { stdio: "inherit" });
    if (result.error) {
      throw result.error;
    }
    return result.status === null ? 1 : result.status;
  }
}

function dispatchPackageCommand(args) {
  const [firstArg, ...remainingArgs] = args;
  const roles = new Set(["kml", "kml-mcp", "kml-mcp-remote"]);
  if (roles.has(firstArg)) {
    return new KmlLauncher(firstArg, remainingArgs).run();
  }
  return new KmlLauncher("kml", args).run();
}

module.exports = { KmlLauncher, dispatchPackageCommand };

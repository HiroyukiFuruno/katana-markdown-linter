#!/usr/bin/env node
const { spawnSync } = require("node:child_process");
const fs = require("node:fs");
const path = require("node:path");

class NpmPublishTargetVerifier {
  constructor(packageRoot) {
    this.packageRoot = path.resolve(packageRoot);
    this.packageJson = JSON.parse(
      fs.readFileSync(path.join(this.packageRoot, "package.json"), "utf8")
    );
    this.packageSpec = `${this.packageJson.name}@${this.packageJson.version}`;
  }

  run() {
    this.verifyPackageMetadata();
    const viewResult = this.viewPackageVersion();
    this.verifyPublishable(viewResult);
    console.log(`npm publish target is available for ${this.packageSpec}`);
  }

  verifyPackageMetadata() {
    for (const field of ["name", "version"]) {
      if (typeof this.packageJson[field] !== "string" || this.packageJson[field].trim() === "") {
        throw new Error(`package.json: ${field} must be a non-empty string`);
      }
    }
  }

  viewPackageVersion() {
    return spawnSync("npm", ["view", this.packageSpec, "version"], {
      encoding: "utf8"
    });
  }

  verifyPublishable(viewResult) {
    if (viewResult.error) {
      throw new Error(
        `npm command failed while checking ${this.packageSpec}: ${viewResult.error.message}`
      );
    }
    const stdout = (viewResult.stdout || "").trim();
    const stderr = (viewResult.stderr || "").trim();
    const combinedOutput = `${stdout}\n${stderr}`;

    if (viewResult.status === 0 && stdout === this.packageJson.version) {
      throw new Error(`${this.packageSpec} is already published on npm.`);
    }
    if (combinedOutput.includes("Unpublished on")) {
      throw new Error(
        `${this.packageSpec} is blocked by npm unpublish state. Wait for npm's republish window before retrying.`
      );
    }
    if (this.isNotFound(viewResult, combinedOutput)) {
      return;
    }
    throw new Error(
      `Could not verify npm publish target for ${this.packageSpec}.\nstdout:\n${stdout}\nstderr:\n${stderr}`
    );
  }

  isNotFound(viewResult, combinedOutput) {
    return (
      viewResult.status !== 0 &&
      (combinedOutput.includes("E404") ||
        combinedOutput.includes("404 Not Found") ||
        combinedOutput.includes("not found"))
    );
  }
}

const packageRoot = process.argv[2] || "wrappers/npm";
try {
  new NpmPublishTargetVerifier(packageRoot).run();
} catch (error) {
  console.error(error.message);
  process.exit(1);
}

#!/usr/bin/env node
const { execFileSync } = require("node:child_process");
const fs = require("node:fs");
const path = require("node:path");

class NpmPackageVerifier {
  constructor(packageRoot) {
    this.packageRoot = path.resolve(packageRoot);
    this.packageJsonPath = path.join(this.packageRoot, "package.json");
    this.packageJson = JSON.parse(fs.readFileSync(this.packageJsonPath, "utf8"));
  }

  run() {
    this.verifyMetadata();
    this.verifyNoRuntimeDependencies();
    this.verifyReadme();
    this.verifyTarballContents();
    console.log(`npm package check passed for ${this.packageJson.name}@${this.packageJson.version}`);
  }

  verifyMetadata() {
    this.requireString("name");
    this.requireString("version");
    this.requireString("description");
    this.requireString("license");
    this.requireString("homepage");
    this.requireObject("repository");
    this.requireObject("bugs");
    this.requireObject("bin");

    if (!Array.isArray(this.packageJson.keywords) || this.packageJson.keywords.length === 0) {
      throw new Error("package.json: keywords must be a non-empty array");
    }
    for (const keyword of this.packageJson.keywords) {
      if (typeof keyword !== "string" || keyword.trim() === "") {
        throw new Error("package.json: keywords must contain only non-empty strings");
      }
    }
    if (this.packageJson.repository.type !== "git" || !this.packageJson.repository.url) {
      throw new Error("package.json: repository must include git type and url");
    }
    if (!this.packageJson.bugs.url) {
      throw new Error("package.json: bugs.url is required");
    }
    const requiredBins = {
      "katana-markdown-linter": "bin/katana-markdown-linter.js",
      "kml": "bin/kml.js",
      "kml-mcp": "bin/kml-mcp.js",
      "kml-mcp-remote": "bin/kml-mcp-remote.js"
    };
    for (const [name, target] of Object.entries(requiredBins)) {
      if (this.packageJson.bin[name] !== target) {
        throw new Error(`package.json: bin.${name} must point to ${target}`);
      }
    }
  }

  verifyNoRuntimeDependencies() {
    const dependencies = this.packageJson.dependencies || {};
    const names = Object.keys(dependencies);
    if (names.length > 0) {
      throw new Error(`package.json: runtime dependencies must stay empty: ${names.join(", ")}`);
    }
  }

  verifyReadme() {
    const readme = fs.readFileSync(path.join(this.packageRoot, "README.md"), "utf8");
    const requiredFragments = [
      "npx --yes katana-markdown-linter@",
      "bunx --package katana-markdown-linter@",
      "kml-mcp --workspace-root",
      "kml-mcp-remote",
      "thin launcher over GitHub Release binary"
    ];
    for (const fragment of requiredFragments) {
      if (!readme.includes(fragment)) {
        throw new Error(`wrappers/npm/README.md is missing \`${fragment}\``);
      }
    }
  }

  verifyTarballContents() {
    const output = execFileSync("npm", ["pack", "--dry-run", "--json"], {
      cwd: this.packageRoot,
      encoding: "utf8"
    });
    const packages = JSON.parse(output);
    if (!Array.isArray(packages) || packages.length !== 1) {
      throw new Error("npm pack --dry-run --json must return exactly one package");
    }

    const files = new Set(packages[0].files.map((file) => file.path));
    const requiredFiles = [
      "README.md",
      "package.json",
      "bin/katana-markdown-linter.js",
      "bin/kml.js",
      "bin/kml-mcp.js",
      "bin/kml-mcp-remote.js",
      "lib/installer.js",
      "lib/launcher.js"
    ];
    for (const required of requiredFiles) {
      if (!files.has(required)) {
        throw new Error(`npm tarball is missing ${required}`);
      }
    }
  }

  requireString(field) {
    if (typeof this.packageJson[field] !== "string" || this.packageJson[field].trim() === "") {
      throw new Error(`package.json: ${field} must be a non-empty string`);
    }
  }

  requireObject(field) {
    const value = this.packageJson[field];
    if (!value || typeof value !== "object" || Array.isArray(value)) {
      throw new Error(`package.json: ${field} must be an object`);
    }
  }
}

const packageRoot = process.argv[2] || "wrappers/npm";
new NpmPackageVerifier(packageRoot).run();

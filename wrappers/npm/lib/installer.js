const { execFileSync } = require("node:child_process");
const { createHash } = require("node:crypto");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

class KmlInstaller {
  constructor() {
    this.packageRoot = path.resolve(__dirname, "..");
    this.packageJson = require(path.join(this.packageRoot, "package.json"));
    this.version = process.env.KML_WRAPPER_VERSION || `v${this.packageJson.version}`;
    this.target = this.resolveTarget();
    this.archiveName = this.resolveArchiveName();
    this.installRoot = process.env.KML_WRAPPER_INSTALL_DIR || path.join(this.packageRoot, "vendor");
  }

  ensureBinary() {
    const binaryPath = path.join(this.installRoot, "bin", this.binaryName());
    if (fs.existsSync(binaryPath)) {
      return binaryPath;
    }
    const workRoot = fs.mkdtempSync(path.join(os.tmpdir(), "kml-npm-wrapper-"));
    const archivePath = this.prepareArchive(workRoot);
    this.verifyChecksum(archivePath);
    this.extractArchive(archivePath, workRoot);
    const extractedBinary = this.findExtractedBinary(workRoot);
    fs.mkdirSync(path.dirname(binaryPath), { recursive: true });
    fs.copyFileSync(extractedBinary, binaryPath);
    fs.chmodSync(binaryPath, 0o755);
    return binaryPath;
  }

  prepareArchive(workRoot) {
    const localDir = process.env.KML_WRAPPER_ARCHIVE_DIR;
    if (localDir) {
      return path.join(localDir, this.archiveName);
    }
    const archivePath = path.join(workRoot, this.archiveName);
    this.download(this.archiveUrl(this.archiveName), archivePath);
    this.download(this.archiveUrl(`${this.archiveName}.sha256`), `${archivePath}.sha256`);
    return archivePath;
  }

  verifyChecksum(archivePath) {
    const checksumPath = `${archivePath}.sha256`;
    const expected = fs.readFileSync(checksumPath, "utf8").trim().split(/\s+/)[0];
    const actual = createHash("sha256").update(fs.readFileSync(archivePath)).digest("hex");
    if (actual !== expected) {
      throw new Error(`Checksum mismatch for ${this.archiveName}`);
    }
  }

  extractArchive(archivePath, workRoot) {
    const extractRoot = path.join(workRoot, "extract");
    fs.mkdirSync(extractRoot, { recursive: true });
    execFileSync("tar", ["-xf", archivePath, "-C", extractRoot], { stdio: "inherit" });
  }

  findExtractedBinary(workRoot) {
    const stack = [path.join(workRoot, "extract")];
    while (stack.length > 0) {
      const current = stack.pop();
      for (const entry of fs.readdirSync(current, { withFileTypes: true })) {
        const entryPath = path.join(current, entry.name);
        if (entry.isDirectory()) {
          stack.push(entryPath);
        } else if (entry.name === this.binaryName()) {
          return entryPath;
        }
      }
    }
    throw new Error(`Archive does not contain ${this.binaryName()}`);
  }

  download(url, outputPath) {
    execFileSync("curl", ["-fsSL", url, "-o", outputPath], { stdio: "inherit" });
  }

  archiveUrl(name) {
    const repo = "HiroyukiFuruno/katana-markdown-linter";
    return `https://github.com/${repo}/releases/download/${this.version}/${name}`;
  }

  resolveArchiveName() {
    const suffix = this.target.includes("windows") ? "zip" : "tar.gz";
    return `kml-${this.version}-${this.target}.${suffix}`;
  }

  resolveTarget() {
    const key = `${process.platform}/${process.arch}`;
    const targets = {
      "darwin/arm64": "aarch64-apple-darwin",
      "darwin/x64": "x86_64-apple-darwin",
      "linux/x64": "x86_64-unknown-linux-gnu",
      "win32/x64": "x86_64-pc-windows-msvc"
    };
    if (!targets[key]) {
      throw new Error(`Unsupported platform for kml wrapper: ${key}`);
    }
    return targets[key];
  }

  binaryName() {
    return this.target.includes("windows") ? "kml.exe" : "kml";
  }
}

module.exports = { KmlInstaller };

if (require.main === module) {
  new KmlInstaller().ensureBinary();
}

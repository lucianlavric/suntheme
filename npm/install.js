const https = require("https");
const fs = require("fs");
const path = require("path");
const { execSync } = require("child_process");
const zlib = require("zlib");

const REPO = "lukalavric/suntheme";
const BIN_DIR = path.join(__dirname, "bin");
const BIN_PATH = path.join(BIN_DIR, "suntheme");

function getPlatform() {
  const platform = process.platform;
  const arch = process.arch;

  if (platform === "darwin") {
    return arch === "arm64"
      ? "aarch64-apple-darwin"
      : "x86_64-apple-darwin";
  } else if (platform === "linux") {
    return "x86_64-unknown-linux-gnu";
  } else {
    throw new Error(`Unsupported platform: ${platform}-${arch}`);
  }
}

function fetch(url) {
  return new Promise((resolve, reject) => {
    https.get(url, { headers: { "User-Agent": "suntheme-npm" } }, (res) => {
      if (res.statusCode === 301 || res.statusCode === 302) {
        return fetch(res.headers.location).then(resolve).catch(reject);
      }
      if (res.statusCode !== 200) {
        reject(new Error(`HTTP ${res.statusCode}`));
        return;
      }
      const chunks = [];
      res.on("data", (chunk) => chunks.push(chunk));
      res.on("end", () => resolve(Buffer.concat(chunks)));
      res.on("error", reject);
    }).on("error", reject);
  });
}

async function getLatestVersion() {
  const data = await fetch(`https://api.github.com/repos/${REPO}/releases/latest`);
  const release = JSON.parse(data.toString());
  return release.tag_name;
}

async function downloadBinary(version, target) {
  const url = `https://github.com/${REPO}/releases/download/${version}/suntheme-${target}.tar.gz`;
  console.log(`Downloading suntheme ${version} for ${target}...`);

  const data = await fetch(url);

  // Create bin directory
  if (!fs.existsSync(BIN_DIR)) {
    fs.mkdirSync(BIN_DIR, { recursive: true });
  }

  // Extract tar.gz
  const tarPath = path.join(__dirname, "suntheme.tar.gz");
  fs.writeFileSync(tarPath, data);

  try {
    execSync(`tar -xzf "${tarPath}" -C "${BIN_DIR}"`, { stdio: "inherit" });
    fs.unlinkSync(tarPath);
  } catch (e) {
    fs.unlinkSync(tarPath);
    throw e;
  }

  // Make executable
  fs.chmodSync(BIN_PATH, 0o755);

  console.log("suntheme installed successfully!");
}

async function main() {
  try {
    const target = getPlatform();
    const version = await getLatestVersion();
    await downloadBinary(version, target);
  } catch (error) {
    console.error("Failed to install suntheme:", error.message);
    console.error("");
    console.error("You can install manually:");
    console.error("  cargo install suntheme");
    console.error("  # or download from https://github.com/lukalavric/suntheme/releases");
    process.exit(1);
  }
}

main();

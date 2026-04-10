#!/usr/bin/env node
/**
 * Postinstall script to download ghr binary for the current platform
 * Replaces go-npm with better cross-platform support
 */

const https = require('https');
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const packageJson = require('../package.json');
const version = packageJson.version;

// Map Node.js arch to Go arch
const archMap = {
  'x64': 'amd64',
  'amd64': 'amd64',
  'arm64': 'arm64',
  'aarch64': 'arm64',
  'arm': 'arm',
};

// Map Node.js platform to Go platform
const platformMap = {
  'darwin': 'darwin',
  'linux': 'linux',
  'win32': 'windows',
  'windows': 'windows',
};

function getBinaryUrl() {
  const nodeArch = process.arch;
  const nodePlatform = process.platform;

  const arch = archMap[nodeArch];
  const platform = platformMap[nodePlatform];

  if (!arch || !platform) {
    throw new Error(`Unsupported platform: ${nodePlatform}/${nodeArch}`);
  }

  const ext = platform === 'windows' ? 'zip' : 'tar.gz';
  return `https://github.com/syxc/gh-repo-cli/releases/download/v${version}/ghr_${version}_${platform}_${arch}.${ext}`;
}

function downloadFile(url, dest) {
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(dest);
    https.get(url, (response) => {
      if (response.statusCode === 302 || response.statusCode === 301) {
        // Follow redirect
        downloadFile(response.headers.location, dest).then(resolve).catch(reject);
        return;
      }
      if (response.statusCode !== 200) {
        reject(new Error(`Download failed with status ${response.statusCode}`));
        return;
      }
      response.pipe(file);
      file.on('finish', () => {
        file.close();
        resolve();
      });
    }).on('error', reject);
  });
}

function extractArchive(archivePath, destDir) {
  const ext = path.extname(archivePath);

  if (archivePath.endsWith('.tar.gz')) {
    execSync(`tar -xzf "${archivePath}" -C "${destDir}"`, { stdio: 'inherit' });
  } else if (ext === '.zip') {
    execSync(`unzip -o "${archivePath}" -d "${destDir}"`, { stdio: 'inherit' });
  } else {
    throw new Error(`Unknown archive format: ${ext}`);
  }
}

async function main() {
  const binDir = path.join(__dirname, '..', 'bin');
  const binaryName = process.platform === 'win32' ? 'ghr.exe' : 'ghr';
  const binaryPath = path.join(binDir, binaryName);

  // Check if binary already exists
  if (fs.existsSync(binaryPath)) {
    console.log('Binary already exists, skipping download');
    return;
  }

  // Create bin directory
  if (!fs.existsSync(binDir)) {
    fs.mkdirSync(binDir, { recursive: true });
  }

  const url = getBinaryUrl();
  const archiveName = path.basename(url);
  const archivePath = path.join(binDir, archiveName);

  console.log(`Downloading ghr binary from ${url}...`);

  try {
    await downloadFile(url, archivePath);
    console.log('Download complete, extracting...');

    extractArchive(archivePath, binDir);

    // Make binary executable (Unix only)
    if (process.platform !== 'win32') {
      fs.chmodSync(binaryPath, 0o755);
    }

    // Clean up archive
    fs.unlinkSync(archivePath);

    console.log(`Successfully installed ghr to ${binaryPath}`);
  } catch (error) {
    console.error('Installation failed:', error.message);
    process.exit(1);
  }
}

main();

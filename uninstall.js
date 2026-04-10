#!/usr/bin/env node
/**
 * Preuninstall script to remove ghr binary
 */

const fs = require('fs');
const path = require('path');

const binDir = path.join(__dirname, 'bin');
const binaryNames = ['ghr', 'ghr.exe'];

for (const name of binaryNames) {
  const binaryPath = path.join(binDir, name);
  if (fs.existsSync(binaryPath)) {
    try {
      fs.unlinkSync(binaryPath);
      console.log(`Removed ${name}`);
    } catch (err) {
      console.error(`Failed to remove ${name}:`, err.message);
    }
  }
}

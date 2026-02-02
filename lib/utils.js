const fs = require('fs');
const path = require('path');

/**
 * Read file content
 */
function readFileContent(filePath) {
  try {
    return fs.readFileSync(filePath, 'utf-8');
  } catch (error) {
    return null;
  }
}

/**
 * Get file stats
 */
function getFileStats(filePath) {
  try {
    const stats = fs.statSync(filePath);
    return {
      size: stats.size,
      modified: stats.mtime,
      isFile: stats.isFile(),
      isDirectory: stats.isDirectory()
    };
  } catch (error) {
    return null;
  }
}

/**
 * Traverse directory
 */
function traverseDir(dir, maxDepth = 3, currentDepth = 0) {
  if (currentDepth >= maxDepth) return [];

  const items = [];

  try {
    const entries = fs.readdirSync(dir, { withFileTypes: true });

    for (const entry of entries) {
      const fullPath = path.join(dir, entry.name);

      // Skip .git directory and node_modules
      if (entry.name === '.git' || entry.name === 'node_modules') {
        continue;
      }

      const item = {
        name: entry.name,
        path: fullPath,
        type: entry.isDirectory() ? 'directory' : 'file'
      };

      if (entry.isDirectory()) {
        item.children = traverseDir(fullPath, maxDepth, currentDepth + 1);
      }

      items.push(item);
    }
  } catch (error) {
    // Skip directories we can't read
  }

  return items;
}

/**
 * Search files for pattern
 */
function searchFiles(dir, pattern, options = {}) {
  const results = [];
  const regex = new RegExp(pattern, options.ignoreCase ? 'gi' : 'g');

  function searchRecursive(currentDir) {
    try {
      const entries = fs.readdirSync(currentDir, { withFileTypes: true });

      for (const entry of entries) {
        // Skip .git and node_modules
        if (entry.name === '.git' || entry.name === 'node_modules') {
          continue;
        }

        const fullPath = path.join(currentDir, entry.name);

        if (entry.isDirectory()) {
          searchRecursive(fullPath);
        } else if (entry.isFile()) {
          // Filter by extension if specified
          if (options.ext && !entry.name.endsWith(options.ext)) {
            continue;
          }

          const content = readFileContent(fullPath);
          if (content) {
            const lines = content.split('\n');
            lines.forEach((line, index) => {
              if (regex.test(line)) {
                results.push({
                  file: path.relative(dir, fullPath),
                  line: index + 1,
                  text: line.trim(),
                  matches: line.match(regex) || []
                });
              }
            });
          }
        }
      }
    } catch (error) {
      // Skip files/directories we can't read
    }
  }

  searchRecursive(dir);
  return results;
}

/**
 * Format bytes to human readable
 */
function formatBytes(bytes) {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i];
}

/**
 * Save output to file
 */
function saveOutput(content, outputFile) {
  try {
    const dir = path.dirname(outputFile);
    fs.mkdirSync(dir, { recursive: true });
    fs.writeFileSync(outputFile, typeof content === 'string' ? content : JSON.stringify(content, null, 2));
    return true;
  } catch (error) {
    console.error(`Failed to save output: ${error.message}`);
    return false;
  }
}

module.exports = {
  readFileContent,
  getFileStats,
  traverseDir,
  searchFiles,
  formatBytes,
  saveOutput
};

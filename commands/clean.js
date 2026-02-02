const chalk = require('chalk');
const fs = require('fs');
const path = require('path');
const { parseRepo } = require('../lib/git');

/**
 * Clean cached repositories
 */
async function clean(repo, options) {
  try {
    const cacheDir = options.cacheDir;

    if (repo) {
      // Clean specific repository
      const { owner, name } = parseRepo(repo);
      const repoPath = path.join(cacheDir, owner, name);

      if (fs.existsSync(repoPath)) {
        fs.rmSync(repoPath, { recursive: true, force: true });
        console.log(chalk.green(`✅ Cleaned cache for ${repo}`));
      } else {
        console.log(chalk.yellow(`⚠️  No cache found for ${repo}`));
      }
    } else {
      // Clean all cache
      if (options.all) {
        if (fs.existsSync(cacheDir)) {
          const size = getDirectorySize(cacheDir);
          fs.rmSync(cacheDir, { recursive: true, force: true });
          console.log(chalk.green(`✅ Cleaned all cache (${formatBytes(size)})`));
        } else {
          console.log(chalk.yellow('⚠️  No cache found'));
        }
      } else {
        console.log(chalk.yellow('⚠️  Use --all flag to clean all cached repositories'));
        console.log(chalk.gray('   Or specify a repository: ghr clean owner/repo'));
      }
    }

    // Clean output directory if requested
    if (options.output) {
      const outputDir = options.outputDir;
      if (fs.existsSync(outputDir)) {
        fs.rmSync(outputDir, { recursive: true, force: true });
        console.log(chalk.green('✅ Cleaned output directory'));
      }
    }

  } catch (error) {
    console.error(chalk.red(`\n❌ Error: ${error.message}`));
    process.exit(1);
  }
}

/**
 * Get directory size recursively
 */
function getDirectorySize(dirPath) {
  let size = 0;

  function calculateSize(filePath) {
    const stats = fs.statSync(filePath);
    if (stats.isDirectory()) {
      const files = fs.readdirSync(filePath);
      files.forEach(file => {
        calculateSize(path.join(filePath, file));
      });
    } else {
      size += stats.size;
    }
  }

  if (fs.existsSync(dirPath)) {
    calculateSize(dirPath);
  }

  return size;
}

/**
 * Format bytes to human readable
 */
function formatBytes(bytes) {
  if (bytes === 0) {return '0 B';}
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i];
}

module.exports = { clean };

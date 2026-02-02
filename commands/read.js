const chalk = require('chalk');
const { cloneRepo } = require('../lib/git');
const { readFileContent, traverseDir } = require('../lib/utils');
const path = require('path');

async function readFile(repo, filePath, options) {
  try {
    // Clone repository
    const repoPath = await cloneRepo(repo, options.cacheDir, options.proxy);

    const fullPath = path.join(repoPath, filePath);

    // Check if file exists
    const content = readFileContent(fullPath);

    if (content === null) {
      console.error(chalk.red(`\n❌ File not found: ${filePath}\n`));

      // Try to suggest similar files
      console.log(chalk.gray('Available files in this directory:\n'));
      const parentDir = path.join(repoPath, path.dirname(filePath));

      try {
        const items = traverseDir(parentDir, 1);
        items.forEach(item => {
          console.log(`  ${item.name}`);
        });
      } catch (e) {
        // Ignore
      }

      return;
    }

    // Display file content
    console.log(chalk.blue(`\n📄 ${filePath}\n`));
    console.log(chalk.gray('─'.repeat(80)));
    console.log(content);
    console.log(chalk.gray('─'.repeat(80)));

  } catch (error) {
    console.error(chalk.red(`\n❌ Error: ${error.message}`));
    process.exit(1);
  }
}

async function listFiles(repo, filePath, options) {
  try {
    console.log(chalk.blue(`\n📂 Listing ${repo}:${filePath}\n`));

    // Clone repository
    const repoPath = await cloneRepo(repo, options.cacheDir, options.proxy);

    const fullPath = path.join(repoPath, filePath);
    const items = traverseDir(fullPath, parseInt(options.depth) || 1);

    // Display items
    if (items.length === 0) {
      console.log(chalk.gray('(empty directory)\n'));
      return;
    }

    const table = [];
    items.forEach(item => {
      const icon = item.type === 'directory' ? '📁' : '📄';
      table.push(`${icon} ${item.name}`);
    });

    table.forEach(item => console.log(`  ${item}`));
    console.log('');

  } catch (error) {
    console.error(chalk.red(`\n❌ Error: ${error.message}`));
    process.exit(1);
  }
}

module.exports = { readFile, listFiles };

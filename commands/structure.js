const chalk = require('chalk');
const { cloneRepo } = require('../lib/git');
const { traverseDir, saveOutput } = require('../lib/utils');

async function structure(repo, options) {
  try {
    console.log(chalk.blue(`\n🌳 Getting structure of ${repo}...\n`));

    // Clone repository
    const repoPath = await cloneRepo(repo, options.cacheDir, options.proxy);

    // Get structure
    const depth = parseInt(options.depth) || 3;
    const tree = traverseDir(repoPath, depth);

    // Display tree
    displayTree(tree);

    // Save if requested
    if (options.output) {
      saveOutput(tree, options.output);
      console.log(chalk.green(`\n✅ Structure saved to ${options.output}`));
    }

  } catch (error) {
    console.error(chalk.red(`\n❌ Error: ${error.message}`));
    process.exit(1);
  }
}

function displayTree(items, prefix = '') {
  for (let i = 0; i < items.length; i++) {
    const item = items[i];
    const isLastItem = i === items.length - 1;
    const connector = isLastItem ? '└─ ' : '├─ ';
    const icon = item.type === 'directory' ? '📁' : '📄';

    console.log(`${prefix}${connector}${icon} ${item.name}`);

    if (item.children && item.children.length > 0) {
      const newPrefix = prefix + (isLastItem ? '   ' : '│  ');
      displayTree(item.children, newPrefix);
    }
  }
}

module.exports = { structure };

const chalk = require('chalk');
const { cloneRepo } = require('../lib/git');
const { readFileContent } = require('../lib/utils');
const path = require('path');

async function readme(repo, options) {
  try {
    console.log(chalk.blue(`\n📖 Getting README from ${repo}...\n`));

    // Clone repository
    const repoPath = await cloneRepo(repo, options.cacheDir, options.proxy);

    // Try different README filenames
    const readmeNames = [
      'README.md',
      'readme.md',
      'README.MD',
      'README.markdown',
      'README.txt',
      'README'
    ];

    let readmeContent = null;
    let foundName = null;

    for (const name of readmeNames) {
      const content = readFileContent(path.join(repoPath, name));
      if (content !== null) {
        readmeContent = content;
        foundName = name;
        break;
      }
    }

    // Also check in docs/ folder
    if (!readmeContent) {
      for (const name of readmeNames) {
        const content = readFileContent(path.join(repoPath, 'docs', name));
        if (content !== null) {
          readmeContent = content;
          foundName = `docs/${name}`;
          break;
        }
      }
    }

    if (!readmeContent) {
      console.log(chalk.yellow('No README found.\n'));
      return;
    }

    // Display README
    console.log(chalk.bold(`${foundName}\n`));
    console.log(chalk.gray('─'.repeat(80)));
    console.log(readmeContent);
    console.log(chalk.gray('─'.repeat(80)));

  } catch (error) {
    console.error(chalk.red(`\n❌ Error: ${error.message}`));
    process.exit(1);
  }
}

module.exports = { readme };

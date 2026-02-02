const chalk = require('chalk');
const Table = require('cli-table3');
const { cloneRepo } = require('../lib/git');
const { searchFiles, saveOutput } = require('../lib/utils');

async function search(repo, query, options) {
  try {
    console.log(chalk.blue(`\n🔍 Searching in ${repo} for: "${query}"\n`));

    // Clone repository
    const repoPath = await cloneRepo(repo, options.cacheDir, options.proxy);

    // Search files
    const results = searchFiles(repoPath, query, options);

    if (results.length === 0) {
      console.log(chalk.yellow('No matches found.\n'));
      return;
    }

    // Display results
    console.log(chalk.green(`Found ${results.length} matches:\n`));

    const table = new Table({
      head: [chalk.cyan('File'), chalk.cyan('Line'), chalk.cyan('Match')],
      colWidths: [40, 8, 80]
    });

    // Limit to 50 results for display
    const displayResults = results.slice(0, 50);
    displayResults.forEach(result => {
      const line = result.text.length > 75 ? result.text.substring(0, 75) + '...' : result.text;
      table.push([result.file, result.line.toString(), line]);
    });

    console.log(table.toString());

    if (results.length > 50) {
      console.log(chalk.gray(`\n... and ${results.length - 50} more matches`));
    }

    // Save full results if requested
    if (options.output) {
      saveOutput(results, options.output);
      console.log(chalk.green(`\n✅ All ${results.length} results saved to ${options.output}`));
    }

  } catch (error) {
    console.error(chalk.red(`\n❌ Error: ${error.message}`));
    process.exit(1);
  }
}

module.exports = { search };

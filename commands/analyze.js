const chalk = require('chalk');
const Table = require('cli-table3');
const { cloneRepo, getRepoInfo } = require('../lib/git');
const { traverseDir, saveOutput } = require('../lib/utils');
const path = require('path');

async function analyze(repo, options) {
  try {
    console.log(chalk.blue(`\n📊 Analyzing ${repo}...\n`));

    // Clone repository
    const repoPath = await cloneRepo(repo, options.cacheDir, options.proxy);

    // Get repo info
    const info = getRepoInfo(repoPath);

    // Get structure
    const structure = traverseDir(repoPath, 2);

    // Count files by type
    const fileTypes = {};
    const languages = {};

    const countFiles = function countFiles(items) {
      for (const item of items) {
        if (item.type === 'file') {
          const ext = path.extname(item.name).toLowerCase() || '(no extension)';
          fileTypes[ext] = (fileTypes[ext] || 0) + 1;

          // Language detection
          const lang = detectLanguage(item.name);
          if (lang) {
            languages[lang] = (languages[lang] || 0) + 1;
          }
        } else if (item.children) {
          countFiles(item.children);
        }
      }
    };

    countFiles(structure);

    // Display results
    console.log(chalk.bold('📁 Repository Info'));
    console.log(`   Name: ${repo}`);
    if (info) {
      console.log(`   Last Update: ${info.date}`);
      console.log(`   Commit: ${info.commit.substring(0, 7)}`);
    }

    console.log(chalk.bold('\n💻 Top Languages'));
    const sortedLangs = Object.entries(languages)
      .sort((a, b) => b[1] - a[1])
      .slice(0, 10);

    const langTable = new Table({
      head: [chalk.cyan('Language'), chalk.cyan('Files')],
      colWidths: [30, 15]
    });

    sortedLangs.forEach(([lang, count]) => {
      langTable.push([lang, count.toString()]);
    });
    console.log(langTable.toString());

    console.log(chalk.bold('\n📄 File Types'));
    const sortedTypes = Object.entries(fileTypes)
      .sort((a, b) => b[1] - a[1])
      .slice(0, 10);

    const typeTable = new Table({
      head: [chalk.cyan('Extension'), chalk.cyan('Count')],
      colWidths: [30, 15]
    });

    sortedTypes.forEach(([ext, count]) => {
      typeTable.push([ext, count.toString()]);
    });
    console.log(typeTable.toString());

    // Display structure
    console.log(chalk.bold('\n🌳 Directory Structure (depth=2)'));
    displayTree(structure, '', true);

    // Save output if requested
    if (options.output) {
      const output = {
        repo,
        info,
        languages: sortedLangs,
        fileTypes: sortedTypes,
        structure
      };
      saveOutput(output, options.output);
      console.log(chalk.green(`\n✅ Output saved to ${options.output}`));
    }

  } catch (error) {
    console.error(chalk.red(`\n❌ Error: ${error.message}`));
    process.exit(1);
  }
}

function detectLanguage(filename) {
  const ext = path.extname(filename).toLowerCase();
  const langMap = {
    '.js': 'JavaScript',
    '.ts': 'TypeScript',
    '.jsx': 'JavaScript React',
    '.tsx': 'TypeScript React',
    '.py': 'Python',
    '.java': 'Java',
    '.cpp': 'C++',
    '.c': 'C',
    '.cs': 'C#',
    '.go': 'Go',
    '.rs': 'Rust',
    '.php': 'PHP',
    '.rb': 'Ruby',
    '.swift': 'Swift',
    '.kt': 'Kotlin',
    '.scala': 'Scala',
    '.sh': 'Shell',
    '.bash': 'Bash',
    '.zsh': 'Zsh',
    '.html': 'HTML',
    '.css': 'CSS',
    '.scss': 'SCSS',
    '.less': 'Less',
    '.json': 'JSON',
    '.xml': 'XML',
    '.yaml': 'YAML',
    '.yml': 'YAML',
    '.toml': 'TOML',
    '.md': 'Markdown',
    '.txt': 'Text'
  };
  return langMap[ext] || null;
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
      displayTree(item.children, newPrefix, isLastItem);
    }
  }
}

module.exports = { analyze };

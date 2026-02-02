#!/usr/bin/env node

const { program } = require('commander');
const path = require('path');
const os = require('os');

// Import commands
const { analyze } = require('./commands/analyze');
const { search } = require('./commands/search');
const { structure } = require('./commands/structure');
const { readFile, listFiles } = require('./commands/read');
const { readme } = require('./commands/readme');
const { clean } = require('./commands/clean');

// Configuration
const CONFIG = {
  cacheDir: path.join(os.homedir(), '.gh-cli-cache'),
  outputDir: path.join(os.homedir(), '.gh-cli-output'),
  // Read proxy from environment variables
  proxy: process.env.GH_PROXY || process.env.HTTPS_PROXY || process.env.HTTP_PROXY || null,
  // Read GitHub token from environment (optional, for higher rate limits)
  token: process.env.GH_TOKEN || process.env.GITHUB_TOKEN || null
};

program
  .name('gh')
  .description('CLI tool for analyzing GitHub repositories')
  .version('1.0.0');

// Analyze command
program
  .command('analyze <repo>')
  .description('Perform comprehensive analysis of a GitHub repository')
  .option('-o, --output <file>', 'Save output to file')
  .option('--no-cache', 'Bypass cache and re-clone')
  .action((repo, options) => {
    analyze(repo, { ...options, ...CONFIG });
  });

// Search command
program
  .command('search <repo> <query>')
  .description('Search for code patterns in a repository')
  .option('-e, --ext <ext>', 'Filter by file extension')
  .option('-i, --ignore-case', 'Case insensitive search')
  .option('--no-cache', 'Bypass cache')
  .action((repo, query, options) => {
    search(repo, query, { ...options, ...CONFIG });
  });

// Structure command
program
  .command('structure <repo>')
  .description('Get repository directory structure')
  .option('-d, --depth <number>', 'Maximum depth', '3')
  .option('-o, --output <file>', 'Save to file')
  .option('--no-cache', 'Bypass cache')
  .action((repo, options) => {
    structure(repo, { ...options, ...CONFIG });
  });

// Read command
program
  .command('read <repo> <file>')
  .description('Read a specific file from repository')
  .option('--no-cache', 'Bypass cache')
  .action((repo, file, options) => {
    readFile(repo, file, { ...options, ...CONFIG });
  });

// Ls command (alias for list files)
program
  .command('ls <repo> [path]')
  .description('List files in a directory')
  .option('-d, --depth <number>', 'Maximum depth', '1')
  .option('--no-cache', 'Bypass cache')
  .action((repo, filePath, options) => {
    listFiles(repo, filePath || '.', { ...options, ...CONFIG });
  });

// Readme command
program
  .command('readme <repo>')
  .description('Get repository README')
  .option('--no-cache', 'Bypass cache')
  .action((repo, options) => {
    readme(repo, { ...options, ...CONFIG });
  });

// Clean command
program
  .command('clean [repo]')
  .description('Clean cached repositories')
  .option('-a, --all', 'Clean all cached repositories')
  .option('-o, --output', 'Clean output directory as well')
  .action((repo, options) => {
    clean(repo, { ...options, ...CONFIG });
  });

program.parse();

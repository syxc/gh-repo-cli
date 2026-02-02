const {
  readFileContent,
  getFileStats,
  traverseDir,
  searchFiles,
  formatBytes,
  saveOutput
} = require('../../lib/utils');
const fs = require('fs');
const path = require('path');
const os = require('os');

describe('Utils Module', () => {
  let tempDir;

  beforeEach(() => {
    // Create temporary directory for each test
    tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'gh-test-'));
  });

  afterEach(() => {
    // Cleanup temp directory
    if (fs.existsSync(tempDir)) {
      fs.rmSync(tempDir, { recursive: true, force: true });
    }
  });

  describe('readFileContent', () => {
    test('should read file content correctly', () => {
      const testFile = path.join(tempDir, 'test.txt');
      fs.writeFileSync(testFile, 'Hello, World!');
      const content = readFileContent(testFile);
      expect(content).toBe('Hello, World!');
    });

    test('should return null for non-existent file', () => {
      const content = readFileContent('/non/existent/file.txt');
      expect(content).toBeNull();
    });

    test('should handle empty files', () => {
      const testFile = path.join(tempDir, 'empty.txt');
      fs.writeFileSync(testFile, '');
      const content = readFileContent(testFile);
      expect(content).toBe('');
    });
  });

  describe('getFileStats', () => {
    test('should return correct file stats', () => {
      const testFile = path.join(tempDir, 'test.txt');
      fs.writeFileSync(testFile, 'Hello');
      const stats = getFileStats(testFile);

      expect(stats).toBeDefined();
      expect(stats.size).toBe(5);
      expect(stats.isFile).toBe(true);
      expect(stats.isDirectory).toBe(false);
      expect(stats.modified).toBeInstanceOf(Date);
    });

    test('should return null for non-existent file', () => {
      const stats = getFileStats('/non/existent/file.txt');
      expect(stats).toBeNull();
    });

    test('should identify directories correctly', () => {
      const testDir = path.join(tempDir, 'testdir');
      fs.mkdirSync(testDir);
      const stats = getFileStats(testDir);

      expect(stats.isDirectory).toBe(true);
      expect(stats.isFile).toBe(false);
    });
  });

  describe('traverseDir', () => {
    beforeEach(() => {
      // Create test directory structure
      fs.mkdirSync(path.join(tempDir, 'src'));
      fs.mkdirSync(path.join(tempDir, 'src', 'components'));
      fs.writeFileSync(path.join(tempDir, 'index.js'), 'console.log("index");');
      fs.writeFileSync(path.join(tempDir, 'src', 'app.js'), 'console.log("app");');
      fs.writeFileSync(path.join(tempDir, 'src', 'components', 'Button.js'), 'Button component');
    });

    test('should traverse directory with default depth', () => {
      const result = traverseDir(tempDir, 3);
      expect(result).toBeDefined();
      expect(result.length).toBeGreaterThan(0);

      // Should contain both files and directories
      const hasFile = result.some(item => item.type === 'file');
      const hasDir = result.some(item => item.type === 'directory');
      expect(hasFile).toBe(true);
      expect(hasDir).toBe(true);
    });

    test('should skip .git directory', () => {
      fs.mkdirSync(path.join(tempDir, '.git'));
      fs.writeFileSync(path.join(tempDir, '.git', 'config'), 'git config');

      const result = traverseDir(tempDir, 2);
      const gitDir = result.find(item => item.name === '.git');
      expect(gitDir).toBeUndefined();
    });

    test('should skip node_modules directory', () => {
      fs.mkdirSync(path.join(tempDir, 'node_modules'));
      fs.writeFileSync(path.join(tempDir, 'node_modules', 'package.json'), '{}');

      const result = traverseDir(tempDir, 2);
      const nodeModules = result.find(item => item.name === 'node_modules');
      expect(nodeModules).toBeUndefined();
    });

    test('should respect maxDepth parameter', () => {
      const result = traverseDir(tempDir, 1);

      // With depth 1, should not have nested children in directories
      const directories = result.filter(item => item.type === 'directory');
      directories.forEach(dir => {
        if (dir.children) {
          const hasNestedChildren = dir.children.some(child =>
            child.type === 'directory' && child.children && child.children.length > 0
          );
          expect(hasNestedChildren).toBe(false);
        }
      });
    });
  });

  describe('searchFiles', () => {
    beforeEach(() => {
      // Create test files
      fs.writeFileSync(path.join(tempDir, 'test.js'), 'const hello = "world";');
      fs.writeFileSync(path.join(tempDir, 'app.py'), 'print("hello world")');
      fs.writeFileSync(path.join(tempDir, 'README.md'), '# Hello World');
      fs.mkdirSync(path.join(tempDir, 'src'));
      fs.writeFileSync(path.join(tempDir, 'src', 'index.js'), 'function hello() {}');
    });

    test('should search for pattern in all files', () => {
      const results = searchFiles(tempDir, 'hello');
      expect(results.length).toBeGreaterThan(0);

      // Check that results have correct structure
      results.forEach(result => {
        expect(result).toHaveProperty('file');
        expect(result).toHaveProperty('line');
        expect(result).toHaveProperty('text');
        expect(result).toHaveProperty('matches');
      });
    });

    test('should filter by extension', () => {
      const jsResults = searchFiles(tempDir, 'hello', { ext: '.js' });
      const allResults = searchFiles(tempDir, 'hello');

      expect(jsResults.length).toBeLessThan(allResults.length);
      jsResults.forEach(result => {
        expect(result.file).toMatch(/\.js$/);
      });
    });

    test('should support case insensitive search', () => {
      const results = searchFiles(tempDir, 'HELLO', { ignoreCase: true });
      expect(results.length).toBeGreaterThan(0);
    });

    test('should skip .git and node_modules', () => {
      fs.mkdirSync(path.join(tempDir, '.git'));
      fs.mkdirSync(path.join(tempDir, 'node_modules'));
      fs.writeFileSync(path.join(tempDir, '.git', 'hello.txt'), 'hello');
      fs.writeFileSync(path.join(tempDir, 'node_modules', 'hello.js'), 'hello');

      const results = searchFiles(tempDir, 'hello');

      // Should not include files from .git or node_modules
      const hasGitFile = results.some(r => r.file.includes('.git'));
      const hasNodeModulesFile = results.some(r => r.file.includes('node_modules'));

      expect(hasGitFile).toBe(false);
      expect(hasNodeModulesFile).toBe(false);
    });
  });

  describe('formatBytes', () => {
    test('should format bytes correctly', () => {
      expect(formatBytes(0)).toBe('0 B');
      expect(formatBytes(1024)).toBe('1 KB');
      expect(formatBytes(1024 * 1024)).toBe('1 MB');
      expect(formatBytes(1024 * 1024 * 1024)).toBe('1 GB');
    });

    test('should handle fractional values', () => {
      expect(formatBytes(1536)).toBe('1.5 KB');
      expect(formatBytes(2560)).toBe('2.5 KB');
      expect(formatBytes(512)).toBe('512 B'); // Values < 1 KB shown in bytes
    });
  });

  describe('saveOutput', () => {
    test('should save string content to file', () => {
      const outputFile = path.join(tempDir, 'output.txt');
      const result = saveOutput('Hello, World!', outputFile);

      expect(result).toBe(true);
      expect(fs.existsSync(outputFile)).toBe(true);
      expect(fs.readFileSync(outputFile, 'utf-8')).toBe('Hello, World!');
    });

    test('should save JSON content to file', () => {
      const outputFile = path.join(tempDir, 'output.json');
      const data = { key: 'value', number: 123 };
      const result = saveOutput(data, outputFile);

      expect(result).toBe(true);
      expect(fs.existsSync(outputFile)).toBe(true);

      const content = JSON.parse(fs.readFileSync(outputFile, 'utf-8'));
      expect(content).toEqual(data);
    });

    test('should create nested directories if needed', () => {
      const outputFile = path.join(tempDir, 'nested', 'dir', 'output.txt');
      const result = saveOutput('test', outputFile);

      expect(result).toBe(true);
      expect(fs.existsSync(outputFile)).toBe(true);
    });

    test('should return false on error', () => {
      // Try to write to an invalid path (e.g., /root/ on non-root)
      const result = saveOutput('test', '/invalid/path/output.txt');
      expect(result).toBe(false);
    });
  });
});

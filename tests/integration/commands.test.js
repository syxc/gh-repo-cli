const { cloneRepo } = require('../../lib/git');
const { traverseDir, searchFiles } = require('../../lib/utils');
const fs = require('fs');
const path = require('path');
const os = require('os');

// Skip integration tests in CI environment or when git credentials are not available
const describeOrSkip = process.env.CI ? describe.skip : describe;

describeOrSkip('Integration Tests', () => {
  let cacheDir;
  const testRepo = 'pietrodim/php-example'; // Small PHP project for testing

  beforeAll(() => {
    // Create temporary cache directory
    cacheDir = fs.mkdtempSync(path.join(os.tmpdir(), 'gh-cache-'));
  });

  afterAll(() => {
    // Cleanup
    if (fs.existsSync(cacheDir)) {
      fs.rmSync(cacheDir, { recursive: true, force: true });
    }
  });

  describe('Repository Cloning', () => {
    test('should clone a repository successfully', async () => {
      const repoPath = await cloneRepo(testRepo, cacheDir);

      expect(repoPath).toBeDefined();
      expect(fs.existsSync(repoPath)).toBe(true);
      expect(fs.existsSync(path.join(repoPath, '.git'))).toBe(true);
    }, 60000); // 60 second timeout

    test('should re-use cached repository', async () => {
      const repoPath1 = await cloneRepo(testRepo, cacheDir);
      const repoPath2 = await cloneRepo(testRepo, cacheDir);

      expect(repoPath1).toBe(repoPath2);
      expect(fs.existsSync(repoPath2)).toBe(true);
    }, 60000);
  });

  describe('Repository Analysis', () => {
    let repoPath;

    beforeAll(async () => {
      repoPath = await cloneRepo(testRepo, cacheDir);
    }, 60000);

    test('should traverse directory structure', () => {
      const structure = traverseDir(repoPath, 2);

      expect(structure).toBeDefined();
      expect(structure.length).toBeGreaterThan(0);

      // Should have mixed files and directories
      const hasFiles = structure.some(item => item.type === 'file');
      const hasDirs = structure.some(item => item.type === 'directory');

      expect(hasFiles).toBe(true);
      expect(hasDirs).toBe(true);
    });

    test('should search for patterns in repository', () => {
      const results = searchFiles(repoPath, 'function');

      expect(results).toBeDefined();
      expect(Array.isArray(results)).toBe(true);

      // Each result should have required properties
      results.forEach(result => {
        expect(result).toHaveProperty('file');
        expect(result).toHaveProperty('line');
        expect(result).toHaveProperty('text');
      });
    });

    test('should filter search by file extension', () => {
      const phpResults = searchFiles(repoPath, 'class', { ext: '.php' });

      expect(phpResults).toBeDefined();
      phpResults.forEach(result => {
        expect(result.file).toMatch(/\.php$/);
      });
    });
  });
});

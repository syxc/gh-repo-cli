const { parseRepo, buildCloneUrl, getRepoInfo } = require('../../lib/git');
const fs = require('fs');
const path = require('path');
const os = require('os');

describe('Git Module', () => {
  describe('parseRepo', () => {
    test('should parse valid owner/repo format', () => {
      const result = parseRepo('facebook/react');
      expect(result).toEqual({
        owner: 'facebook',
        name: 'react'
      });
    });

    test('should throw error for invalid format - missing owner', () => {
      expect(() => parseRepo('react')).toThrow('Invalid repo format');
    });

    test('should throw error for invalid format - missing name', () => {
      expect(() => parseRepo('facebook/')).toThrow('Invalid repo format');
    });

    test('should throw error for empty string', () => {
      expect(() => parseRepo('')).toThrow('Invalid repo format');
    });

    test('should handle organizations with hyphens', () => {
      const result = parseRepo('vercel/next.js');
      expect(result).toEqual({
        owner: 'vercel',
        name: 'next.js'
      });
    });
  });

  describe('buildCloneUrl', () => {
    test('should build correct HTTPS URL', () => {
      const url = buildCloneUrl('facebook/react');
      expect(url).toBe('https://github.com/facebook/react.git');
    });

    test('should build URL for organization with hyphens', () => {
      const url = buildCloneUrl('some-org/project-name');
      expect(url).toBe('https://github.com/some-org/project-name.git');
    });

    test('should handle proxy parameter (for future use)', () => {
      const url = buildCloneUrl('facebook/react', 'http://proxy:8080');
      expect(url).toBe('https://github.com/facebook/react.git');
    });
  });

  describe('getRepoInfo', () => {
    let tempDir;

    beforeEach(() => {
      // Create temporary directory
      tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'gh-test-'));
    });

    afterEach(() => {
      // Cleanup temp directory
      if (fs.existsSync(tempDir)) {
        fs.rmSync(tempDir, { recursive: true, force: true });
      }
    });

    test('should return null for non-existent directory', () => {
      const result = getRepoInfo('/non/existent/path');
      expect(result).toBeNull();
    });

    test('should return null for directory without .git', () => {
      const result = getRepoInfo(tempDir);
      expect(result).toBeNull();
    });
  });
});

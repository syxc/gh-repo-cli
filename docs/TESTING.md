# Testing Guide

## Overview

This document describes the testing setup for gh-repo-cli.

## Test Structure

```
tests/
├── lib/
│   ├── git.test.js       # Tests for git module
│   └── utils.test.js     # Tests for utility functions
└── integration/
    └── commands.test.js  # Integration tests
```

## Running Tests

### Run all tests
```bash
npm test
```

### Run tests in watch mode
```bash
npm run test:watch
```

### Run tests with coverage
```bash
npm run test:coverage
```

### Run specific test file
```bash
npm test git.test.js
```

### Run tests using the test script
```bash
bash scripts/test.sh
```

## Test Coverage

Current coverage targets:

| Metric   | Target |
|----------|--------|
| Branches | 70%    |
| Functions| 70%    |
| Lines    | 70%    |
| Statements| 70%   |

## Writing Tests

### Unit Tests

Unit tests should:
- Test individual functions in isolation
- Mock external dependencies (file system, network, etc.)
- Be fast and deterministic

Example:
```javascript
describe('parseRepo', () => {
  test('should parse valid owner/repo format', () => {
    const result = parseRepo('facebook/react');
    expect(result).toEqual({
      owner: 'facebook',
      name: 'react'
    });
  });
});
```

### Integration Tests

Integration tests should:
- Test multiple components working together
- Use real Git operations (clone, fetch, etc.)
- Be marked with longer timeouts

Example:
```javascript
describe('Repository Cloning', () => {
  test('should clone a repository successfully', async () => {
    const repoPath = await cloneRepo(testRepo, cacheDir);
    expect(fs.existsSync(repoPath)).toBe(true);
  }, 60000);
});
```

## Linting

### Run linter
```bash
npm run lint
```

### Fix linting issues automatically
```bash
npm run lint:fix
```

## Pre-commit Hook

Before committing, run the pre-commit checks:

```bash
bash scripts/pre-commit.sh
```

Or install it as a Git hook:

```bash
ln -s ../../scripts/pre-commit.sh .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```

## CI/CD

### GitHub Actions

The project uses GitHub Actions for continuous integration:

- **CI**: Runs on every push and pull request
- **Release**: Runs on version tags
- **Code Quality**: Runs weekly and on every push
- **Dependency Update**: Runs weekly

### Local CI Testing

To test CI locally:

```bash
# Install act (https://github.com/nektos/act)
brew install act  # macOS
# or
choco install act-cli  # Windows

# Run CI workflow
act push

# Run specific job
act -j test
```

## Troubleshooting

### Tests timeout

If tests timeout, increase the timeout in the test:

```javascript
test('slow test', async () => {
  // ...
}, 120000); // 2 minutes
```

### Integration tests fail

Integration tests require network access to clone repositories. If they fail:

1. Check your internet connection
2. Check if GitHub is accessible
3. Try a different test repository

### Coverage not generating

Ensure `jest.config.js` is properly configured and run:

```bash
npm run test:coverage
```

# Contributing to gh-repo-cli

Thank you for your interest in contributing to gh-repo-cli!

## How to Contribute

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates.

When creating a bug report, include:
- **Clear title and description**
- **Steps to reproduce** the issue
- **Expected behavior** vs **actual behavior**
- **Environment details** (OS, Node version, etc.)
- **Screenshots** if applicable

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues.

When suggesting enhancements:
- Use a clear and descriptive title
- Provide a detailed explanation of the feature
- Explain why this feature would be useful
- Include examples if possible

### Pull Requests

#### 1. Fork and Clone

```bash
git clone https://github.com/syxc/gh-repo-cli.git
cd gh-repo-cli
npm install
```

#### 2. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-bug-fix
```

#### 3. Make Changes

- Follow the existing code style
- Add tests for new features
- Update documentation as needed
- Run tests and linter

```bash
npm run lint
npm test
```

#### 4. Commit Changes

Use clear commit messages following [Conventional Commits](https://www.conventionalcommits.org/):

```bash
git commit -m "feat: add support for .yml files"
# or
git commit -m "fix: resolve issue with proxy configuration"
# or
git commit -m "docs: update installation instructions"
```

Commit message prefixes:
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `test:` Test changes
- `refactor:` Code refactoring
- `chore:` Maintenance tasks
- `perf:` Performance improvements

#### 5. Push and Create PR

```bash
git push origin feature/your-feature-name
```

Then create a pull request on GitHub with:
- Clear title and description
- Reference related issues (e.g., "Fixes #123")
- Link to any relevant demos

## Development Setup

### Prerequisites

- Node.js >= 14.0.0
- Git
- npm

### Installation

```bash
git clone https://github.com/syxc/gh-repo-cli.git
cd gh-repo-cli
npm install
npm link
```

### Running Tests

```bash
# Run all tests
npm test

# Run with coverage
npm run test:coverage

# Run in watch mode
npm run test:watch

# Run specific test file
npm test -- git.test.js
```

See [docs/TESTING.md](docs/TESTING.md) for detailed testing information.

### Linting

```bash
# Check for issues
npm run lint

# Fix automatically
npm run lint:fix
```

### Pre-commit Checks

Before committing, run the validation script:

```bash
npm run validate
```

Or use the pre-commit script:

```bash
bash scripts/pre-commit.sh
```

## Code Style

- Use 2 spaces for indentation
- Use single quotes for strings
- Add semicolons
- Write meaningful comments
- Keep functions small and focused

### Example

```javascript
// Good
function parseRepo(repo) {
  const [owner, name] = repo.split('/');
  if (!owner || !name) {
    throw new Error(`Invalid repo format: ${repo}`);
  }
  return { owner, name };
}

// Bad
function parseRepo(r){
  var o=r.split('/')
  return {owner:o[0],name:o[1]}
}
```

## Project Structure

```
gh-repo-cli/
├── commands/       # CLI commands
│   ├── analyze.js
│   ├── search.js
│   ├── structure.js
│   ├── read.js
│   ├── readme.js
│   └── clean.js
├── lib/           # Core utilities
│   ├── git.js
│   └── utils.js
├── tests/         # Test files
│   ├── lib/
│   └── integration/
├── scripts/       # Build and utility scripts
├── docs/          # Documentation
├── .github/       # GitHub workflows and templates
│   └── workflows/ # CI/CD configurations
└── index.js       # Main entry point
```

## Testing Guidelines

- Write tests for all new features
- Maintain test coverage above 70%
- Unit tests for individual functions
- Integration tests for complete workflows
- Use meaningful test descriptions

## Documentation

- Update README.md for user-facing changes
- Add JSDoc comments for functions
- Update CHANGELOG.md for releases
- Keep API documentation current

## Getting Help

- Open an issue for bugs or questions
- Check existing documentation
- Review past issues and PRs
- Check [docs/TESTING.md](docs/TESTING.md) for testing help

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

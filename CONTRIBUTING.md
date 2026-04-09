# Contributing to ghr

Thank you for your interest in contributing to ghr!

## How to Contribute

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates.

When creating a bug report, include:
- **Clear title and description**
- **Steps to reproduce** the issue
- **Expected behavior** vs **actual behavior**
- **Environment details** (OS, Go version, etc.)
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
git clone https://github.com/syxc/ghr.git
cd ghr
go mod download
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
- Run tests and linting

```bash
go vet ./...
go test -v ./...
gofmt -s -w .
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

- Go >= 1.23
- Git

### Installation

```bash
git clone https://github.com/syxc/ghr.git
cd ghr
go mod download
go build -o ghr .
```

### Running Tests

```bash
# Run all tests
go test -v ./...

# Run with coverage
go test -v -race -coverprofile=coverage.out ./...

# View coverage report
go tool cover -func=coverage.out
go tool cover -html=coverage.out

# Run specific package tests
go test -v ./internal/git
go test -v ./internal/utils
```

### Linting

```bash
# Run go vet
go vet ./...

# Install and run golint
go install golang.org/x/lint/golint@latest
golint ./...

# Format code
gofmt -s -w .

# Check formatting
if [ "$(gofmt -s -l . | wc -l)" -gt 0 ]; then
  echo "Please run 'gofmt -s -w .' to format your code"
  gofmt -s -l .
fi
```

### Pre-commit Checks

Before committing, run the validation:

```bash
# Format code
gofmt -s -w .

# Run linter
go vet ./...

# Run tests
go test -v ./...

# Check go.mod is tidy
go mod tidy
git diff --exit-code go.mod go.sum
```

## Code Style

- Use `gofmt` for formatting
- Follow standard Go conventions
- Write meaningful comments (exported functions must have doc comments)
- Keep functions small and focused
- Handle errors explicitly
- Use meaningful variable names

### Example

```go
// Good
// ParseRepo parses the owner/repo format and returns the components.
func ParseRepo(repo string) (owner, name string, err error) {
    parts := strings.Split(repo, "/")
    if len(parts) != 2 || parts[0] == "" || parts[1] == "" {
        return "", "", fmt.Errorf("invalid repo format: %s", repo)
    }
    return parts[0], parts[1], nil
}

// Bad
func parseRepo(r string) (string, string, error) {
    p := strings.Split(r, "/")
    return p[0], p[1], nil
}
```

## Project Structure

```
ghr/
├── cmd/                  # CLI commands (Cobra)
│   ├── root.go
│   ├── analyze.go
│   ├── search.go
│   ├── structure.go
│   ├── read.go
│   ├── readme.go
│   └── clean.go
├── internal/             # Internal packages
│   ├── config/          # Configuration management
│   │   └── config.go
│   ├── git/             # Git operations
│   │   ├── git.go
│   │   └── git_test.go
│   └── utils/           # Utility functions
│       ├── file.go
│       ├── output.go
│       ├── search.go
│       └── utils_test.go
├── docs/                 # Documentation
├── scripts/              # Build and utility scripts
├── .github/              # GitHub workflows and templates
│   └── workflows/        # CI/CD configurations
├── go.mod               # Go module definition
├── go.sum               # Go module checksums
├── main.go              # Main entry point
└── README.md            # Project documentation
```

## Testing Guidelines

- Write tests for all new features
- Maintain test coverage above 70%
- Unit tests for individual functions
- Integration tests for complete workflows
- Use table-driven tests where appropriate
- Use meaningful test descriptions

### Example Test

```go
func TestParseRepo(t *testing.T) {
    tests := []struct {
        input    string
        wantOwn  string
        wantName string
        wantErr  bool
    }{
        {"facebook/react", "facebook", "react", false},
        {"invalid", "", "", true},
    }

    for _, tt := range tests {
        t.Run(tt.input, func(t *testing.T) {
            owner, name, err := ParseRepo(tt.input)
            if tt.wantErr {
                if err == nil {
                    t.Errorf("ParseRepo(%q) expected error", tt.input)
                }
                return
            }
            if owner != tt.wantOwn || name != tt.wantName {
                t.Errorf("ParseRepo(%q) = (%q, %q), want (%q, %q)",
                    tt.input, owner, name, tt.wantOwn, tt.wantName)
            }
        })
    }
}
```

## Documentation

- Update README.md for user-facing changes
- Add doc comments for exported functions (required by Go conventions)
- Update CHANGELOG.md for releases
- Keep AI_INTEGRATION.md current if changing AI-related features

## Getting Help

- Open an issue for bugs or questions
- Check existing documentation
- Review past issues and PRs
- Check [docs/TESTING.md](docs/TESTING.md) for testing help

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

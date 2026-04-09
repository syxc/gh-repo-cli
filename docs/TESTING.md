# Testing Guide

## Overview

This document describes the testing setup for ghr (Go rewrite).

## Test Structure

```
internal/
├── git/
│   └── git_test.go       # Tests for git module
└── utils/
    └── utils_test.go     # Tests for utility functions
```

## Running Tests

### Run all tests
```bash
go test -v ./...
```

### Run tests with coverage
```bash
go test -v -race -coverprofile=coverage.out ./...

# View coverage report
go tool cover -func=coverage.out

# Generate HTML coverage report
go tool cover -html=coverage.out -o coverage.html
```

### Run specific package tests
```bash
go test -v ./internal/git
go test -v ./internal/utils
```

### Run specific test function
```bash
go test -v -run TestParseRepo ./internal/git
```

## Test Coverage

Current coverage targets:

| Metric    | Target |
|-----------|--------|
| Packages  | 100%   |
| Functions | 70%    |
| Lines     | 70%    |

## Writing Tests

### Unit Tests

Unit tests should:
- Test individual functions in isolation
- Use table-driven tests for multiple test cases
- Be fast and deterministic
- Not depend on external services

Example:
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

### Integration Tests

Integration tests should:
- Test multiple components working together
- May use real Git operations (clone, fetch, etc.)
- Be marked with longer timeouts if needed
- Be in separate `_test.go` files or use build tags

Example:
```go
func TestCloneRepo_Integration(t *testing.T) {
    if testing.Short() {
        t.Skip("Skipping integration test in short mode")
    }

    cacheDir := t.TempDir()
    repoPath, err := CloneRepo("octocat/Hello-World", cacheDir, "")
    if err != nil {
        t.Fatalf("CloneRepo failed: %v", err)
    }

    if _, err := os.Stat(repoPath); os.IsNotExist(err) {
        t.Errorf("Repository not cloned to %s", repoPath)
    }
}
```

## Linting

### Run go vet
```bash
go vet ./...
```

### Install and run golint
```bash
go install golang.org/x/lint/golint@latest
golint ./...
```

### Format code
```bash
gofmt -s -w .
```

### Check formatting
```bash
if [ "$(gofmt -s -l . | wc -l)" -gt 0 ]; then
  echo "Please run 'gofmt -s -w .' to format your code"
  gofmt -s -l .
fi
```

## Pre-commit Checks

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

## CI/CD

### GitHub Actions

The project uses GitHub Actions for continuous integration:

- **CI**: Runs on every push and pull request
  - Lint checks (go vet, formatting)
  - Unit tests
  - Cross-platform builds
- **Release**: Runs on version tags
- **Code Quality**: Runs weekly and on every push

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

If tests timeout, increase the timeout:

```bash
go test -v -timeout 5m ./...
```

### Integration tests fail

Integration tests require network access to clone repositories. If they fail:

1. Check your internet connection
2. Check if GitHub is accessible
3. Try a different test repository
4. Run with `-short` flag to skip integration tests

### Coverage not generating

Ensure tests are running correctly:

```bash
go test -v -coverprofile=coverage.out ./...
go tool cover -func=coverage.out
```

### Module issues

If you encounter module-related errors:

```bash
# Clean module cache
go clean -modcache

# Re-download dependencies
go mod download

# Verify modules
go mod verify
```

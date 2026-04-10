package utils_test

import (
	"os"
	"path/filepath"
	"testing"

	"github.com/syxc/gh-repo-cli/internal/utils"
)

func TestReadFileContent(t *testing.T) {
	dir := t.TempDir()

	// Normal file
	f := filepath.Join(dir, "test.txt")
	if err := os.WriteFile(f, []byte("Hello, World!"), 0644); err != nil {
		t.Fatal(err)
	}
	content, err := utils.ReadFileContent(f)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if content != "Hello, World!" {
		t.Errorf("got %q, want %q", content, "Hello, World!")
	}

	// Non-existent file
	_, err = utils.ReadFileContent(filepath.Join(dir, "nonexistent"))
	if err == nil {
		t.Error("expected error for non-existent file")
	}

	// Empty file
	emptyF := filepath.Join(dir, "empty.txt")
	if err := os.WriteFile(emptyF, []byte(""), 0644); err != nil {
		t.Fatal(err)
	}
	content, err = utils.ReadFileContent(emptyF)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if content != "" {
		t.Errorf("got %q, want empty string", content)
	}
}

func TestGetFileStats(t *testing.T) {
	dir := t.TempDir()

	// File stats
	f := filepath.Join(dir, "test.txt")
	if err := os.WriteFile(f, []byte("Hello"), 0644); err != nil {
		t.Fatal(err)
	}
	stats, err := utils.GetFileStats(f)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if stats.Size != 5 {
		t.Errorf("size = %d, want 5", stats.Size)
	}
	if !stats.IsFile {
		t.Error("expected IsFile = true")
	}
	if stats.IsDirectory {
		t.Error("expected IsDirectory = false")
	}

	// Directory stats
	stats, err = utils.GetFileStats(dir)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if !stats.IsDirectory {
		t.Error("expected IsDirectory = true")
	}

	// Non-existent
	_, err = utils.GetFileStats(filepath.Join(dir, "nonexistent"))
	if err == nil {
		t.Error("expected error for non-existent path")
	}
}

func TestTraverseDir(t *testing.T) {
	dir := t.TempDir()

	// Create structure: src/components/Button.js + index.js
	os.MkdirAll(filepath.Join(dir, "src", "components"), 0755)
	os.WriteFile(filepath.Join(dir, "index.js"), []byte("console.log('index');"), 0644)
	os.WriteFile(filepath.Join(dir, "src", "app.js"), []byte("console.log('app');"), 0644)
	os.WriteFile(filepath.Join(dir, "src", "components", "Button.js"), []byte("Button"), 0644)

	items, err := utils.TraverseDir(dir, 3, 0)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	hasFile := false
	hasDir := false
	for _, item := range items {
		if item.Type == "file" {
			hasFile = true
		}
		if item.Type == "directory" {
			hasDir = true
		}
	}
	if !hasFile {
		t.Error("expected at least one file")
	}
	if !hasDir {
		t.Error("expected at least one directory")
	}

	// Should skip .git
	os.MkdirAll(filepath.Join(dir, ".git"), 0755)
	os.WriteFile(filepath.Join(dir, ".git", "config"), []byte("git config"), 0644)
	items, _ = utils.TraverseDir(dir, 2, 0)
	for _, item := range items {
		if item.Name == ".git" {
			t.Error(".git should be skipped")
		}
	}

	// Should skip node_modules
	os.MkdirAll(filepath.Join(dir, "node_modules"), 0755)
	os.WriteFile(filepath.Join(dir, "node_modules", "pkg.json"), []byte("{}"), 0644)
	items, _ = utils.TraverseDir(dir, 2, 0)
	for _, item := range items {
		if item.Name == "node_modules" {
			t.Error("node_modules should be skipped")
		}
	}

	// Should respect maxDepth
	items, _ = utils.TraverseDir(dir, 1, 0)
	for _, item := range items {
		if item.Type == "directory" && item.Children != nil {
			for _, child := range item.Children {
				if child.Type == "directory" && len(child.Children) > 0 {
					t.Error("depth 1 should not have nested directory children")
				}
			}
		}
	}
}

func TestSearchFiles(t *testing.T) {
	dir := t.TempDir()

	os.WriteFile(filepath.Join(dir, "test.js"), []byte(`const hello = "world";`), 0644)
	os.WriteFile(filepath.Join(dir, "app.py"), []byte(`print("hello world")`), 0644)
	os.WriteFile(filepath.Join(dir, "README.md"), []byte(`# Hello World`), 0644)
	os.MkdirAll(filepath.Join(dir, "src"), 0755)
	os.WriteFile(filepath.Join(dir, "src", "index.js"), []byte(`function hello() {}`), 0644)

	// Basic search
	results, err := utils.SearchFiles(dir, "hello", utils.SearchOptions{})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(results) == 0 {
		t.Error("expected search results for 'hello'")
	}
	for _, r := range results {
		if r.File == "" || r.Line == 0 || r.Text == "" {
			t.Errorf("result missing fields: %+v", r)
		}
	}

	// Filter by extension
	jsResults, _ := utils.SearchFiles(dir, "hello", utils.SearchOptions{Ext: ".js"})
	allResults, _ := utils.SearchFiles(dir, "hello", utils.SearchOptions{})
	if len(jsResults) >= len(allResults) {
		t.Error("filtered results should be fewer than all results")
	}
	for _, r := range jsResults {
		if filepath.Ext(r.File) != ".js" {
			t.Errorf("filtered result should have .js extension, got %s", r.File)
		}
	}

	// Case insensitive
	results, _ = utils.SearchFiles(dir, "HELLO", utils.SearchOptions{IgnoreCase: true})
	if len(results) == 0 {
		t.Error("case insensitive search should find results")
	}

	// Skip .git and node_modules
	os.MkdirAll(filepath.Join(dir, ".git"), 0755)
	os.WriteFile(filepath.Join(dir, ".git", "hello.txt"), []byte("hello"), 0644)
	os.MkdirAll(filepath.Join(dir, "node_modules"), 0755)
	os.WriteFile(filepath.Join(dir, "node_modules", "hello.js"), []byte("hello"), 0644)
	results, _ = utils.SearchFiles(dir, "hello", utils.SearchOptions{})
	for _, r := range results {
		if filepath.IsLocal(r.File) {
			if stringsContains(r.File, ".git") {
				t.Errorf("should not include .git files: %s", r.File)
			}
			if stringsContains(r.File, "node_modules") {
				t.Errorf("should not include node_modules files: %s", r.File)
			}
		}
	}
}

func stringsContains(s, substr string) bool {
	for i := 0; i <= len(s)-len(substr); i++ {
		if s[i:i+len(substr)] == substr {
			return true
		}
	}
	return false
}

func TestFormatBytes(t *testing.T) {
	tests := []struct {
		input int64
		want  string
	}{
		{0, "0 B"},
		{1024, "1.0 KB"},
		{1536, "1.5 KB"},
		{1024 * 1024, "1.0 MB"},
		{1024 * 1024 * 1024, "1.0 GB"},
		{512, "512 B"},
	}

	for _, tt := range tests {
		got := utils.FormatBytes(tt.input)
		if got != tt.want {
			t.Errorf("FormatBytes(%d) = %q, want %q", tt.input, got, tt.want)
		}
	}
}

func TestSaveOutput(t *testing.T) {
	dir := t.TempDir()

	// String content
	outFile := filepath.Join(dir, "output.txt")
	if err := utils.SaveOutput("Hello, World!", outFile); err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	data, _ := os.ReadFile(outFile)
	if string(data) != "Hello, World!" {
		t.Errorf("got %q, want %q", string(data), "Hello, World!")
	}

	// JSON content
	jsonFile := filepath.Join(dir, "output.json")
	data_in := map[string]interface{}{"key": "value", "number": 123}
	if err := utils.SaveOutput(data_in, jsonFile); err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	data, _ = os.ReadFile(jsonFile)
	if string(data) == "" {
		t.Error("expected non-empty JSON output")
	}

	// Nested directory
	nestedFile := filepath.Join(dir, "nested", "dir", "output.txt")
	if err := utils.SaveOutput("test", nestedFile); err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if _, err := os.Stat(nestedFile); err != nil {
		t.Error("nested file should exist")
	}
}

func TestDetectLanguage(t *testing.T) {
	tests := []struct {
		filename string
		want     string
	}{
		{"main.go", "Go"},
		{"app.js", "JavaScript"},
		{"index.ts", "TypeScript"},
		{"App.tsx", "TypeScript React"},
		{"server.py", "Python"},
		{"README.md", "Markdown"},
		{"config.json", "JSON"},
		{"docker.yaml", "YAML"},
		{".gitignore", ""},
		{"Makefile", ""},
		{"README", ""},
	}

	for _, tt := range tests {
		got := utils.DetectLanguage(tt.filename)
		if got != tt.want {
			t.Errorf("DetectLanguage(%q) = %q, want %q", tt.filename, got, tt.want)
		}
	}
}

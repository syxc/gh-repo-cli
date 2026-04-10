package config

import (
	"os"
	"path/filepath"
	"testing"
)

func TestLoad(t *testing.T) {
	cfg := Load()

	if cfg == nil {
		t.Fatal("Load() returned nil")
	}

	// Check CacheDir is set
	if cfg.CacheDir == "" {
		t.Error("CacheDir should not be empty")
	}

	// Check OutputDir is set
	if cfg.OutputDir == "" {
		t.Error("OutputDir should not be empty")
	}

	// CacheDir should be in home directory
	homeDir, _ := os.UserHomeDir()
	expectedCache := filepath.Join(homeDir, ".ghr-cache")
	if cfg.CacheDir != expectedCache {
		t.Errorf("Expected CacheDir to be %s, got %s", expectedCache, cfg.CacheDir)
	}
}

func TestLoad_WithProxy(t *testing.T) {
	// Set proxy environment variable
	os.Setenv("GH_PROXY", "http://127.0.0.1:7890")
	defer os.Unsetenv("GH_PROXY")

	cfg := Load()

	if cfg.Proxy != "http://127.0.0.1:7890" {
		t.Errorf("Expected Proxy to be http://127.0.0.1:7890, got %s", cfg.Proxy)
	}
}

func TestLoad_WithHTTPSProxy(t *testing.T) {
	// Unset GH_PROXY first
	os.Unsetenv("GH_PROXY")

	// Set HTTPS_PROXY
	os.Setenv("HTTPS_PROXY", "https://proxy.example.com:8080")
	defer os.Unsetenv("HTTPS_PROXY")

	cfg := Load()

	if cfg.Proxy != "https://proxy.example.com:8080" {
		t.Errorf("Expected Proxy to be https://proxy.example.com:8080, got %s", cfg.Proxy)
	}
}

func TestGetEnv(t *testing.T) {
	// Test with existing variable
	os.Setenv("TEST_VAR", "test_value")
	defer os.Unsetenv("TEST_VAR")

	result := getEnv("TEST_VAR", "default")
	if result != "test_value" {
		t.Errorf("Expected 'test_value', got '%s'", result)
	}

	// Test with non-existing variable
	result = getEnv("NON_EXISTENT_VAR", "default")
	if result != "default" {
		t.Errorf("Expected 'default', got '%s'", result)
	}
}

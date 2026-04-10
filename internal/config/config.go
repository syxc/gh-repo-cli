// Package config provides configuration management for ghr
package config

import (
	"os"
	"path/filepath"
)

// Config holds the application configuration
type Config struct {
	CacheDir  string
	OutputDir string
	Proxy     string
}

// Load creates a new Config from environment variables
func Load() *Config {
	homeDir, _ := os.UserHomeDir()

	return &Config{
		CacheDir:  filepath.Join(homeDir, ".ghr-cache"),
		OutputDir: filepath.Join(homeDir, ".ghr-output"),
		Proxy:     getEnv("GH_PROXY", getEnv("HTTPS_PROXY", getEnv("HTTP_PROXY", ""))),
	}
}

func getEnv(key, defaultValue string) string {
	if value := os.Getenv(key); value != "" {
		return value
	}
	return defaultValue
}

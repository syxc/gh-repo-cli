package cmd

import (
	"testing"
)

func TestVersion(t *testing.T) {
	// Version should not be empty
	if Version == "" {
		t.Error("Version should not be empty")
	}
}

func TestGetConfig(t *testing.T) {
	cfg := getConfig()
	if cfg == nil {
		t.Error("getConfig() should not return nil")
	}

	// CacheDir should be set
	if cfg.CacheDir == "" {
		t.Error("CacheDir should not be empty")
	}

	// OutputDir should be set
	if cfg.OutputDir == "" {
		t.Error("OutputDir should not be empty")
	}
}

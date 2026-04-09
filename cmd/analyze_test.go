package cmd

import (
	"testing"
)

func TestSortMapByValue(t *testing.T) {
	input := map[string]int{
		"go":    10,
		"js":    5,
		"python": 15,
		"rust":  3,
	}

	result := sortMapByValue(input)

	// Should be sorted by value descending
	expected := []string{"python", "go", "js", "rust"}

	if len(result) != len(expected) {
		t.Fatalf("Expected %d items, got %d", len(expected), len(result))
	}

	for i, exp := range expected {
		if result[i].Key != exp {
			t.Errorf("Expected key at position %d to be %s, got %s", i, exp, result[i].Key)
		}
	}

	// Check values are correct
	if result[0].Value != 15 {
		t.Errorf("Expected first value to be 15, got %d", result[0].Value)
	}
}

func TestSortMapByValue_Empty(t *testing.T) {
	result := sortMapByValue(map[string]int{})
	if len(result) != 0 {
		t.Error("Empty map should return empty slice")
	}
}

func TestKeyValue(t *testing.T) {
	kv := KeyValue{Key: "test", Value: 42}
	if kv.Key != "test" {
		t.Error("Key mismatch")
	}
	if kv.Value != 42 {
		t.Error("Value mismatch")
	}
}

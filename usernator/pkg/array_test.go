package pkg

import "testing"

func TestContainsString(t *testing.T) {
	needle := "hello"
	haystack := []string{"hello", "world"}
	if !ContainsString(haystack, needle) {
		t.Fatalf("%s should contain %s", haystack, needle)
	}
}

func TestNotContainsString(t *testing.T) {
	needle := "hello"
	haystack := []string{"hello2", "world"}
	if ContainsString(haystack, needle) {
		t.Fatalf("%s should not contain %s", haystack, needle)
	}
}

func BenchmarkContainsString(b *testing.B) {
	needle := "hello"
	haystack := []string{"hello", "world", "this", "is", "a", "test", "some", "more", "strings"}
	for i := 0; i < b.N; i++ {
		ContainsString(haystack, needle)
	}
}

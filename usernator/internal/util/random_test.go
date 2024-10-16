package util

import "testing"

func TestRandomString(t *testing.T) {

	random1 := StringWithCharset(32)
	random2 := StringWithCharset(32)

	if random1 == random2 {
		t.Errorf("Random Strings are the same")
	}
}

func BenchmarkRandomString(b *testing.B) {
	for i := 0; i < b.N; i++ {
		StringWithCharset(32)
	}
}

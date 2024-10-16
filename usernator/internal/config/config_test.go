package config

import "testing"

func TestConfigNotNil(t *testing.T) {
	conf, err := LoadConfiguration("../../config.json")
	if err != nil {
		t.Errorf("Error loading configuration: %v", err)
	}
	if conf == nil {
		t.Errorf("Configuration is nil")
	}
}

func BenchmarkLoadConfiguration(b *testing.B) {
	for i := 0; i < b.N; i++ {
		_, err := LoadConfiguration("../../config.json")
		if err != nil {
			panic(err)
		}
	}
}

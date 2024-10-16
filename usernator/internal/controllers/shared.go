package controllers

import (
	"time"
	"usernator/internal/shared"
)

func waitForTasky() bool {
	count := 0
	for shared.Tasky == nil {
		time.Sleep(1 * time.Second)
		if count >= 5 {
			return false
		}
		count += 1
	}
	return true
}

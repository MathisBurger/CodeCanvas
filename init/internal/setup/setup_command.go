package setup

import (
	"initializer/pkg/prompt"
	"os"
)

func SetupCommand() {

	// Checks if compose file already exists and then deletes it if it should be overwritten
	if _, err := os.Stat("./docker-compose.yml"); err == nil {
		if !prompt.YesNoPrompt("There is already an docker-compose.yml. Do you want to overwrite it?") {
			return
		}
		_ = os.Remove("./docker-compose.yml")
	}

}

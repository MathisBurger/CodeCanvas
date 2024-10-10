package main

import (
	"fmt"
	"github.com/fatih/color"
	"os"
)

func main() {
	argsWithoutProg := os.Args[1:]
	if len(argsWithoutProg) == 0 {
		color.Red("Please enter a valid command like setup, start, stop, upgrade")
		return
	}
	switch argsWithoutProg[0] {
	case "start":
		fmt.Println("Starting application...")
		break
	case "stop":
		fmt.Println("Stopping application...")
		break
	case "upgrade":
		fmt.Println("Upgrading application...")
		break
	case "setup":
		fmt.Println("Setting up application...")
		break
	default:
		color.Red("Invalid command: " + argsWithoutProg[0])
		break
	}
}

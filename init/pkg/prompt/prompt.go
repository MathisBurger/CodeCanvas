package prompt

import (
	"errors"
	"github.com/manifoldco/promptui"
)

// YesNoPrompt creates a yes/no prompt
func YesNoPrompt(prompt string) bool {
	p := promptui.Prompt{
		Label: prompt + " (y/n)",
		Validate: func(input string) error {
			if input != "yes" && input != "y" && input != "no" && input != "n" {
				return errors.New("Please enter a correct value")
			}
			return nil
		},
	}
	result, _ := p.Run()
	if result == "yes" || result == "y" {
		return true
	}
	return false
}

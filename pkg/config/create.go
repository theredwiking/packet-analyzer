package config

import (
	"fmt"
	"os"
)

func CreateConfig() {
	_, err := os.Stat("./config.toml")

	if os.IsNotExist(err) {
		file, err := os.Create("./config.toml")
		if err != nil {
			fmt.Println(err)
			return
		}
		defer file.Close()

		fmt.Fprintf(file, "")
	} else {
		fmt.Println("Config already exist")
		return
	}
}

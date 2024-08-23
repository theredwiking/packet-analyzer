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

		content := fmt.Sprint(`[network]
interface = "<eht0>"
snaplen = <262144>
promiscuous = <false>

[database]
url = "http://<ip>:<port>"
bucket = "<bucket>"
token = "<apikey>"`)
		fmt.Fprintf(file, content)
	} else {
		fmt.Println("Config already exist")
		return
	}
}

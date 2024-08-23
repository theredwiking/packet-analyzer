package config

import (
	"errors"
	"fmt"
	"os"
)

// Creates new config file in directory provide with filename
//
// Example CreateConfig("./config") it auto add .toml
func CreateConfig(filename string) error {
	filename = fmt.Sprintf("%s.toml", filename)
	_, err := os.Stat(filename)

	if os.IsNotExist(err) {
		file, err := os.Create(filename)
		if err != nil {
			return err
		}
		defer file.Close()

		content := fmt.Sprint(`[network]
interface = "<eht0>"
snaplen = <262144>
promiscuous = <false>

[database]
url = "http://<ip>:<port>"
bucket = "<bucket>"
token = "<apikey>"
`)
		fmt.Fprintf(file, content)
	} else {
		return errors.New("File already exist")
	}
	return nil
}

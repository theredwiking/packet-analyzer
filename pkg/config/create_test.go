package config

import (
	"fmt"
	"io"
	"os"
	"testing"

	"github.com/attic-labs/testify/assert"
)

func TestCreateConfig(t *testing.T) {
	filename := "./test"
	assert := assert.New(t)

	err := CreateConfig(filename)

	filename = fmt.Sprintf("%s.toml", filename)

	assert.Nil(err, "Shouldn't return error")

	_, err = os.Stat(filename)
	assert.Nil(err, "Checks if file was create")

	file, err := os.Open(filename)
	assert.Nil(err)
	defer file.Close()

	buf := make([]byte, 150)
	if _, err := io.ReadFull(file, buf); err != nil {
		if err == io.EOF {
			err = io.ErrUnexpectedEOF
		}
	}
	content := fmt.Sprint(`[network]
interface = "<eht0>"
snaplen = <262144>
promiscuous = <false>

[database]
url = "http://<ip>:<port>"
bucket = "<bucket>"
token = "<apikey>"
`)
	assert.Equal(content, string(buf), "Should contain the same content")
	
	err = os.Remove(filename)
	assert.Nil(err, "File failed to delete")
}

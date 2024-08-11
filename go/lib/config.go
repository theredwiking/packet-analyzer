package lib

import (
	"fmt"
	"log"
	"os"

	"github.com/spf13/viper"
	"github.com/theredwiking/packet-analyzer/go/models"
)

// Loads config from toml file
func LoadConfig() *models.Config {
	viper.SetConfigName("config")
	viper.SetConfigType("toml")
	viper.AddConfigPath("$HOME/.config/packet_analyzer/")
	viper.AddConfigPath(".")

	if err := viper.ReadInConfig(); err != nil {
		if _, ok := err.(viper.ConfigFileNotFoundError); ok {
			fmt.Println("No config file in excepted paths")
			os.Exit(1)
		} else {
			log.Fatalln(err)
		}
	}
	return models.NewConfig(viper.GetString("network.interface"), viper.GetInt32("network.snaplen"), viper.GetBool("network.promiscuous"), viper.GetString("database.url"), viper.GetString("database.bucket"), viper.GetString("database.token"))
}

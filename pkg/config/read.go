package config

import (
	"errors"

	"github.com/spf13/viper"
	"github.com/theredwiking/packet-analyzer/go/models"
)

// Loads config from toml file
//
// It looks in $HOME/.config/packet_analyzer or from current directory
func LoadConfig() (*models.Config, error) {
	viper.SetConfigName("config")
	viper.SetConfigType("toml")
	viper.AddConfigPath("$HOME/.config/packet_analyzer/")
	viper.AddConfigPath(".")

	if err := viper.ReadInConfig(); err != nil {
		if _, ok := err.(viper.ConfigFileNotFoundError); ok {
			return nil, errors.New("No config file in excepted paths")
		} else {
			return nil, err
		}
	}
	return models.NewConfig(viper.GetString("network.interface"), viper.GetInt32("network.snaplen"), viper.GetBool("network.promiscuous"), viper.GetString("database.url"), viper.GetString("database.bucket"), viper.GetString("database.token")), nil
}

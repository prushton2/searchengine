package config

import (
	"os"

	"gopkg.in/yaml.v3"
)

type Config struct {
	Backend  BackendConfig  `yaml:"backend"`
	Database PostgresDBInfo `yaml:"database"`
}

type PostgresDBInfo struct {
	Host     string `yaml:"host"`
	Username string `yaml:"username"`
	Password string `yaml:"password"`
	Dbname   string `yaml:"dbname"`
}

type BackendConfig struct {
	PageSize int `yaml:"page_size"`
}

func ReadFromFile(filename string) (Config, error) {
	file, err := os.ReadFile(filename)
	if err != nil {
		return Config{}, err
	}

	var conf Config
	err = yaml.Unmarshal(file, &conf)
	if err != nil {
		return Config{}, err
	}

	return conf, nil
}

package config

// Config defines the configuration structure.
type Config struct {
	General struct {
		LogLevel int `mapstructure:"log_level"`
	}

	LoraServer struct {
		Address string
		Bind    string
	}
}

// C holds the global configuration.
var C Config

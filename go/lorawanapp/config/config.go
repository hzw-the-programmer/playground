package config

type Config struct {
	LogLevel         int    `mapstructure:"log_level"`
	Broker           string `mapstructure:"broker"`
	SubTopic         string `mapstructure:"sub_topic"`
	SubJoinTopic     string `mapstructure:"sub_join_topic"`
	PubTopicTemplate string `mapstructure:"pub_topic_template"`
	DSN              string `mapstructure:"dsn"`
}

var C Config

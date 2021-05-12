/**
 * Author: Zhiwen He <18676797056@163.com>
 */
package cmd

import (
	"bytes"
	log "github.com/sirupsen/logrus"
	"github.com/spf13/cobra"
	"github.com/spf13/viper"
	"io/ioutil"
	"reflect"
	"strings"

	"lorawanapp/config"
)

var version string
var configFile string

func Execute(v string) {
	version = v
	if err := rootCmd.Execute(); err != nil {
		log.Fatal(err)
	}
}

func init() {
	cobra.OnInitialize(initConfig)

	rootCmd.PersistentFlags().StringVarP(&configFile, "config", "c", "", "path to configuration file (optional)")
	rootCmd.PersistentFlags().Int("log-level", 4, "debug=5, info=4, error=2, fatal=1, panic=0")

	rootCmd.AddCommand(versionCmd)
	rootCmd.AddCommand(configfileCmd)
	rootCmd.AddCommand(testCmd)

	viper.BindPFlag("log_level", rootCmd.PersistentFlags().Lookup("log-level"))
	viper.SetDefault("broker", "localhost:1883")
	viper.SetDefault("sub_topic", "application/+/device/+/rx")
	viper.SetDefault("sub_join_topic", "application/+/device/+/join")
	viper.SetDefault("pub_topic_template", "application/{{ .ApplicationID }}/device/{{ .DevEUI }}/tx")
	viper.SetDefault("dsn", "postgres://lorawanapp:lorawanapp@localhost/lorawanapp?sslmode=disable")
}

func initConfig() {
	if configFile != "" {
		b, err := ioutil.ReadFile(configFile)
		if err != nil {
			log.WithError(err).WithField("config", configFile).Fatal("error loading config file")
		}
		viper.SetConfigType("toml")
		if err := viper.ReadConfig(bytes.NewBuffer(b)); err != nil {
			log.WithError(err).WithField("config", configFile).Fatal("error loading config file")
		}
	} else {
		viper.SetConfigName("lorawanapp")
		viper.AddConfigPath(".")
		viper.AddConfigPath("$HOME/.config/lorawanapp")
		viper.AddConfigPath("/etc/lorawanapp")
		if err := viper.ReadInConfig(); err != nil {
			switch err.(type) {
			case viper.ConfigFileNotFoundError:
				log.Warning("No configuration file found, using defaults.")
			default:
				log.WithError(err).Fatal("read configuration file error")
			}
		}
	}

	viperBindEnvs(config.C)

	if err := viper.Unmarshal(&config.C); err != nil {
		log.WithError(err).Fatal("unmarshal config error")
	}
}

func viperBindEnvs(iface interface{}, parts ...string) {
	ift := reflect.TypeOf(iface)
	ifv := reflect.ValueOf(iface)
	for i := 0; i < ift.NumField(); i++ {
		t := ift.Field(i)
		v := ifv.Field(i)
		tv, ok := t.Tag.Lookup("mapstructure")
		if !ok {
			tv = strings.ToLower(t.Name)
		}
		if tv == "-" {
			continue
		}

		switch v.Kind() {
		case reflect.Struct:
			viperBindEnvs(v.Interface(), append(parts, tv)...)
		default:
			key := strings.Join(append(parts, tv), ".")
			viper.BindEnv(key)
		}
	}
}

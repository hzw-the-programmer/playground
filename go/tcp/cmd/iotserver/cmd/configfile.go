package cmd

import (
	"html/template"
	"os"

	"iotserver/internal/config"

	"github.com/pkg/errors"
	"github.com/spf13/cobra"
)

const configfileTemplate = `[general]
# debug=5, info=4, warning=3, error=2, fatal=1, panic=0
log_level = {{ .General.LogLevel }}


[loraserver]
# Lora Server address that Lora Client connected to.
#
# Set to empty string to disable Lora Client.
#
# Example:
# address = "localhost:25001"
address = "{{ .LoraServer.Address }}"

# Lora Server bind address.
#
# Set to empty string to disable Lora Server.
#
# Example:
# bind = "0.0.0.0:25001"
bind = "{{ .LoraServer.Bind }}"
`

var configfileCmd = &cobra.Command{
	Use:   "configfile",
	Short: "Print Iot Server configuration file",
	RunE: func(cmd *cobra.Command, args []string) error {
		t := template.Must(template.New("config").Parse(configfileTemplate))
		err := t.Execute(os.Stdout, config.C)
		if err != nil {
			return errors.Wrap(err, "execute config template error")
		}
		return nil
	},
}

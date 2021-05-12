/**
 * Author: Zhiwen He <18676797056@163.com>
 */
package cmd

import (
	"os"
	"text/template"

	"github.com/pkg/errors"
	"github.com/spf13/cobra"

	"lorawanapp/config"
)

const configfileTemplate = `log_level={{ .LogLevel }}
broker="{{ .Broker }}"
sub_topic="{{ .SubTopic }}"
sub_join_topic="{{ .SubJoinTopic }}"
pub_topic_template="{{ .PubTopicTemplate }}"
dsn="{{ .DSN }}"
`

var configfileCmd = &cobra.Command{
	Use:   "configfile",
	Short: "Print the Lorawan App configuration file",
	RunE: func(cmd *cobra.Command, args []string) error {
		t := template.Must(template.New("configfile").Parse(configfileTemplate))
		err := t.Execute(os.Stdout, config.C)
		if err != nil {
			return errors.Wrap(err, "execute config template error")
		}
		return nil
	},
}

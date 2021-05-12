/**
 * Author: Zhiwen He <18676797056@163.com>
 */
package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

var versionCmd = &cobra.Command{
	Use:   "version",
	Short: "Print Lorawan App version",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println(version)
	},
}

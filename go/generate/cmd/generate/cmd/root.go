package cmd

import (
	"generate/internal/project"

	log "github.com/sirupsen/logrus"
	"github.com/spf13/cobra"
)

var version string

func init() {
	rootCmd.AddCommand(versionCmd)
}

func run(cmd *cobra.Command, args []string) error {
	project.PrintContributors()
	return nil
}

var rootCmd = &cobra.Command{
	Use:   "generate",
	Short: "go generate demo",
	RunE:  run,
}

func Execute(v string) {
	version = v
	if err := rootCmd.Execute(); err != nil {
		log.Fatal(err)
	}
}

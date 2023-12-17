/*
Copyright © 2023 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"context"
	"fmt"
	"log"
	"os"

	"t1/grpc/users"

	"github.com/spf13/cobra"
	"google.golang.org/grpc"
)

// grpcCliCmd represents the grpcCli command
var grpcCliCmd = &cobra.Command{
	Use:   "grpcCli",
	Short: "A brief description of your command",
	Long: `A longer description that spans multiple lines and likely contains examples
and usage of using your command. For example:

Cobra is a CLI library for Go that empowers applications.
This application is a tool to generate the needed files
to quickly create a Cobra application.`,
	Run: grpcCliRun,
}

func init() {
	rootCmd.AddCommand(grpcCliCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// grpcCliCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// grpcCliCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}

func grpcCliRun(cmd *cobra.Command, args []string) {
	conn, err := grpc.DialContext(context.Background(), args[0], grpc.WithInsecure(), grpc.WithBlock())
	if err != nil {
		log.Fatal(err)
	}
	defer conn.Close()

	cli := users.NewUsersClient(conn)
	res, err := cli.GetUser(context.Background(), &users.UserGetRequest{
		Email: "coder@hzw.com",
	})
	if err != nil {
		log.Fatal(err)
	}

	fmt.Fprintf(os.Stdout, "User: %s %s \n", res.User.FirstName, res.User.LastName)
}

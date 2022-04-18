/*
Copyright © 2022 NAME HERE <EMAIL ADDRESS>

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/
package cmd

import (
	"fmt"
	"net/http"
	"io/ioutil"
	"log"
	"encoding/json"

	"github.com/spf13/cobra"
)

// randomCmd represents the random command
var randomCmd = &cobra.Command{
	Use:   "random",
	Short: "Get a random dad joke",
	Long: `This command fetches a random dad joke from the icanhazdadjoke api`,
	Run: func(cmd *cobra.Command, args []string) {
		url := "https://icanhazdadjoke.com/"
		joke, err := getRandomJoke(url)
		if err != nil {
			log.Println(err)
		}
		fmt.Println(joke)
	},
}

func init() {
	rootCmd.AddCommand(randomCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// randomCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// randomCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}

// curl -H "User-Agent: My Library (https://github.com/username/repo)" https://icanhazdadjoke.com/
// curl -H "Accept: application/json" https://icanhazdadjoke.com/
// curl -H "Accept: text/plain" https://icanhazdadjoke.com/
type Joke struct {
	Id string `json:"id"`
	Joke string `json:"joke"`
	Status int `json:"status"`
}

func getRandomJoke(url string) (string, error) {
	buf, err := getJokeData(url)
	if err != nil {
		return "", err
	}

	joke := Joke{}
	err = json.Unmarshal(buf, &joke)
	return joke.Joke, err
}

func getJokeData(baseApi string) ([]byte, error) {
	req, err := http.NewRequest(
		http.MethodGet,
		baseApi,
		nil)
	if err != nil {
		return nil, err
	}

	req.Header.Add("Accept", "application/json")
	req.Header.Add("User-Agent", "Dadjoke CLI (github.com/example/dadjoke)")

	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	return ioutil.ReadAll(resp.Body)
}
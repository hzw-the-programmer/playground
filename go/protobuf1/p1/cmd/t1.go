package cmd

import (
	"io/ioutil"
	"log"

	"github.com/golang/protobuf/proto"
	"github.com/spf13/cobra"

	"p1/pb"
)

var t1Cmd = &cobra.Command{
	Use: "t1",
	Run: func(cmd *cobra.Command, args []string) {
		t1 := &pb.Test1{
			F1: "hj01",
			F2: "hj02",
			F3: "bj01",
			F4: "bj02",
			F5: "bj03",
			F6: 123,
			F7: 456,
			F8: "tj01",
			F9: "tj02",
		}
		out, err := proto.Marshal(t1)
		if err != nil {
			log.Fatalln("t1 marshal failed:", err)
		}
		if err := ioutil.WriteFile("t1", out, 0644); err != nil {
			log.Fatalln("t1 write file error:", err)
		}
	},
}

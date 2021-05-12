/**
* Author: Zhiwen He <18676797056@163.com>
 */
package cmd

import (
	"encoding/base64"
	"encoding/json"
	"log"
	"time"

	MQTT "github.com/eclipse/paho.mqtt.golang"
	"github.com/spf13/cobra"

	"lorawanapp/config"
	// "lorawanapp/protocol/oulang"
	"lorawanapp/protocol/cleargrass"
)

var testCmd = &cobra.Command{
	Use:   "test",
	Short: "Lorawan App test server",
	RunE: func(cmd *cobra.Command, args []string) error {
		opts := MQTT.NewClientOptions()
		opts.AddBroker(config.C.Broker)

		client := MQTT.NewClient(opts)
		if token := client.Connect(); token.Wait() && token.Error() != nil {
			panic(token.Error())
		}

		jn := JoinNotification{
			// DeviceName: "02112191300107",
			DeviceName: "02112191300108",
			DevAddr:    [4]byte{0x01, 0x02, 0x03, 0x04},
		}
		jsjn, _ := json.Marshal(jn)

		token := client.Publish("application/2/device/3333333333330002/join", 0, false, jsjn)
		token.Wait()
		if token.Error() != nil {
			panic(token.Error())
		}

		var status int
		for {
			if status == 0x00 {
				status = 0x01
			} else {
				status = 0x00
			}
			select {
			case <-time.Tick(3 * time.Second):
				log.Print()
				var dupl DataUpPayload
				// dupl.ApplicationName = "idas"
				// dupl.DeviceName = "02112191300107"
				// pkt := oulang.Packet{
				// 	Battery:     0xff,
				// 	Temperature: 0x1a,
				// 	Type:        oulang.PTStatus,
				// 	Channel:     1,
				// 	CType:       oulang.CTGNDH,
				// 	Status:      status,
				// 	Data:        40.3,
				// }
				dupl.ApplicationName = "cleargrass"
				dupl.DeviceName = "02112191300108"
				pkt := cleargrass.Packet{
					Addr: cleargrass.Addr,
					Data: &cleargrass.RealTimeData{
						Time: time.Now(),
						SD:   cleargrass.SensorData{26.4, 66.6, 100.86, 78.0},
						Ver:  "1.0.0_0041",
					},
				}
				pktbs, err := pkt.Marshal()
				if err != nil {
					panic(err)
				}
				// dupl.Data = pktbs
				b64 := make([]byte, base64.StdEncoding.EncodedLen(len(pktbs)))
				base64.StdEncoding.Encode(b64, pktbs)
				dupl.Data = b64

				jspl, err := json.Marshal(dupl)
				if err != nil {
					panic(err)
				}
				token := client.Publish("application/2/device/3333333333330002/rx", 0, false, jspl)
				token.Wait()
				if token.Error() != nil {
					panic(token.Error())
				}
			}
		}

		return nil
	},
}

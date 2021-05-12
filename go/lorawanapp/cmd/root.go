/**
 * Author: Zhiwen He <18676797056@163.com>
 */
package cmd

import (
	"bytes"
	"database/sql"
	"encoding/base64"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"log"
	"os"
	"strings"
	"text/template"
	"time"

	"github.com/brocaar/lorawan"
	MQTT "github.com/eclipse/paho.mqtt.golang"
	_ "github.com/go-sql-driver/mysql"
	"github.com/gofrs/uuid"
	"github.com/spf13/cobra"

	"lorawanapp/config"
	"lorawanapp/protocol/cleargrass"
	"lorawanapp/protocol/idas"
	"lorawanapp/protocol/oulang"
	"lorawanapp/storage"
)

var rootCmd = &cobra.Command{
	Use:   "lorawanapp",
	Short: "Lorawan application server",
	Long: `Lorawan App is an open-source application server
> documentation & support: https://github.com/hzw-the-programmer/python-play-ground
> source & copyright information: https://github.com/hzw-the-programmer/python-play-ground`,
	RunE: run,
}

func run(cmd *cobra.Command, args []string) error {
	qos := 0
	pubTopicTemplate, _ := template.New("pubTopic").Parse(config.C.PubTopicTemplate)

	msgChan := make(chan MQTT.Message)

	opts := MQTT.NewClientOptions()
	opts.AddBroker(config.C.Broker)
	opts.SetDefaultPublishHandler(func(client MQTT.Client, msg MQTT.Message) {
		msgChan <- msg
	})

	client := MQTT.NewClient(opts)
	if token := client.Connect(); token.Wait() && token.Error() != nil {
		panic(token.Error())
	}

	if token := client.Subscribe(config.C.SubTopic, byte(qos), nil); token.Wait() && token.Error() != nil {
		fmt.Println(token.Error())
		os.Exit(1)
	}

	//sub join
	if token := client.Subscribe(config.C.SubJoinTopic, byte(qos), nil); token.Wait() && token.Error() != nil {
		fmt.Println(token.Error())
		os.Exit(1)
	}

	storage.SetUpStorage()

	for {
		msg := <-msgChan

		log.Printf("topic: %s", msg.Topic())
		log.Printf("msg  : %s", string(msg.Payload()))

		if strings.HasSuffix(msg.Topic(), "join") {
			joinPacketHandler(storage.GetDB(), msg)
		} else if strings.HasSuffix(msg.Topic(), "rx") {
			var dupl DataUpPayload
			err := json.Unmarshal(msg.Payload(), &dupl)
			if err != nil {
				log.Print(err)
				continue
			}

			var data []byte
			if dupl.ApplicationName == "idas" {
				data = dupl.Data
			} else {
				data, err = base64.StdEncoding.DecodeString(string(dupl.Data))
				if err != nil {
					log.Print(err)
					continue
				}
			}

			log.Printf("data : %s", hex.EncodeToString(data))
			if len(data) == 0 {
				continue
			}

			if dupl.ApplicationName == "idas" {
				var pkt idas.Packet

				err = pkt.Unmarshal(data)

				if err != nil {
					log.Print(err)

					var pkt oulang.Packet

					err = pkt.Unmarshal(data)

					if err != nil {
						log.Print(err)
						continue
					}

					mpid, err := storage.GetMPID(dupl.DeviceName, pkt.Channel, 0, pkt.CType)
					if err != nil {
						log.Print(err)
						continue
					}
					if mpid == 0 {
						log.Print("channel does not exist")
						continue
					}

					var status int
					if pkt.CType == oulang.CTWS {

					} else {
						if pkt.Status&0x01 == 0x01 {
							status = 0x20
						} else {
							status = 0x00
						}
					}

					now := time.Now().Unix()

					_, err = storage.UpdateRealTimeStatus(mpid, 0, status, now, pkt.Data)
					if err != nil {
						log.Print(err)
						continue
					}

					_, err = storage.InsertStatus(mpid, 0, status, now, pkt.Data)
					if err != nil {
						log.Print(err)
						continue
					}

					_, err = storage.InsertData(mpid, 0, now, pkt.Data)
					if err != nil {
						log.Print(err)
						continue
					}
				}
			} else if dupl.ApplicationName == "vmtest" {

			} else {
				var pkt cleargrass.Packet

				err = pkt.Unmarshal(data)
				if err != nil {
					log.Print(err)
					continue
				}

				switch pkt.Cmd {
				case cleargrass.TimeSyncCmd:
					ts := cleargrass.TimeSync{
						Time: time.Now(),
					}
					pkt := cleargrass.Packet{
						Addr: cleargrass.Addr,
						Cmd:  cleargrass.TimeSyncCmd,
						Data: &ts,
					}
					pktb, err := pkt.Marshal()
					if err != nil {
						log.Fatal(err)
					}
					log.Print(hex.EncodeToString(pktb))

					pubTopic := bytes.NewBuffer(nil)
					err = pubTopicTemplate.Execute(pubTopic, struct {
						ApplicationID int64
						DevEUI        lorawan.EUI64
					}{dupl.ApplicationID, dupl.DevEUI})
					if err != nil {
						log.Fatal(err)
					}

					var ddpl DataDownPayload
					ddpl.Confirmed = true
					ddpl.FPort = 10
					pkts := base64.StdEncoding.EncodeToString(pktb)
					ddpl.Data = append(ddpl.Data, []byte(pkts)...)
					jspl, err := json.Marshal(ddpl)
					token := client.Publish(pubTopic.String(), byte(qos), false, jspl)
					token.Wait()
				case cleargrass.DataUpCmd:
					switch data := pkt.Data.(type) {
					case *cleargrass.RealTimeData:
						now := time.Now().Unix()

						for slot := uint8(1); slot < 3; slot++ {
							var typ uint8
							var d float32

							if slot == 1 {
								typ = idas.ChanTypeTemp
								d = data.SD.Temperature
							} else {
								typ = idas.ChanTypeHumi
								d = data.SD.Humility
							}

							mpid, err := storage.GetMPID(dupl.DeviceName, slot, 0, typ)
							if err != nil {
								log.Print(err)
								continue
							}
							if mpid == 0 {
								log.Print("channel does not exist")
								continue
							}

							_, err = storage.InsertData(mpid, 0, now, d)
							if err != nil {
								log.Print(err)
								continue
							}
						}
					}
				}
			}
		}

		log.Print()
	}

	client.Disconnect(250)
	fmt.Println("Sample Subscriber Disconnected")
	return nil
}

func joinPacketHandler(db *sql.DB, msg MQTT.Message) {
	var joinPacket JoinNotification
	err := json.Unmarshal(msg.Payload(), &joinPacket)
	if err != nil {
		log.Print(err)
		return
	}

	log.Print("receive join packet, device_name: %s", joinPacket.DeviceName)

	var device_id int
	err = db.QueryRow("SELECT id FROM devices_info WHERE sn = ? LIMIT 1", joinPacket.DeviceName).Scan(&device_id)

	if err != nil {
		log.Print(err)
		if err == sql.ErrNoRows {
			now := time.Now().Unix()
			result, err := db.Exec(`INSERT INTO devices_info
				(parent_id, sn, mac, ip, port, devaddr, type, status, level, version, createtime, updatetime, power, nw_type)
				values(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`,
				0, joinPacket.DeviceName, joinPacket.DeviceName, "", 0, hex.EncodeToString(joinPacket.DevAddr[:]), 0, 0, 1, "", now, now, 0, "LORAWAN",
			)

			if err != nil {
				log.Print(err)
				return
			}

			lastId, err := result.LastInsertId()
			if err != nil {
				log.Print(err)
			}
			rowCnt, err := result.RowsAffected()
			if err != nil {
				log.Print(err)
			}
			log.Printf("ID = %d, affected rows = %d\n", lastId, rowCnt)

		}
		return
	}

	//update
	now := time.Now().Unix()
	result, err := db.Exec(
		"UPDATE devices_info SET devaddr = ?, updatetime = ? WHERE id = ?",
		hex.EncodeToString(joinPacket.DevAddr[:]), now, device_id,
	)
	if err != nil {
		log.Print(err)
		return
	}

	rowCnt, err := result.RowsAffected()
	if err != nil {
		log.Print(err)
		return
	}
	log.Print("update device info, rows affected:%d", rowCnt)
}

type Location struct {
	Latitude  float64 `json:"latitude"`
	Longitude float64 `json:"longitude"`
	Altitude  float64 `json:"altitude"`
}

type RXInfo struct {
	GatewayID lorawan.EUI64 `json:"gatewayID"`
	UplinkID  uuid.UUID     `json:"uplinkID"`
	Name      string        `json:"name"`
	Time      *time.Time    `json:"time,omitempty"`
	RSSI      int           `json:"rssi"`
	LoRaSNR   float64       `json:"loRaSNR"`
	Location  *Location     `json:"location"`
}

type TXInfo struct {
	Frequency int `json:"frequency"`
	DR        int `json:"dr"`
}

type DataUpPayload struct {
	ApplicationID   int64             `json:"applicationID,string"`
	ApplicationName string            `json:"applicationName"`
	DeviceName      string            `json:"deviceName"`
	DevEUI          lorawan.EUI64     `json:"devEUI"`
	RXInfo          []RXInfo          `json:"rxInfo,omitempty"`
	TXInfo          TXInfo            `json:"txInfo"`
	ADR             bool              `json:"adr"`
	FCnt            uint32            `json:"fCnt"`
	FPort           uint8             `json:"fPort"`
	Data            []byte            `json:"data"`
	Object          interface{}       `json:"object,omitempty"`
	Tags            map[string]string `json:"tags,omitempty"`
	Variables       map[string]string `json:"-"`
}

type DataDownPayload struct {
	ApplicationID int64           `json:"applicationID,string"`
	DevEUI        lorawan.EUI64   `json:"devEUI"`
	Confirmed     bool            `json:"confirmed"`
	FPort         uint8           `json:"fPort"`
	Data          []byte          `json:"data"`
	Object        json.RawMessage `json:"object"`
}

type JoinNotification struct {
	ApplicationID   int64             `json:"applicationID,string"`
	ApplicationName string            `json:"applicationName"`
	DeviceName      string            `json:"deviceName"`
	DevEUI          lorawan.EUI64     `json:"devEUI"`
	DevAddr         lorawan.DevAddr   `json:"devAddr"`
	Tags            map[string]string `json:"tags,omitempty"`
	Variables       map[string]string `json:"-"`
}

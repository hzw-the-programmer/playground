### 网关和服务器通讯主要涉及两个程序
1. [packet_forwarder](https://github.com/Lora-net/packet_forwarder)
2. [lora-gateway-bridge](https://github.com/brocaar/lora-gateway-bridge)

packet_forwarder在网关上运行。

main线程负责收集网关收发包等统计信息。

thread_up线程负责从lora取数据(lgw_receive)，
并将取得数据和main线程收集的网关统计数据以PUSH_DATA包通过UDP发送给lora-gateway-bridge。

thread_down线程负责每隔一段时间向lora-gateway-bridge发送PULL_DATA包，
并接收lora-gateway-bridge发送给它的PULL_DATA_ACK包和PULL_RESP包。
当PULL_RESP包里面的下行数据交给lora发送后向lora-gateway-bridge发送TX_ACK包。

lora-gateway-bridge启动时在UDP端口1700收网关数据。

当收到PUSH_DATA包时，向网关发送PUSH_DATA_ACK包。
将PUSH_DATA包中的网关统计信息解析出来以gateway/{gatewayid}/event/stats主题通知MQTT代理服务器。
将PUSH_DATA包中的lora数据解析出来以gateway/{gatewayid}/event/up主题通知MQTT代理服务器。

当收到PULL_DATA包时，向网关发送PULL_DATA_ACK包并更新内存中网关数据。
如果该网关是新网关将会向MQTT代理服务器订阅gateway/{gatewayid}/command/#主题。

当收到TX_ACK包时，向MQTT代理服务器推送gateway/{gatewayid}/event/ack主题。

当收到MQTT代理服务器推送主题gateway/{gatewayid}/command/down时向网关发送PULL_RESP包。
当收到MQTT代理服务器推送主题gateway/{gatewayid}/command/config时配置网关。
当收到MQTT代理服务器推送主题gateway/{gatewayid}/command/exec时执行命令。

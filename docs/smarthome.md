device type

gateway    0001
socket     0002
switch     0003
doormagnet 0004

device

id, mac, type, pid
01, 001, 0001, 000
02, 002, 0002, 001
03, 003, 0003, 001
04, 004, 0004, 001

服务器向MQTT Broker订阅如下主题：
网关上行数据topic为/loratoo/xxx/uplink  xxx为网关的gatwway ID
网关上线的topic为/loratoo/gwonline
网关下线的topic为/loratoo/gwoffline

服务器向MQTT Broker发布如下主题：
服务器下行数据topic为/loratoo/xxx/downlink xxx为网关的gatwway ID

每个主题对应数据参见《LoRa网关SDK协议.docx》

服务器端分为两个模块：
1. HTTPS RESTFULL API接口模块：负责配置、查询功能。
2. 安全WebSocket模块：负责和前端通讯，包括如下两个功能：
   主动推送设备状态至前端
   接收前端命令并发布到MQTT Broker

Item
设备状态管理。

Function
设备离线管理。
设备实时状态推送。

Specification
根据网关上传的心跳包判断网关是否离线。
将网关上传的设备状态推送至前端。

Item
设备配置信息下发。

Function
前端对设备的配置信息下发到网关。

Specification
网关根据配置信息在无服务器的情况下也能工作。

Item
设备控制命令下发。

Function
前端对设备直接下发命令。

Specification
用户可以通过前端界面直接向设备发送控制命令。

Item
设备联动命令下发。

Function
根据前端配置的联动条件及联动动作向相应设备发送相应命令。

Specification
用户可在前端新建联动方案，定义联动条件及相应联动动作。
服务器在联动条件满足时下发相应联动命令。

Item
设备场景命令下发

Function
用户可在前端下发场景命令

Specification
在收到前端发送的场景命令后根据前端之前配置的场景命令下发命令。

Item
设备状态管理。
设备配置信息下发。
设备控制命令下发。
设备联动命令下发。
设备场景命令下发

Function
设备离线管理。
设备实时状态推送。
前端对设备的配置信息下发到网关。
前端对设备直接下发命令。
根据前端配置的联动条件及联动动作向相应设备发送相应命令。
用户可在前端下发场景命令

Specification
根据网关上传的心跳包判断网关是否离线。
将网关上传的设备状态推送至前端。
网关根据配置信息在无服务器的情况下也能工作。
用户可以通过前端界面直接向设备发送控制命令。
用户可在前端新建联动方案，定义联动条件及相应联动动作。
服务器在联动条件满足时下发相应联动命令。
在收到前端发送的场景命令后根据前端之前配置的场景命令下发命令。

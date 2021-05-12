*********************************
lora server
*********************************

service_profile

service_profile_id: 在插入数据库之前由uuid.NewV4()生成
network_server_id: 已存在的ID
organization_id: 已存在的ID
created_at: time.Now()
updated_at: time.Now()
name: 用户输入

*********************************
lora app server
*********************************

service_profile

created_at: time.Now()
updated_at: time.Now()

service_profile_id: 在lora server生成
ul_rate
ul_bucket_size
ul_rate_policy
dl_rate
dl_bucket_size
dl_rate_policy
add_gw_metadata
dev_status_req_freq
report_dev_status_battery
report_dev_status_margin
dr_min
dr_max
channel_mask
pr_allowed
hr_allowed
ra_allowed
nwk_geo_loc
target_per
min_gw_diversity

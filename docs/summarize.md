1. 4+4 wap+本地版本
    完成wap端功能开发：登录、会话列表、消息列表、消息发送、联系人列表、多国语言、联系人同步、json转换pb
2. 4+4 缩容版本
    protobuf缩容22k
    编写ra_buffer及其单元测试。替换key_value_list达到缩容及减少内存碎片
    简化S平台宽字符打印函数，省2k
    crc32去掉table，省1k
    用S平台aes接口，省2k
    用S平台sha256接口，省1k
3. 小屏适配
    文本滚动
    左键改选项菜单，中键移入选项菜单
    动态适配长软键文本
    垂直方向线性布局
    滚动条
4. 游客模式
   RSA公钥加密、游客登录、手机号绑定
5. 代码优化
    完成M平台内存管理优化
6. 效率相关
   编写ROM空间统计工具
   编写提包工具
   用平台、分辨率、版本、release宏简化宏配置，同时也方便提包工具自动提取这些信息并生成包名
   优化S平台编译文件
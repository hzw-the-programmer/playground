Repeat 是源。
Take 是适配器，包装 Stream，本身是 Stream。
collect 是消费器，包装 Stream，本身是 Future。

Pending 源 poll_next 直接返回 Pending，没有注册 Waker，怎么通知任务呢？

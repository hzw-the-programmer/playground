# 用go语言“默认值可用”理念简化如下代码
```
void func(parameter *p) {
    if (p != NULL) {
        // 一大段对p的处理
    }
    
    process();
}
```
# 简化后代码
```
void func(parameter *p) {
    parameter param = {0};
    
    if (p != NULL) {
        param = *p;
    }
    
    // 这之后可以大胆使用param，不用再判断 p != NULL
    process();
}
```
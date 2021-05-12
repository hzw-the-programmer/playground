在web/app.php中有如下代码：
$response = $kernel->handle($request);
$response->send();
可见，reponse由kernel处理request后返回。
接着调用response的send将headers和content发送出去。代码如下：
vendor/symfony/symfony/src/Symfony/Component/HttpFoundation/Response.php
public function send()
{
    $this->sendHeaders();
    $this->sendContent();
    ......
}
public function sendHeaders()
{
    ......
    header($name.': '.$value, false, $this->statusCode);
    ......
}
public function sendContent()
{
    echo $this->content;

    return $this;
}

StreamResponse中的sendContent:
vendor/symfony/symfony/src/Symfony/Component/HttpFoundation/StreamResponse.php
public function sendContent()
{
    call_user_func($this->callback);
}
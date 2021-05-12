<?php
require __DIR__ . '/vendor/autoload.php';

use Psr\Http\Message\ServerRequestInterface;
use React\EventLoop\Factory;
use React\Http\Response;
use React\Http\Server;
use React\Stream\ReadableResourceStream;

$loop = Factory::create();

$queryParamMiddleware = function(ServerRequestInterface $request, callable $next) {
    $params = $request->getQueryParams();

    if (!isset($params['file'])) {
        return new Response(200, ['Content-Type' => 'text/plain'], 'Video streaming server');
    }

    return $next($request);
};

$checkFileExistsMiddleware = function(ServerRequestInterface $request, callable $next) {
    $file = $request->getQueryParams()['file'];
    $filePath = __DIR__ . DIRECTORY_SEPARATOR . 'media' . DIRECTORY_SEPARATOR . basename($file);
    @$fileStream = fopen($filePath, 'r');
    if (!$fileStream) {
        return new Response(404, ['Content-Type' => 'text/plain'], "Video $file doesn't exist on server");
    }

    return $next($request);
};

$videoStreamingMiddleware = function(ServerRequestInterface $request) use($loop) {
    $file = $request->getQueryParams()['file'];
    $filePath = __DIR__ . DIRECTORY_SEPARATOR . 'media' . DIRECTORY_SEPARATOR . basename($file);
    $video = new ReadableResourceStream(fopen($filePath, 'r'), $loop);
    return new Response(200, ['Content-Type' => 'video/mp4'], $video);
};

$server = new Server([
    $queryParamMiddleware,
    $checkFileExistsMiddleware,
    $videoStreamingMiddleware,
]);
$socket = new React\Socket\Server('0.0.0.0:8000', $loop);
$server->listen($socket);

echo 'Listening on ' . str_replace('tcp', 'http', $socket->getAddress()) . PHP_EOL;

$loop->run();

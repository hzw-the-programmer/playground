<?php
require __DIR__ . '/vendor/autoload.php';

use React\EventLoop\Factory;
use React\Http\Response;
use React\Http\Server;
use React\Stream\ReadableResourceStream;
use Psr\Http\Message\ServerRequestInterface;

$loop = Factory::create();

$socket = new React\Socket\Server('0.0.0.0:8000', $loop);

$server = new Server(function(ServerRequestInterface $request) use($loop) {
    $params = $request->getQueryParams();

    $file = $params['video'] ?? '';
    if (empty($file)) {
        return new Response(200, ['Content-Type' => 'text/plain'], 'Video streaming server');
    }

    $filePath = __DIR__ . DIRECTORY_SEPARATOR . $file;
    @$fileStream = fopen($filePath, 'r');
    if (!$fileStream) {
        return new Response(404, ['Content-Type' => 'text/plain'], "Video $file doesn't exist on server.");
    }

    $video = new ReadableResourceStream($fileStream, $loop);

    return new Response(200, ['Content-Type' => getMimeTypeByExtension($filePath)], $video);
});
$server->listen($socket);

$loop->run();

function getMimeTypeByExtension($filename) {
    $types = [
        '.avi' => 'video/avi',
        '.m1v' => 'video/mpeg',
        '.m2a' => 'video/mpeg',
        '.m2v' => 'video/mpeg',
        '.mov' => 'video/quicktime',
        '.mp4' => 'video/mp4',
    ];

    foreach ($types as $extension => $type) {
        if (substr($filename, -strlen($extension)) === $extension) {
            return $type;
        }
    }

    return null;
}

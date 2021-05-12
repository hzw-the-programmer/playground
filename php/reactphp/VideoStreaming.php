<?php
use Psr\Http\Message\ServerRequestInterface;
use React\EventLoop\LoopInterface;
use React\Http\Response;
use React\Stream\ReadableResourceStream;

class VideoStreaming {
    private $eventLoop;

    public function __construct(LoopInterface $eventLoop) {
        $this->eventLoop = $eventLoop;
    }

    function __invoke(ServerRequestInterface $request) {
        $filePath = $this->getFilePath($request);
        if (empty($filePath)) {
            return new Response(200, ['Content-Type' => 'text/plain'], 'Video streaming server');
        }

        return $this->makeResponseFromFile($filePath);
    }

    private function getFilePath(ServerRequestInterface $request) {
        $file = $request->getQueryParams()['file'] ?? '';
        if (empty($file)) return '';
        return __DIR__ . DIRECTORY_SEPARATOR . 'media' . DIRECTORY_SEPARATOR . basename($file);
    }

    private function makeResponseFromFile($filePath) {
        @$fileStream = fopen($filePath, 'r');
        if (!$fileStream) {
            return new Response(404, ['Content-Type' => 'text/plain'], "Video $filePath doesn't exist on server.");
        }

        $video = new ReadableResourceStream($fileStream, $this->eventLoop);
        return new Response(200, ['Content-Type' => $this->getMimeTypeByExtension($filePath)], $video);
    }

    private function getMimeTypeByExtension($filename) {
        $types = [
            '.avi' => 'video/avi',
            '.m1v' => 'video/mpeg',
            '.m2a' => 'video/mpeg',
            '.m2v' => 'video/mpeg',
            '.mov' => 'video/mpeg',
            '.mp4' => 'video/mp4',
        ];

        foreach ($types as $extension => $type) {
            if (substr($filename, -strlen($extension)) === $extension) {
                return $type;
            }
        }

        return null;
    }
}

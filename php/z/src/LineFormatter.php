<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw;

use Monolog\Formatter\FormatterInterface;

class LineFormatter implements FormatterInterface {
    public function format(array $record) {
        $datetime = $record['datetime']->format('Y-m-d H:i:s');
        $channel = $record['channel'];
        $level_name = $record['level_name'];

        $message = $record['message'];
        $context = $record['context'];

        switch ($message) {
            case 'sendto':
                $address = $context['address'];
                $data = $context['data'];
                $packet = Packet::unpack($data);
                $packet->setTarget($address);
                $message = sprintf("%s %s %s", $message, $packet, bin2hex($data));
                break;

            case 'multicast':
                $addresses = array_map(function($e) {
                    return implode(':', $e);
                }, $context['addresses']);
                $addresses = implode(',', $addresses);
                $data = $context['data'];
                $packet = Packet::unpack($data);
                $message = sprintf("%s %s %s %s", $message, $addresses, $packet, bin2hex($data));
                break;

            case 'onPacket':
                $packet = $context['packet'];
                $data = $context['data'];
                $message = sprintf("%s %s %s", $message, $packet, bin2hex($data));
                break;

            default:
                $message = sprintf("%s %s", $message, static::arrToStr($context));
                break;
        }

        return "[$datetime] $channel.$level_name: $message\n";
    }

    public function formatBatch(array $records) {
        $message = '';
        foreach ($records as $record) {
            $message .= $this->format($record);
        }

        return $message;
    }

    public static function arrToStr($arr) {
        return  rtrim(array_reduce(
            array_keys($arr),
            function($carry, $key) use ($arr) {
                $value = $arr[$key];
                if (is_bool($value)) {
                    $value = $value ? 'true' : 'false';
                } else if (is_array($value)) {
                    $value = static::arrToStr($value);
                }
                return "$carry$key:$value,";
            },
            '['
        ), ',') . ']';
    }
}

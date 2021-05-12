<?php
/**
 * Author: Zhiwen He <18676797056@163.com>
 */
namespace Hzw;

use Hzw\Cmd\SetSn;
use Hzw\Cmd\GetSn;
use Hzw\Cmd\SetParam;
use Hzw\Cmd\GetParam;
use Hzw\Cmd\SetWorkTime;
use Hzw\Cmd\GetWorkTime;
use Hzw\Cmd\Open;
use Hzw\Cmd\Close;
use Hzw\Cmd\Ack;

use Hzw\Task\OpenMpoint;
use Hzw\Task\CloseMpoint;

class Client {
    private $cntx;
    private $fd;
    private $config;
    private $tid;

    public function __construct($cntx, $fd) {
        $this->cntx = $cntx;
        $this->fd = $fd;
    }

    private function setConfig($config) {
        $cntx = $this->cntx;
        $serv = $cntx->getServ();

        $this->config = $config;
        if ($config) {
            $this->tid = $serv->after(Config::DELAY, [$this, 'timeout']);
        } else if ($this->tid) {
            $serv->clearTimer($this->tid);
        }
    }

    public function timeout() {
        $cntx = $this->cntx;
        $logger = $cntx->getLogger();

        $msg = 'default';
        if (isset($this->config->address)) {
            $msg = implode(':', $this->config->address);
        } else if (isset($this->config->seqs)) {
            $msg = implode(',', array_keys($this->config->seqs));
        }

        $logger->warning(sprintf(
            'config: client %02d: operation timeout: %s.', $this->fd, $msg
        ));
        $this->config = null;
        $this->tid = null;
    }

    private function checkAddress($address) {

    }

    private function checkPacket($packet) {
        $config = $this->config;

        $body = $packet->getBody();
        if ($body::CMD !== Ack::CMD
            || $body->getCmd() !== $config->cmd
        ) {
            return false;
        }

        $seq = $body->getSeq();

        if (isset($config->address)) {
            $source = $packet->getSource();
            if ($config->address === $source && $config->seq === $seq) {
                $this->setConfig(null);
                return true;
            }
            return false;
        }

        if (isset($config->seqs)) {
            $header = $packet->getHeader();
            $sns = $header->getSns();
            $sn = $sns[count($sns) - 1];
            if (isset($config->seqs[$sn]) && $config->seqs[$sn] === $seq) {
                unset($config->seqs[$sn]);
                if (!$config->seqs) {
                    $this->setConfig(null);
                }
                return true;
            }
        }
        return false;
    }

    private function getBody($config) {
        switch ($config->cmd) {
            case GetSn::CMD:
                return new GetSn();
            case SetSn::CMD:
                return new SetSn($config->sns);

            case Open::CMD:
            case Close::CMD:
                $config->channels = Utils::convertToChannels($config->channels);
                return $config->cmd === Open::CMD
                    ? new Open($config->channels)
                    : new Close($config->channels);

            case GetParam::CMD:
                return new GetParam($config->type);
            case SetParam::CMD:
                return new SetParam($config->param);

            case GetWorkTime::CMD:
                return new GetWorkTime();
            case SetWorkTime::CMD:
                return new SetWorkTime($config->periods);

            default:
                return null;
        }
    }

    public function onMessage($data) {
        $cntx = $this->cntx;
        $logger = $cntx->getLogger();
        $deviceMgr = $cntx->getDeviceMgr();
        $clientMgr = $cntx->getClientMgr();

        if ($this->config) {
            $logger->warning(sprintf(
                'config: client %02d: busy.', $this->fd
            ));
            return;
        }

        $config = json_decode($data);

        $body = $this->getBody($config);
        if (!$body) {
            $logger->warning(sprintf(
                'config: client %02d: cmd 0x%02X is not found.',
                $this->fd, $config->cmd
            ));
            return;
        }

        $dt = time();

        if (isset($config->ip) && $config->ip) {
            $sns = [];

            $ip = $config->ip;
            $port = isset($config->port) && $config->port
                ? intval($config->port)
                : Config::UDP_PORT;
            $address = ['ip' => $ip, 'port' => $port];

            $seq = $clientMgr->nextSeq();

            $logger->debug(sprintf(
                'config: client %02d: address=%s,seq=%d.',
                $this->fd, implode(':', $address), $seq
            ));

            $header = new Header($sns, $seq, $dt);
            $packet = new Packet($header, $body);
            $cntx->sendto($address, $packet->pack());

            $config->address = $address;
            $config->seq = $seq;
            $this->setConfig($config);
            return;
        }

        if (!isset($config->sns) || !$config->sns) {
            $logger->warning(sprintf(
                'config: client %02d: sns is not set.', $this->fd
            ));
            return;
        }

        $seqs = [];
        $loraAddresses = [];
        $loraSeq = null;
        foreach ($config->sns as $sn) {
            $sns = [$sn];
            $device = $deviceMgr->getDevice($sn);
            if (!$device) {
                $logger->warning(sprintf(
                    'config: client %02d: sn %s does not exist.',
                    $this->fd, $sn
                ));
                continue;
            }
            $psn = $device->getPsn();
            $address = $device->getAddress();

            if ($psn) {
                array_unshift($sns, $psn);
                $pdevice = $deviceMgr->getDevice($psn);
                if ($pdevice) {
                    $address = $pdevice->getAddress();
                } else {
                    $logger->warning(sprintf(
                        'config: client: %02d: psn %s does not exist.',
                        $this->fd, $psn
                    ));
                }
            }

            if (isset($address['devaddr'])) {
                if (!$loraSeq) $loraSeq = $clientMgr->nextSeq();
                $seqs[$sn] = $loraSeq;
                $loraAddresses[] = $address;
            } else if (isset($address['ip'])) {
                $seqs[$sn] = $seq = $clientMgr->nextSeq();

                $logger->debug(sprintf(
                    'config: client %02d: address=%s,seq=%d.',
                    $this->fd, implode(':', $address), $seq
                ));

                $header = new Header($sns, $seq, $dt);
                $packet = new Packet($header, $body);
                $cntx->sendto($address, $packet->pack());
            } else {
                $logger->warning(sprintf(
                    'config: client %02d: sn %s ip or devaddr does not exist.',
                    $this->fd, $sn
                ));
                continue;
            }
        }

        if ($loraAddresses) {
            $sns = [];
            $header = new Header($sns, $loraSeq, $dt);
            $packet = new Packet($header, $body);
            if (count($loraAddresses) === 1) {
                $cntx->sendto($loraAddresses[0], $packet->pack());
            } else {
                $cntx->multicast($loraAddresses, $packet->pack());
            }
        }

        $config->seqs = $seqs;
        $this->setConfig($config);
    }

    function onPacket($packet) {
        $config = $this->config;
        if (!$config) return false;
        if (!$this->checkPacket($packet)) return false;

        $cntx = $this->cntx;
        $deviceMgr = $cntx->getDeviceMgr();

        $header = $packet->getHeader();
        $body = $packet->getBody();
        $source = $packet->getSource();
        $pcdt = $packet->getPcdt();
        
        $sns = $header->getSns();
        $seq = $header->getSeq();
        $dt = $header->getDt();

        $sn = $sns[count($sns) - 1];

        $cmd = $body->getCmd();
        $status = $body->getStatus();
        $data = $body->getData();

        $push = true;
        $res = ['sn' => $sn, 'cmd' => $cmd, 'result' => $status];
        switch ($cmd) {
            case GetSn::CMD:
                $res['sns'] = $data;
                foreach ($data as $sn) {
                    $arr = [$sn];
                    if ($sn !== $data[0]) {
                        array_unshift($arr, $data[0]);
                    }
                    $deviceMgr->updateDevice($arr, $source, $pcdt);
                }
                break;

            case Open::CMD:
            case Close::CMD:
                if (!$status) {
                    $push = false;
                    $channels = $config->channels;
                    if ($config->cmd === Open::CMD) {
                        $task = new OpenMpoint($sn, $seq, $dt, $pcdt, $channels, $this->fd);
                    } else {
                        $task = new CloseMpoint($sn, $seq, $dt, $pcdt, $channels, $this->fd);
                    }
                    TaskManager::workerTask($task);
                }
                break;

            case GetParam::CMD:
                $res['param'] = $data;
                break;
            case SetParam::CMD:
                $res['type'] = $config->param->type;
                break;

            case GetWorkTime::CMD:
                $res['periods'] = $data;
                break;

            default:
                break;
        }

        if ($push) {
            $cntx->push($this->fd, json_encode($res));
        }

        return true;
    }
}

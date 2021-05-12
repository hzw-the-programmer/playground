<?php
namespace AppBundle\Controller;

use Sensio\Bundle\FrameworkExtraBundle\Configuration\Route;
use Symfony\Bundle\FrameworkBundle\Controller\Controller;
use Symfony\Component\HttpFoundation\Request;
use Symfony\Component\HttpFoundation\Response;

class ConfigOfflineController extends Controller {
    const PATH = "/home/zhiwenhe/work/python-play-ground/php/z/src/Config.php";
    const PATTERN = "/const OFFLINE_TIMEOUT = (\d+);/";

    /**
     * @Route("hidden/config/offline", name="config_offline")
     */
    public function configAction(Request $request) {
        $success = false;

        $rtime = intval($request->get("time"));
        $stime = "";
        $msg = "";

        again:
        $contents = file_get_contents(self::PATH);
        if ($contents === false) {
            $msg = error_get_last()["message"];
            goto end;
        }

        if (preg_match(self::PATTERN, $contents, $matches)) {
            $stime = intval($matches[1]);
        }

        if ($rtime <= 0) {
            $msg = "time must be positive integer.";
            goto end;
        }

        if ($rtime === $stime) {
            $msg = "success.";
            goto end;
        }

        $replacement = preg_replace(["/\//", "/\(\\\\d\+\)/"], ["", $rtime], self::PATTERN);
        $contents = preg_replace(self::PATTERN, $replacement, $contents);
        if (file_put_contents(self::PATH, $contents) === false) {
            $msg = error_get_last()["message"];
        } else {
            goto again;
        }

        end:
        return $this->render("icloud/config_offline.html.twig", [
            "action" => $this->generateUrl("config_offline"),
            "time" => $stime,
            "msg" => $msg
        ]);
    }
}

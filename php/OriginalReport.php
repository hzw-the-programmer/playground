<?php
namespace Hzw;

class OriginalReport extends Controller
{
    public function tableData(Request $request)
    {
        if ('OPTIONS' == $request->getMethod()) {
            $response = new Response();
            $response->headers->set('Access-Control-Allow-Origin', '*');
            $response->headers->set('Access-Control-Allow-Headers', 'Content-Type');
            return $response;
        }

        $json = json_decode($request->getContent());

        $dt = new \DateTime('now', new \DateTimeZone('Asia/Shanghai'));
        $dt->sub(new \DateInterval('P3D'));
        $dt->setTime(0, 0);
        $starttime = isset($json->starttime) && is_numeric($json->starttime) ? $json->starttime : $dt->getTimestamp();
        
        $dt->add(new \DateInterval('P3D'));
        $dt->setTime(23, 59, 59);
        $endtime = isset($json->endtime) && is_numeric($json->endtime) ? $json->endtime : $dt->getTimestamp();

        $selectFrom = $this->getSelectFrom($starttime, $endtime);

        $sn = isset($json->sn) ? $json->sn : null;
        $slot = isset($json->slot) ? $json->slot : null;
        $port = isset($json->port) ? $json->port : null;
        $porttype = isset($json->porttype) ? $json->porttype : null;
        $placeId = isset($json->placeId) ? $json->placeId : null;
        $mpoint = isset($json->mpoint) ? $json->mpoint : null;

        $where = $this->getWhere($sn, $slot, $port, $porttype, $placeId, $mpoint);
        
        $orderBy = $this->getOrderBy();
        
        $page = isset($json->page) && is_numeric($json->page) ? $json->page : 1;
        $pageSize = isset($json->pageSize) && is_numeric($json->pageSize) ? $json->pageSize : 10;

        $pagination = $this->getPagination($page, $pageSize);

        $em = $this->getDoctrine()->getManager();
        $conn = $em->getConnection();

        $stmt = $conn->prepare("SELECT COUNT(*) AS total FROM (" . $selectFrom . $where . ") AS t");
        $stmt->execute();
        $result['total'] = $stmt->fetch()['total'];

        $stmt = $conn->prepare($selectFrom . $where . $orderBy . $pagination);
        $stmt->execute();
        //$result['rows'] = $stmt->fetchAll();
        while ($row = $stmt->fetch()) {
            $row['starttime'] = $dt->setTimestamp($row['starttime'])->format('Y-m-d H:i:s');
            $row['endtime'] = $dt->setTimestamp($row['endtime'])->format('Y-m-d H:i:s');
            $row['status'] = CommonFunc::getMpointStatusByStatusCodeAndType($row['status'], $row['porttype']);
            $row['porttype'] = CommonFunc::toPorttype($row['porttype']);
            $result['rows'][] = $row;
        }

        $response = new Response(json_encode($result));
        $response->headers->set('Access-Control-Allow-Origin', '*');
        return $response;
    }

    public function exportCsv(Request $request)
    {
        $dt = new \DateTime('now', new \DateTimeZone('Asia/Shanghai'));
        $dt->sub(new \DateInterval('P3D'));
        $dt->setTime(0, 0);
        $starttime = $request->request->get('starttime');
        $starttime = is_numeric($starttime) ? $starttime : $dt->getTimestamp();
        
        $dt->add(new \DateInterval('P3D'));
        $dt->setTime(23, 59, 59);
        $endtime = $request->request->get('endtime');
        $endtime = is_numeric($endtime) ? $endtime : $dt->getTimestamp();

        $dt->setTimestamp($starttime);
        $filename = 'original-report_' . $dt->format('Y-m-d-H-i-s');
        $dt->setTimestamp($endtime);
        $filename .= '_' . $dt->format('Y-m-d-H-i-s') . '.csv';

        $selectFrom = $this->getSelectFrom($starttime, $endtime);

        $sn = $request->request->get('sn');
        $slot = $request->request->get('slot');
        $port = $request->request->get('port');
        $porttype = $request->request->get('porttype');
        $placeId = $request->request->get('placeId');
        $mpoint = $request->request->get('mpoint');

        $where = $this->getWhere($sn, $slot, $port, $porttype, $placeId, $mpoint);
        
        $orderBy = $this->getOrderBy();

        $em = $this->getDoctrine()->getManager();
        $conn = $em->getConnection();

        $stmt = $conn->prepare($selectFrom . $where . $orderBy);
        $stmt->execute();

        $response = new StreamedResponse(function () use ($stmt, $dt) {
            $output = fopen('php://output', 'w');

            fwrite($output, "\xEF\xBB\xBF");
            fputcsv($output, [
                '工厂', '车间', '区域', '线体', '工位', '监控点',
                'SN', '列号', '端口', '类型',
                '开始时间', '结束时间', '持续时间', '状态']);
            while ($row = $stmt->fetch()) {
                unset($row['id'], $row['plantid'], $row['workshopid'], $row['regionid'], $row['lineid'], $row['stationid']);
                $row['starttime'] = '="' . $dt->setTimestamp($row['starttime'])->format('Y-m-d H:i:s') . '"';
                $row['endtime'] = '="' . $dt->setTimestamp($row['endtime'])->format('Y-m-d H:i:s') . '"';
                $row['status'] = CommonFunc::getMpointStatusByStatusCodeAndType($row['status'], $row['porttype']);
                $row['porttype'] = CommonFunc::toPorttype($row['porttype']);
                $row['sn'] = '="' . $row['sn'] . '"';
                fputcsv($output, $row);
            }
        });

        $response->headers->set('Content-Type', 'text/csv; charset=utf-8');
        $response->headers->set('Content-Disposition', "attachment; filename=$filename");

        return $response;
    }

    private function getSelectFrom($starttime, $endtime) {
        return "
            SELECT
                id, plant, workshop, region, line, station, mpoint,
                sn, slot, port, porttype,
                starttime, endtime, endtime - starttime AS duration,
                status,
                plantid, workshopid, regionid, lineid, stationid
            FROM (
                SELECT
                    id, plant, workshop, region, line, station, mpoint,
                    sn, slot, port, porttype,
                    CASE
                        WHEN starttime < $starttime THEN $starttime
                        ELSE starttime
                    END AS starttime,
                    CASE
                        WHEN endtime IS NULL OR endtime > $endtime THEN $endtime
                        ELSE endtime
                    END AS endtime,
                    status,
                    plantid, workshopid, regionid, lineid, stationid
                FROM chan_status
                WHERE starttime < $endtime AND (endtime IS NULL OR endtime > $starttime)
            ) AS t
        ";
    }

    private function getWhere($sn, $slot, $port, $porttype, $placeId, $mpoint) {
        $where = 'WHERE 1 = 1';
        $where .= trim($sn) != '' ? ' AND sn = \'' . trim($sn) . '\'' : '';
        $where .= is_numeric($slot) ? ' AND slot = ' . $slot : '';
        $where .= is_numeric($port) ? ' AND port = ' . $port : '';
        $where .= is_numeric($porttype) ? ' AND porttype = ' . $porttype : '';
        $where .= is_numeric($placeId) ?
            ' AND (plantid = ' . $placeId . ' OR workshopid = ' . $placeId .
            ' OR regionid = ' . $placeId . ' OR lineid = ' . $placeId .
            ' OR stationid = ' . $placeId . ')' : '';
        $where .= trim($mpoint) != '' ? ' AND mpoint = \'' . trim($mpoint) . '\'' : '';

        return $where;
    }

    private function getOrderBy() {
        return "
            ORDER BY plant DESC, workshop, region, line, station, mpoint, sn, slot, port, porttype, starttime DESC
        ";
    }

    private function getPagination($page, $pageSize) {
        return "
            OFFSET ($page - 1) * $pageSize ROWS FETCH NEXT $pageSize ROWS ONLY
        ";
    }
}
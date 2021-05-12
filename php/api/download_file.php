<?php
header("Access-Control-Allow-Origin: http://localhost:3000");

// $filename = "test_file.cs";
// header("Content-Disposition: attachment; filename=\"$filename\"");
// header("Content-Type: text/csv");
$out = fopen('php://output', 'w');
fwrite($out, chr(0xEF).chr(0xBB).chr(0xBF));
fputcsv($out, ['日期', '类型', '工厂', '车间',
               '区域', '线体', '工位', '失效率', '合格率']);
fclose($out);

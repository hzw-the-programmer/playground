<?php
require "../db_config.php";

try {
    $start = microtime(true);
    $dbh = new PDO(DSN, USERNAME, PASSWD);
    $attrs = [
        //PDO::ATTR_AUTOCOMMIT,
        PDO::ATTR_CASE,
        PDO::ATTR_CLIENT_VERSION,
        //PDO::ATTR_CONNECTION_STATUS,
        PDO::ATTR_DRIVER_NAME,
        PDO::ATTR_ERRMODE,
        PDO::ATTR_ORACLE_NULLS,
        PDO::ATTR_PERSISTENT,
        //PDO::ATTR_PREFETCH,
        PDO::ATTR_SERVER_INFO,
        PDO::ATTR_SERVER_VERSION,
        //PDO::ATTR_TIMEOUT,
        PDO::ATTR_STRINGIFY_FETCHES,
        PDO::ATTR_STATEMENT_CLASS,
        //PDO::ATTR_EMULATE_PREPARES,
        //PDO::MYSQL_ATTR_USE_BUFFERED_QUERY,
        PDO::ATTR_DEFAULT_FETCH_MODE,
        PDO::SQLSRV_ATTR_QUERY_TIMEOUT,
        PDO::SQLSRV_ATTR_ENCODING,
    ];
    foreach ($attrs as $attr) {
        echo attrToStr($attr, $dbh->getAttribute($attr)) . "\n";
    }
} catch (PDOException $e) {
    echo $e->getMessage() . "\n";
} finally {
    echo (microtime(true) - $start) . "\n";
}

function attrToStr($attr, $val) {
    switch ($attr) {
    case PDO::ATTR_AUTOCOMMIT:
        $attr = "PDO::ATTR_AUTOCOMMIT";
        break;

    case PDO::ATTR_CASE:
        $attr = "PDO::ATTR_CASE";
        switch ($val) {
        case PDO::CASE_LOWER:
            $val = "PDO::CASE_LOWER";
            break;
        case PDO::CASE_NATURAL:
            $val = "PDO::CASE_NATURAL";
            break;
        case PDO::CASE_UPPER:
            $val = "PDO::CASE_UPPER";
            break;
        }
        break;

    case PDO::ATTR_CLIENT_VERSION:
        $attr = "PDO::ATTR_CLIENT_VERSION";
        $val = print_r($val, true);
        break;

    case PDO::ATTR_CONNECTION_STATUS:
        $attr = "PDO::ATTR_CONNECTION_STATUS";
        break;

    case PDO::ATTR_DRIVER_NAME:
        $attr = "PDO::ATTR_DRIVER_NAME";
        break;

    case PDO::ATTR_ERRMODE:
        $attr = "PDO::ATTR_ERRMODE";
        switch ($val) {
        case PDO::ERRMODE_SILENT:
            $val = "PDO::ERRMODE_SILENT";
            break;
        case PDO::ERRMODE_WARNING:
            $val = "PDO::ERRMODE_WARNING";
            break;
        case PDO::ERRMODE_EXCEPTION:
            $val = "PDO::ERRMODE_EXCEPTION";
            break;
        }

    case PDO::ATTR_ORACLE_NULLS:
        $attr = "PDO::ATTR_ORACLE_NULLS";
        switch ($val) {
        case PDO::NULL_NATURAL:
            $val = "PDO::NULL_NATURAL";
            break;
        case PDO::NULL_EMPTY_STRING:
            $val = "PDO::NULL_EMPTY_STRING";
            break;
        case PDO::NULL_TO_STRING:
            $val = "PDO::NULL_TO_STRING";
            break;
        }
        break;

    case PDO::ATTR_PERSISTENT:
        $attr = "PDO::ATTR_PERSISTENT";
        $val = $val ? "TRUE" : "FALSE";
        break;

    case PDO::ATTR_PREFETCH:
        $attr = "PDO::ATTR_PREFETCH";
        break;

    case PDO::ATTR_SERVER_INFO:
        $attr = "PDO::ATTR_SERVER_INFO";
        $val = print_r($val, true);
        break;

    case PDO::ATTR_SERVER_VERSION:
        $attr = "PDO::ATTR_SERVER_VERSION";
        break;

    case PDO::ATTR_TIMEOUT:
        $attr = "PDO::ATTR_TIMEOUT";
        break;

    case PDO::ATTR_STRINGIFY_FETCHES:
        $attr = "PDO::ATTR_STRINGIFY_FETCHES";
        $val = $val ? "TRUE" : "FALSE";
        break;
    
    case PDO::ATTR_STATEMENT_CLASS:
        $attr = "PDO::ATTR_STATEMENT_CLASS";
        $val = print_r($val, true);
        break;

    case PDO::ATTR_EMULATE_PREPARES:
        $attr = "PDO::ATTR_EMULATE_PREPARES";
        break;

    /*
    case PDO::MYSQL_ATTR_USE_BUFFERED_QUERY:
        $attr = "PDO::MYSQL_ATTR_USE_BUFFERED_QUERY";
        break;
    */

    case PDO::ATTR_DEFAULT_FETCH_MODE:
        $attr = "PDO::ATTR_DEFAULT_FETCH_MODE";
        switch ($val) {
        case PDO::FETCH_ASSOC:
            $val = "PDO::FETCH_ASSOC";
            break;
        case PDO::FETCH_BOTH:
            $val = "PDO::FETCH_BOTH";
            break;
        case PDO::FETCH_BOUND:
            $val = "PDO::FETCH_BOUND";
            break;
        case PDO::FETCH_CLASS:
            $val = "PDO::FETCH_CLASS";
            break;
        case PDO::FETCH_INTO:
            $val = "PDO::FETCH_INTO";
            break;
        case PDO::FETCH_LAZY:
            $val = "PDO::FETCH_LAZY:";
            break;
        case PDO::FETCH_NAMED:
            $val = "PDO::FETCH_NAMED";
            break;
        case PDO::FETCH_NUM:
            $val = "PDO::FETCH_NUM";
            break;
        case PDO::FETCH_OBJ:
            $val = "PDO::FETCH_OBJ";
            break;
        case PDO::FETCH_PROPS_LATE:
            $val = "PDO::FETCH_PROPS_LATE";
            break;
        }
        break;

    case PDO::SQLSRV_ATTR_QUERY_TIMEOUT:
        $attr = "PDO::SQLSRV_ATTR_QUERY_TIMEOUT";
        break;

    case PDO::SQLSRV_ATTR_ENCODING:
        $attr = "PDO::SQLSRV_ATTR_ENCODING";
        switch ($val) {
        case PDO::SQLSRV_ENCODING_UTF8:
            $val = "PDO::SQLSRV_ENCODING_UTF8";
            break;
        case PDO::SQLSRV_ENCODING_SYSTEM:
            $val = "PDO::SQLSRV_ENCODING_SYSTEM";
            break;
        }
        break;
    }

    return "$attr: $val";
}

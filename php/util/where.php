<?php
/*
$conditions = ['name', 'email', 'sex'];
printf("%s\n", getwhere($conditions));
function getwhere($conditions) {
    $where = '';

    foreach ($conditions as $condition) {
        if (!$where) {
            $where .= 'where';
        } else {
            $where .= ' and';
        }
        $where .= " $condition = :$condition";
    }

    return $where;
}
*/
/*
$conditions = ['name' => 'hzw', 'email' => 'hzw@163.com', 'sex' => 'm'];
printf("%s\n", getwhere($conditions));
function getwhere($conditions) {
    $where = '';

    foreach ($conditions as $key => $value) {
        if (!$where) {
            $where .= 'where';
        } else {
            $where .= ' and';
        }
        $where .= " $key = '$value'";
    }

    return $where;
}
*/
$name = '';
$email = 'hzw@123.com';
$sex = 'm';
$min_age = 32;
$max_age = 32;
$where = '';
$where = append($where, 'name', '=', $name);
$where = append($where, 'email', '=', $email);
$where = append($where, 'sex', '=', $sex);
$where = append($where, 'age', '>=', $min_age);
$where = append($where, 'age', '<=', $max_age);
printf("%s\n", $where);
function append($where, $key, $op, $value) {
    if (empty($value)) return $where;
    $isnum = is_numeric($value);
    if (!$where) $where .= 'where';
    else $where .= ' and';
    $where .= " $key $op ";
    if (!$isnum) $where .= "'";
    $where .= $value;
    if (!$isnum) $where .= "'";
    return $where;
}

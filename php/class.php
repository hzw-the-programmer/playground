<?php
class MyClass {
    public $a;
    private $b;
}

$obj = new MyClass();
var_dump($obj);

if (property_exists($obj, 'd')) {
    echo "property d is exists\n";
}

$obj->d = '';

if (property_exists($obj, 'a')) {
    echo "property a is exists\n";
}
if (property_exists($obj, 'b')) {
    echo "property b is exists\n";
}
if (property_exists($obj, 'c')) {
    echo "property c is exists\n";
}
if (property_exists($obj, 'd')) {
    echo "property d is exists\n";
}

if (!isset($obj->a)) {
    echo "a is not set\n";
}
if (!isset($obj->b)) {
    echo "b is not set\n";
}
if (!isset($obj->c)) {
    echo "c is not set\n";
}
if (!isset($obj->d)) {
    echo "d is not set\n";
}

if (empty($obj->a)) {
    echo "a is empty\n";
}
if (empty($obj->b)) {
    echo "b is empty\n";
}
if (empty($obj->c)) {
    echo "c is empty\n";
}
if (empty($obj->d)) {
    echo "d is empty\n";
}

if (!isset($obj->a[1])) {
    $obj->a[1] = "hello";
}
var_dump($obj);

unset($obj->a);
unset($obj->d);
//$obj->a = NULL;
var_dump($obj);

if (property_exists($obj, 'a')) {
    echo "property a is exists\n";
}
if (property_exists($obj, 'd')) {
    echo "property d is exists\n";
}

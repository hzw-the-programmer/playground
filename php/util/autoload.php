<?php
namespace n1;
//include "./people.php";
spl_autoload_register('n1\load');
$obj = new People;
$obj = new Person;
function load($name) {
    echo "load: $name\n";
    $name = "./" . implode("/", explode("\\", $name)) . ".php";
    echo "load: $name\n";
    include $name;
}

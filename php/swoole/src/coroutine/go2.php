<?php
go(function () {
    echo "1\n";
    go(function () {
        echo "2\n";
        co::sleep(1);
        echo "5\n";
    });
    echo "3\n";
});
echo "4\n";

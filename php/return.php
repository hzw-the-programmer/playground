<?php
function func1() {
    echo "begin try\n";
    try {
        echo "in try\n";
        return;
        echo "after return\n";
    } finally {
        echo "in finally\n";
    }
}

func1();

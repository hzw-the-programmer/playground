#!/bin/bash
DBNAMES="website_skeleton"
HOST="--host=localhost"
USER="--user=root"
PASSWORD="--password=asdF@123"
BACKUP_DIR="/tmp/bkup"

MAIN="__main.sql"
SCHEMA="__schema.sql"
OPTIONS="--no-create-info --complete-insert --compact -q"

DATE=`/bin/date '+%y%m%d_%H%M%S'`

if [ ! -d $BACKUP_DIR ]; then
    mkdir -p $BACKUP_DIR
fi

cd $BACKUP_DIR

echo removing old temporary files if they exists...
rm -fr *.sql > /dev/null 2>&1
rm -fr *.tar > /dev/null 2>&1

for DB in $DBNAMES
do
    echo "======================="
    echo $DB
    echo "======================="

    echo "SET FOREIGN_KEY_CHECKS=0;" > $MAIN

    mysqldump --no-data $HOST $USER $PASSWORD $DB > $SCHEMA
    echo "SOURCE $SCHEMA;" >> $MAIN

    for TB in `mysql $HOST $USER $PASSWORD $DB -e "SHOW TABLES" | egrep -v "Tables_in_"`
    do
        TBN=`echo $TB | awk '{ printf "%s", $0 }'`
        FN="$TBN.sql"
        echo Dumping $TBN
        mysqldump $OPTIONS $HOST $USER $PASSWORD $DB $TB >> $FN
        echo "SOURCE $FN;" >> $MAIN
    done

    echo "SET FOREIGN_KEY_CHECKS=1;" >> $MAIN

    echo making tar...
    tar -cf ${DB}_$DATE.tar *.sql > /dev/null 2>&1

    echo compressing...
    gzip -9 ${DB}_$DATE.tar > /dev/null 2>&1

    echo removing temporary files...
    rm -fr *.sql > /dev/null 2>&1
    rm -fr *.tar > /dev/null 2>&1

    echo "done with" $DB
done

echo "======================="
echo "done with all databases"
echo "======================="

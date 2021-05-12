sudo apt upgrade
composer create-project symfony/website-skeleton website-skeleton
cd website-skeleton
rm -fr .git
git add .
git commit
php -S 0.0.0.0:8000 -t public/
#composer require server --dev
php bin/console server:run
#php bin/console server:start 0.0.0.0:8000
composer require sec-checker --dev
composer require doctrine
composer require maker --dev

sudo swapon --show
free -h

sudo swapon -s

sudo fdisk -l
sudo blkid /dev/sda2
sudo gedit /etc/fstab

.env
DATABASE_URL=mysql://db_user:db_password@127.0.0.1:3306/db_name

php bin/console doctrine:database:create

#sudo apt install php-mysql
sudo apt install php7.0-mysql
php -i | grep mysql
php -m | grep mysql
php -m | grep sqlsrv
ls /usr/lib/php/20151012/
ls /etc/php/7.0/mods-available/

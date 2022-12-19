strawberry-perl-5.32.1.1-64bit-portable\portableshell.bat
cd /d d:
cd openssl
// perl Configure no-asm no-threads
perl Configure VC-WIN32 no-asm no-threads
// where nmake
nmake

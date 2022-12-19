cd /d D:\work\mtk6260
call make {customer} {project} gen_modis
pause

build\{customer}\log\MoDIS_setup\boringssl_setEnv_MoDIS.log

make\modisBuild.mak

MoDIS_VC9\createMoDIS.pm

```
create_lib_vcproj
create_Files_List
create_filter_tree
output_Source_File
```

MoDIS_VC9\createMoDIS.pl

```
open $LOG_HANDLE, ">$LOG_ROOT/gen_${modis_uesim}.log" or die "Fail to write $LOG_ROOT/gen_${modis_uesim}.log";
// MoDIS_VC9\_BUILD_LOG\gen_MoDIS.log
```

```
if (open $WRITE_HANDLE, ">$SLN_ROOT/lib_list${modis_suffix}")
// MoDIS_VC9\lib_list
{
	foreach my $lib (split('\s+', $infomake{'COMPLIST'}))
	{
		print $WRITE_HANDLE "$lib\n" if ($lib ne "modis");
		my $complis = "${compfolder}/${lib}.lis";
        // build\{customer}\module\MoDIS\boringssl\boringssl.lis
		if (open FILE_HANDLE, "<$complis")
		{
			my $line;
			while ($line = <FILE_HANDLE>)
			{
				chomp($line);
				$line =~ s/[\\\/]+/\//g;
				$line =~ s/^\.[\\\/]//;
				next if ($line eq "");
				$lib_source{$lib} .= " " if ($lib_source{$lib} ne "");
				$lib_source{$lib} .= $line;
				$lib_all_include .= " " if ($lib_all_include ne "");
				$lib_all_include .= dirname($line);
			}
			close FILE_HANDLE;
		}
		else
		{
			&error_handle("Fail to read $complis");
		}
	}
	close $WRITE_HANDLE;
}
```
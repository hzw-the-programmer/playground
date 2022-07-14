application\CoolSimulator\GenCoolSim_dsp.sh
```
cp -f application/CoolSimulator/IncOption.txt.module tempinc.txt
sed  's/$/\r/' tempinc.txt > application/CoolSimulator/IncOption.txt

cp -f application/CoolSimulator/CoolSimulator.vcxproj.module tempvc.vcxproj
sed  's/$/\r/' tempvc.vcxproj > application/CoolSimulator/CoolSimulator.vcxproj
```

application\CoolSimulator\CoolSimulator.vcxproj
```
  <ItemDefinitionGroup Condition="'$(Configuration)|$(Platform)'=='Debug|Win32'">
    <ClCompile>
      <AdditionalOptions> /Zm800 %40"TargetOption.txt" %40"IncOption.txt"</AdditionalOptions>
    </ClCompile>
    <Midl>
    </Midl>
    <ResourceCompile>
    </ResourceCompile>
    <Bscmake>
    </Bscmake>
    <Link>
      <AdditionalDependencies>./lib/gb_lt.lib;./lib/gb_sc.lib;./lib/gb_arabic_tran.lib;./lib/MediaTek_Hwre_demo.lib;./lib/VC6_x86_Dynamic_TimeLimit_HZRecog.lib;./lib/VC6_X86_Dynamic_TimeLimit_CHN_HZRecog.lib;./lib/WS2_32.LIB;./lib/kmxime_modis.lib;./lib/revlib.lib;./lib/revKeypad.lib;./lib/RevDictionary.lib;%(AdditionalDependencies)</AdditionalDependencies>
      <AdditionalOptions>/nodefaultlib:libc %(AdditionalOptions)</AdditionalOptions>
    </Link>
  </ItemDefinitionGroup>
```

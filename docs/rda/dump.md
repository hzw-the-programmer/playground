1. click coolwatch.exe
2. select 8809e2
3. lastcomport: 13
4. click ok

connection success, display:

Event sniffer (re)starting. (Connection: Event Sniffer (COM13))
Loading profile script : 8809e2.rb
EVENT SNIFFING ERROR: CONNECTION BROKEN?
EVENT SNIFFING RESURRECTED.

when crash happen, display:

Detected event: 0x91060001.
Detected event: 0x91060001.
Detected event: 0xddddaaaa.
Detected event: 0xddddaaaa.
Detected event: 0xddddaaaa.

5. enter cmd:
> elfdump("D:/dump/dump.elf","F:/cooltools/chipgen/Modem2G/toolpool/map/elfdump/8809e2.xml")
building elf for 8809e2...
Reading page reg @0x81a0c000...
done
Reading xcpu reg @0x81a2b000...
done
Reading bcpu reg @0x8190a000...
done
Reading RFSPI reg @0x81a0e000...
done
Reading tcu reg @0x81a0f000...
done
Reading sys ctrl reg @0x81a00000...
done
Reading dma reg @0x81a08000...
done
Reading sys ifc reg @0x81a09000...
done
Reading voc ahb reg @0x81970000...
done
Reading voc sram @0x81940000...
done
Reading internal sram @0x81c00000...
done
Reading bb sram @0x81980000...
done
Reading external sram @0x82000000...
done
Reading dual port sram @0x81b00000...
done
Reading internal rom @0x81e00000...
done
Reading bcpu rom @0x81e80000...
done
Skip reading flash
building D:/dump/dump.elf...
done

6. close coolwatcher
7. start coolhost
8. click start tcp server
8. select data tap
9. right click to add two files: dump.elf and flash.lod 
10. clock load
11. start coolwatch
12. select 8809e2, lastport 20, clock ok
display:
[COM OPEN OK] 
Event sniffer (re)starting. (Connection: Event Sniffer (COM20))
Loading profile script : 8809e2.rb
EVENT SNIFFING ERROR: CONNECTION BROKEN?
EVENT SNIFFING RESURRECTED.
13. tools -> GDB launcher
14. elf select flash.elf
15. click launch


Configurations/windows-makefile.tmpl
 join(' ', $lib_cppflags,
    (map { '-D'.quotify1($_) }
        "OPENSSLDIR=\"$openssldir\"",
        "ENGINESDIR=\"$enginesdir\"",
        "MODULESDIR=\"$modulesdir\""),
    '$(CNF_CPPFLAGS)', '$(CPPFLAGS)') -}

NOCRYPT
MODULESDIR=\"Mhzw\"
OPENSSLDIR=\"Ohzw\"
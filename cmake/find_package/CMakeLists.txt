find_package(Perl)
IF(NOT PERL_FOUND)
    MESSAGE(FATAL_ERROR "Perl not found -- need it to generate source code")
ENDIF()
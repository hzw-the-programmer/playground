%.o %.d :%.c $(COMPONENT).dep
	@$(ECHO)
	@$(ECHO) Compiling Source File $<...
ifeq ($(strip $(OPEN_MODULE_SUPPORT)),TRUE)
	@-$(PERL) make/perl_script/rm_file.pl $(TARGET) $(BUILD_DIR)/lib/
else
	@-$(PERL) make/perl_script/rm_file.pl $(TARGET) lib/$(PROJECT)/
endif
	@-$(PERL) make/perl_script/rm_file.pl $*.o $(MOBJPATH)/
#	$(ECHO) $(CCOMPLAN) $(CFLAGS)  $(VIA) $(BUILD_DIR)/dep/$(COMPONENT)_C_MACRO_INC.txt -c   -I$(ARMINC)  $<  $(MDFLAGS) $(DEPPATH)/$*.d.tmp -o $(MOBJPATH)/$*.o
	$(CCOMPLAN) $(CFLAGS)  $(VIA) $(BUILD_DIR)/dep/$(COMPONENT)_C_MACRO_INC.txt -c   -I$(ARMINC)  $<  $(MDFLAGS) $(DEPPATH)/$*.d.tmp -o $(MOBJPATH)/$*.o  \
             $(if $(findstring 0,$(STOP)),, || $(ECHO) && $(ECHO) !!! Error !!! You can terminate batch job by Control+C !  && $(ECHO) && pause)
	@$(PERL) ./make/perl_script/path_process.pl ./$(DEPPATH)/$*.d.tmp  $(BUILD_DIR)/dep/$(COMPONENT).dep "$(MAKESHELL)" > ./$(DEPPATH)/$*.d
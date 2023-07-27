%.a:
	@$(ECHO)
	@$(ECHO) Building library $* \( refer to $(BUILD_DIR)/log/$*.log for detail \), Please Wait ... 
	@-$(PERL) make/perl_script/getsecond.pl >./$(BUILD_DIR)/tmp/$*_start.txt
	@$(PERL) make/perl_script/mk_dir.pl "$*" $(BUILD_DIR)/obj
	@$(PERL) make/perl_script/mk_dir.pl "$*" $(BUILD_DIR)/dep
	@$(MAKE) PROJECT=$(PROJECT) -f make/pclint/pclint.mk -k -r -R COMPONENT=$* moduledep
	@$(PERL) make/perl_script/createfile.pl $(BUILD_DIR)/dep/$*.dep
	@$(PERL) make/perl_script/diff.pl $(BUILD_DIR)/dep/$*.dep.tmp $(BUILD_DIR)/dep/$*.dep $(BUILD_DIR)/dep/$*.cmp
	@$(PERL) ./make/perl_script/module_dep_process.pl  $(BUILD_DIR) $*  $(subst \,/,$(MAKE_HOME))
	@$(PERL) make/perl_script/rm_file.pl $(BUILD_DIR)/dep/$*.cmp ""

ifeq ($(strip $(STOP)),0)
	@$(MAKE)  -s $(JOB_SUM) PROJECT=$(PROJECT)  -f Makefile.modules  -I ./$(BUILD_DIR)/dep/$* -I ./$(BUILD_DIR)/dep/$*/cache -k -r -R COMPONENT=$* 2>&1 | $(TEE)  $(BUILD_DIR)/log/$*.log
else
	@$(MAKE)  $(JOB_SUM) PROJECT=$(PROJECT)  -f Makefile.modules  -I ./$(BUILD_DIR)/dep/$* -r -R COMPONENT=$* 
endif
	@$(ECHO) \n$* library build finished  
	@-$(PERL) make/perl_script/getcompiletime.pl $* ./$(BUILD_DIR)/tmp/$*_start.txt  2>&1 | $(TEE) -a $(BUILD_DIR)/log/time_consuming.log
	@$(PERL) make/perl_script/rm_file.pl ./$(BUILD_DIR)/tmp/$*_start.txt ""
#	@-$(PERL) make/perl_script/gen_loadrules.pl p=$(PROJECT) m=$* $(if $(strip $(IB)),ib=1,)
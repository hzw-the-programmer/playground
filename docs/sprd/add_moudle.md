project_project_240X320BAR_64MB.mk
```
# *************************************************************************
# Feature Options (For User)
# !!!!!!!! The customer value can be modified in this option part.!!!!!!!!!

PICOTLS_SUPPORT = TRUE  ###  TLS1.3 Support
                        # Option0:TRUE
                        # Option1:FALSE

#****************************************************************************************#
#  Segment: SPRD MACRO
```

make/picotls/picotls.mk
```
ifeq ($(strip $(PICOTLS_SUPPORT)), TRUE)
PICOTLS_ROOT = Third-party

CFLAGS += --c99 --gnu
MCFLAG_OPT += -D__SPRD_PORTING__

MINCPATH  += $(PICOTLS_ROOT)/picotls/deps/cifra/src \
        $(PICOTLS_ROOT)/picotls/deps/cifra/src/ext \
        $(PICOTLS_ROOT)/picotls/deps/micro-ecc \
        $(PICOTLS_ROOT)/picotls/include

MSRCPATH += $(PICOTLS_ROOT)/picotls/deps/cifra/src \
        $(PICOTLS_ROOT)/picotls/deps/micro-ecc \
        $(PICOTLS_ROOT)/picotls/lib \
        $(PICOTLS_ROOT)/picotls/lib/cifra

SOURCES += uECC.c
SOURCES += aes.c
SOURCES += blockwise.c
SOURCES += chacha20.c
SOURCES += chash.c
SOURCES += curve25519.c
SOURCES += drbg.c
SOURCES += hmac.c
SOURCES += gcm.c
SOURCES += gf128.c
SOURCES += modes.c
SOURCES += poly1305.c
SOURCES += sha256.c
SOURCES += sha512.c

SOURCES += cifra.c
SOURCES += x25519.c
SOURCES += chacha20_wrapper.c
SOURCES += aes128.c
SOURCES += aes256.c
SOURCES += random.c
SOURCES += minicrypto-pem.c
SOURCES += uecc_wrapper.c
SOURCES += asn1.c
SOURCES += ffx.c

SOURCES += hpke.c
SOURCES += picotls.c
SOURCES += pembase64.c
endif
```

PROJECT.modules
```
#****************************************************************************************#
#  Segment: COMPLIBS
#  Description: The Following Libs Will Be Compiled By Default (Using Make PROJECT update or New)
#****************************************************************************************#

ifeq ($(strip $(LETSCHAT_SUPPORT)),TRUE)
COMPLIBS_DBG += picotls
endif

#****************************************************************************************#
#  Segment: LINKLIBLIST
#  Description: Libs Need By The Final Target
#               Custom Can Add Lib or Obj files
#****************************************************************************************#
```
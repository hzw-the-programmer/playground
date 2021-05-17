#include "stdint.h"
#include "stdbool.h"
#include "protobuf-c.h"

typedef struct _ScannedMember ScannedMember;
struct _ScannedMember {
	uint32_t tag;
	uint8_t wire_type;
	uint8_t length_prefix_len;
	size_t len;
	const uint8_t *data;
};

extern bool pb_scanned_member(size_t len, const uint8_t *buf, ScannedMember *sm);

#include <assert.h>
#include <stdint.h>

#define PB_LABEL_NONE 0
#define PB_LABEL_REPEATED 1
#define PB_LABEL_REQUIRED 2
#define PB_LABEL_OPTIONAL 3
#define PB_LABEL_MASK 0x03

#define PB_FLAG_PACKED (1 << 2)
#define PB_FLAG_DEPRECATED (2 << 2)
#define PB_FLAG_ONEOF (3 << 2)
#define PB_FLAG_MASK 0x0c

#define PB_TYPE_INT32 (1 << 4)
#define PB_TYPE_SINT32 (2 << 4)
#define PB_TYPE_SFIXED32 (3 << 4)
#define PB_TYPE_INT64 (4 << 4)
#define PB_TYPE_SINT64 (5 << 4)
#define PB_TYPE_SFIXED64 (6 << 4)
#define PB_TYPE_UINT32 (7 << 4)
#define PB_TYPE_FIXED32 (8 << 4)
#define PB_TYPE_UINT64 (9 << 4)
#define PB_TYPE_FIXED64 (10 << 4)
#define PB_TYPE_FLOAT (11 << 4)
#define PB_TYPE_DOUBLE (12 << 4)
#define PB_TYPE_BOOL (13 << 4)
#define PB_TYPE_ENUM (14 << 4)
#define PB_TYPE_STRING (15 << 4)
#define PB_TYPE_BYTES (16 << 4)
#define PB_TYPE_MESSAGE (17 << 4)
#define PB_TYPE_MASK 0xfff0

typedef int PbBool;

typedef struct PbBinaryData {
	size_t	len;
	uint8_t	*data;
} PbBinaryData;

typedef struct PbFieldDescriptor {
	uint32_t		id;
	unsigned		quantifier_offset;
	unsigned		offset;
	const void		*descriptor; 
	uint16_t		flag;
} PbFieldDescriptor;

typedef struct PbStructDescriptor {
	size_t				sizeof_message;
	unsigned			n_fields;
	const PbFieldDescriptor	*fields;
} PbStructDescriptor;

extern size_t pb_get_packed_size(const void *message, const PbStructDescriptor *descriptor);

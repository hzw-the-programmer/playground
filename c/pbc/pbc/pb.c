#include <string.h>

#include "pb.h"

#define TRUE 1
#define FALSE 0

#define PB__ASSERT_NOT_REACHED() assert(0)

static PbBool field_is_zeroish(const PbFieldDescriptor *field, const void *member) {
    uint16_t type = field->flag & PB_FLAG_MASK;

	switch (type) {
	case PB_TYPE_INT32:
    case PB_TYPE_ENUM:
	case PB_TYPE_UINT32:
	case PB_TYPE_SINT32:
	case PB_TYPE_SFIXED32:
	case PB_TYPE_FIXED32:
		return 0 == *(const uint32_t *) member;
	
    case PB_TYPE_INT64:
	case PB_TYPE_UINT64:
    case PB_TYPE_SINT64:
	case PB_TYPE_SFIXED64:
	case PB_TYPE_FIXED64:
		return 0 == *(const uint64_t *) member;
    
    case PB_TYPE_BOOL:
		return 0 == *(const PbBool *) member;
	
    case PB_TYPE_FLOAT:
		return 0 == *(const float *) member;
	
    case PB_TYPE_DOUBLE:
		return 0 == *(const double *) member;
	
    case PB_TYPE_STRING:
		return (NULL == *(const char * const *) member) || ('\0' == **(const char * const *) member);
	
    case PB_TYPE_BYTES:
    case PB_TYPE_MESSAGE:
		return NULL == *(const void * const *) member;
	}

	return TRUE;
}

static uint32_t zigzag32(int32_t v) {
	return ((uint32_t)(v) << 1) ^ (uint32_t)(v >> 31);
}

static uint64_t zigzag64(int64_t v) {
	return ((uint64_t)(v) << 1) ^ (uint64_t)(v >> 63);
}

static size_t int32_size(int32_t v) {
	if (v < 0) {
		return 10;
	} else if (v < (1L << 7)) {
		return 1;
	} else if (v < (1L << 14)) {
		return 2;
	} else if (v < (1L << 21)) {
		return 3;
	} else if (v < (1L << 28)) {
		return 4;
	} else {
		return 5;
	}
}

static size_t uint32_size(uint32_t v) {
	if (v < (1UL << 7)) {
		return 1;
	} else if (v < (1UL << 14)) {
		return 2;
	} else if (v < (1UL << 21)) {
		return 3;
	} else if (v < (1UL << 28)) {
		return 4;
	} else {
		return 5;
	}
}

static size_t sint32_size(int32_t v) {
	return uint32_size(zigzag32(v));
}

static size_t uint64_size(uint64_t v) {
	uint32_t upper_v = (uint32_t) (v >> 32);

	if (upper_v == 0) {
		return uint32_size((uint32_t) v);
	} else if (upper_v < (1UL << 3)) {
		return 5;
	} else if (upper_v < (1UL << 10)) {
		return 6;
	} else if (upper_v < (1UL << 17)) {
		return 7;
	} else if (upper_v < (1UL << 24)) {
		return 8;
	} else if (upper_v < (1UL << 31)) {
		return 9;
	} else {
		return 10;
	}
}

static size_t sint64_size(int64_t v) {
	return uint64_size(zigzag64(v));
}

static size_t get_tag_size(uint32_t id) {
	if (id < (1UL << 4)) {
		return 1;
	} else if (id < (1UL << 11)) {
		return 2;
	} else if (id < (1UL << 18)) {
		return 3;
	} else if (id < (1UL << 25)) {
		return 4;
	} else {
		return 5;
	}
}

static size_t required_field_get_packed_size(const PbFieldDescriptor *field, const void *member) {
	size_t rv = get_tag_size(field->id);
    uint16_t type = field->flag & PB_TYPE_MASK;

	switch (type) {
    case PB_TYPE_INT32:
	case PB_TYPE_ENUM:
		return rv + int32_size(*(const int32_t *) member);
	case PB_TYPE_UINT32:
		return rv + uint32_size(*(const uint32_t *) member);
	case PB_TYPE_SINT32:
		return rv + sint32_size(*(const int32_t *) member);
	
    case PB_TYPE_SFIXED32:
	case PB_TYPE_FIXED32:
		return rv + 4;

    case PB_TYPE_INT64:
	case PB_TYPE_UINT64:
		return rv + uint64_size(*(const uint64_t *) member);
    case PB_TYPE_SINT64:
		return rv + sint64_size(*(const int64_t *) member);

    case PB_TYPE_SFIXED64:
	case PB_TYPE_FIXED64:
		return rv + 8;
	
    case PB_TYPE_BOOL:
		return rv + 1;

    case PB_TYPE_FLOAT:
		return rv + 4;

    case PB_TYPE_DOUBLE:
		return rv + 8;

	case PB_TYPE_STRING: {
		const char *str = *(char * const *) member;
		size_t len = str ? strlen(str) : 0;
		return rv + uint32_size(len) + len;
	}

    case PB_TYPE_BYTES: {
		size_t len = ((const PbBinaryData *) member)->len;
		return rv + uint32_size(len) + len;
	}

    case PB_TYPE_MESSAGE: {
		const void *msg = *(void * const *) member;
		size_t subrv = msg ? pb_get_packed_size(msg, field->descriptor) : 0;
		return rv + uint32_size(subrv) + subrv;
	}
	}
	
    PB__ASSERT_NOT_REACHED();
	return 0;
}

static size_t unlabeled_field_get_packed_size(const PbFieldDescriptor *field, const void *member) {
	if (field_is_zeroish(field, member))
		return 0;
	return required_field_get_packed_size(field, member);
}

static size_t repeated_field_get_packed_size(const PbFieldDescriptor *field, size_t count, const void *member) {
	size_t rv = 0;
    size_t header_size;
	unsigned i;
	void *array = *(void * const *) member;
    const uint16_t type = field->flag | PB_TYPE_MASK;
    const uint16_t packed = (field->flag | PB_FLAG_MASK) == PB_FLAG_PACKED;

	if (count == 0)
		return 0;

	switch (type) {
    case PB_TYPE_INT32:
    case PB_TYPE_ENUM:
		for (i = 0; i < count; i++)
			rv += int32_size(((int32_t *) array)[i]);
		break;
    case PB_TYPE_UINT32:
		for (i = 0; i < count; i++)
			rv += uint32_size(((uint32_t *) array)[i]);
		break;
	case PB_TYPE_SINT32:
		for (i = 0; i < count; i++)
			rv += sint32_size(((int32_t *) array)[i]);
		break;

    case PB_TYPE_SFIXED32:
	case PB_TYPE_FIXED32:
		rv += 4 * count;
		break;

    case PB_TYPE_INT64:
	case PB_TYPE_UINT64:
		for (i = 0; i < count; i++)
			rv += uint64_size(((uint64_t *) array)[i]);
		break;
	case PB_TYPE_SINT64:
		for (i = 0; i < count; i++)
			rv += sint64_size(((int64_t *) array)[i]);
		break;
	
    case PB_TYPE_SFIXED64:
	case PB_TYPE_FIXED64:
		rv += 8 * count;
		break;
	
    case PB_TYPE_BOOL:
		rv += count;
		break;

    case PB_TYPE_FLOAT:
		rv += 4 * count;
		break;

	case PB_TYPE_DOUBLE:
		rv += 8 * count;
		break;

    case PB_TYPE_STRING:
		for (i = 0; i < count; i++) {
			size_t len = strlen(((char **) array)[i]);
			rv += uint32_size(len) + len;
		}
		break;
	
    case PB_TYPE_BYTES:
		for (i = 0; i < count; i++) {
			size_t len = ((PbBinaryData *) array)[i].len;
			rv += uint32_size(len) + len;
		}
		break;
	
    case PB_TYPE_MESSAGE:
		for (i = 0; i < count; i++) {
			size_t len = pb_get_packed_size(((void **) array)[i], field->descriptor);
			rv += uint32_size(len) + len;
		}
		break;
	}

    header_size = get_tag_size(field->id);		
    if (packed) {
		header_size += uint32_size(rv);
    } else {
        header_size *= count;
    }
	
    return  rv + header_size;
}

size_t pb_get_packed_size(const void *message, const PbStructDescriptor *descriptor) {
	unsigned i;
	size_t rv = 0;

	for (i = 0; i < descriptor->n_fields; i++) {
		const PbFieldDescriptor *field = descriptor->fields + i;
		const void *member = ((const char *) message) + field->offset;
		const void *qmember = ((const char *) message) + field->quantifier_offset;
        const uint16_t label = field->flag & PB_LABEL_MASK;

		if (label == PB_LABEL_NONE) {
            rv += unlabeled_field_get_packed_size(field, member);
		} else {
			rv += repeated_field_get_packed_size(field, *(const size_t *) qmember, member);
		}
	}

	return rv;
}

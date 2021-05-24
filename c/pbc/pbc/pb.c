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

static void copy_to_little_endian_32(void *out, const void *in, const unsigned n) {
#if !defined(WORDS_BIGENDIAN)
	memcpy(out, in, n * 4);
#else
	unsigned i;
	const uint32_t *ini = in;
	for (i = 0; i < n; i++)
		fixed32_pack(ini[i], (uint32_t *) out + i);
#endif
}

static void copy_to_little_endian_64(void *out, const void *in, const unsigned n) {
#if !defined(WORDS_BIGENDIAN)
	memcpy(out, in, n * 8);
#else
	unsigned i;
	const uint64_t *ini = in;
	for (i = 0; i < n; i++)
		fixed64_pack(ini[i], (uint64_t *) out + i);
#endif
}

static size_t sizeof_elt_in_repeated_array(uint16_t type) {
	switch (type) {
    case PB_TYPE_INT32:
    case PB_TYPE_ENUM:
    case PB_TYPE_UINT32:
	case PB_TYPE_SINT32:
	case PB_TYPE_SFIXED32:
	case PB_TYPE_FIXED32:
	case PB_TYPE_FLOAT:
		return 4;

	case PB_TYPE_INT64:
	case PB_TYPE_UINT64:
    case PB_TYPE_SINT64:
	case PB_TYPE_SFIXED64:
	case PB_TYPE_FIXED64:
	case PB_TYPE_DOUBLE:
		return 8;
	
    case PB_TYPE_BOOL:
		return sizeof(PbBool);
	
    case PB_TYPE_STRING:
	case PB_TYPE_MESSAGE:
		return sizeof(void *);
	
    case PB_TYPE_BYTES:
		return sizeof(PbBinaryData);
	}
	
    PB__ASSERT_NOT_REACHED();
	return 0;
}

static unsigned get_type_min_size(uint16_t type) {
	if (type == PB_TYPE_SFIXED32 ||
	    type == PB_TYPE_FIXED32 ||
	    type == PB_TYPE_FLOAT) {
		return 4;
	}
	
    if (type == PB_TYPE_SFIXED64 ||
	    type == PB_TYPE_FIXED64 ||
	    type == PB_TYPE_DOUBLE) {
		return 8;
	}
	
    return 1;
}

static size_t uint32_pack(uint32_t value, uint8_t *out) {
	unsigned rv = 0;

	if (value >= 0x80) {
		out[rv++] = value | 0x80;
		value >>= 7;
		if (value >= 0x80) {
			out[rv++] = value | 0x80;
			value >>= 7;
			if (value >= 0x80) {
				out[rv++] = value | 0x80;
				value >>= 7;
				if (value >= 0x80) {
					out[rv++] = value | 0x80;
					value >>= 7;
				}
			}
		}
	}

    out[rv++] = value;

    return rv;
}

static size_t int32_pack(int32_t value, uint8_t *out) {
	if (value < 0) {
		out[0] = value | 0x80;
		out[1] = (value >> 7) | 0x80;
		out[2] = (value >> 14) | 0x80;
		out[3] = (value >> 21) | 0x80;
		out[4] = (value >> 28) | 0x80;
		out[5] = out[6] = out[7] = out[8] = 0xff;
		out[9] = 0x01;
		return 10;
	} else {
		return uint32_pack(value, out);
	}
}

static size_t sint32_pack(int32_t value, uint8_t *out) {
	return uint32_pack(zigzag32(value), out);
}

static size_t fixed32_pack(uint32_t value, void *out) {
#if !defined(WORDS_BIGENDIAN)
	memcpy(out, &value, 4);
#else
	uint8_t *buf = out;

	buf[0] = value;
	buf[1] = value >> 8;
	buf[2] = value >> 16;
	buf[3] = value >> 24;
#endif
	return 4;
}

static size_t uint64_pack(uint64_t value, uint8_t *out) {
	uint32_t hi = (uint32_t) (value >> 32);
	uint32_t lo = (uint32_t) value;
	unsigned rv;

    if (hi == 0) {
    	return uint32_pack((uint32_t) lo, out);
    }
	
    out[0] = (lo) | 0x80;
	out[1] = (lo >> 7) | 0x80;
	out[2] = (lo >> 14) | 0x80;
	out[3] = (lo >> 21) | 0x80;

    if (hi < 8) {
		out[4] = (hi << 4) | (lo >> 28);
		return 5;
	} else {
		out[4] = ((hi & 7) << 4) | (lo >> 28) | 0x80;
		hi >>= 3;
	}
	rv = 5;
	while (hi >= 128) {
		out[rv++] = hi | 0x80;
		hi >>= 7;
	}
	out[rv++] = hi;
	return rv;
}

static size_t sint64_pack(int64_t value, uint8_t *out) {
	return uint64_pack(zigzag64(value), out);
}

static size_t fixed64_pack(uint64_t value, void *out) {
#if !defined(WORDS_BIGENDIAN)
	memcpy(out, &value, 8);
#else
	fixed32_pack(value, out);
	fixed32_pack(value >> 32, ((char *) out) + 4);
#endif
	return 8;
}

static size_t boolean_pack(PbBool value, uint8_t *out) {
	*out = value ? TRUE : FALSE;
	return 1;
}

static size_t string_pack(const char *str, uint8_t *out) {
	if (str == NULL) {
		out[0] = 0;
		return 1;
	} else {
		size_t len = strlen(str);
		size_t rv = uint32_pack(len, out);
		memcpy(out + rv, str, len);
		return rv + len;
	}
}

static size_t binary_data_pack(const PbBinaryData *bd, uint8_t *out) {
	size_t len = bd->len;
	size_t rv = uint32_pack(len, out);
	memcpy(out + rv, bd->data, len);
	return rv + len;
}

static size_t prefixed_message_pack(const void *message, const PbStructDescriptor *descriptor, uint8_t *out) {
	if (message == NULL) {
		out[0] = 0;
		return 1;
	} else {
		size_t rv = pb_pack(message, descriptor, out + 1);
		uint32_t rv_packed_size = uint32_size(rv);
		if (rv_packed_size != 1)
			memmove(out + rv_packed_size, out + 1, rv);
		return uint32_pack(rv, out) + rv;
	}
}

static size_t tag_pack(uint32_t id, uint8_t *out) {
	if (id < (1UL << (32 - 3)))
		return uint32_pack(id << 3, out);
	else
		return uint64_pack(((uint64_t) id) << 3, out);
}

static size_t required_field_pack(const PbFieldDescriptor *field, const void *member, uint8_t *out) {
	size_t rv = tag_pack(field->id, out);
    const uint16_t type = field->flag & PB_TYPE_MASK;

	switch (type) {
    case PB_TYPE_INT32:
	case PB_TYPE_ENUM:
		out[0] |= PB_WIRE_TYPE_VARINT;
		return rv + int32_pack(*(const int32_t *) member, out + rv);
    case PB_TYPE_UINT32:
		out[0] |= PB_WIRE_TYPE_VARINT;
		return rv + uint32_pack(*(const uint32_t *) member, out + rv);
    case PB_TYPE_SINT32:
		out[0] |= PB_WIRE_TYPE_VARINT;
		return rv + sint32_pack(*(const int32_t *) member, out + rv);

    case PB_TYPE_SFIXED32:
	case PB_TYPE_FIXED32:
	case PB_TYPE_FLOAT:
		out[0] |= PB_WIRE_TYPE_32BIT;
		return rv + fixed32_pack(*(const uint32_t *) member, out + rv);

    case PB_TYPE_INT64:
	case PB_TYPE_UINT64:
		out[0] |= PB_WIRE_TYPE_VARINT;
		return rv + uint64_pack(*(const uint64_t *) member, out + rv);
	case PB_TYPE_SINT64:
		out[0] |= PB_WIRE_TYPE_VARINT;
		return rv + sint64_pack(*(const int64_t *) member, out + rv);
	
    case PB_TYPE_SFIXED64:
	case PB_TYPE_FIXED64:
	case PB_TYPE_DOUBLE:
        out[0] |= PB_WIRE_TYPE_64BIT;
		return rv + fixed64_pack(*(const uint64_t *) member, out + rv);
	
    case PB_TYPE_BOOL:
		out[0] |= PB_WIRE_TYPE_VARINT;
		return rv + boolean_pack(*(const PbBool *) member, out + rv);
	
    case PB_TYPE_STRING:
		out[0] |= PB_WIRE_TYPE_LENGTH_PREFIXED;
		return rv + string_pack(*(char *const *) member, out + rv);
	
    case PB_TYPE_BYTES:
		out[0] |= PB_WIRE_TYPE_LENGTH_PREFIXED;
		return rv + binary_data_pack((const PbBinaryData *) member, out + rv);
	
    case PB_TYPE_MESSAGE:
		out[0] |= PB_WIRE_TYPE_LENGTH_PREFIXED;
        return rv + prefixed_message_pack(*(void * const *) member, field->descriptor, out + rv);
	}
	
    PB__ASSERT_NOT_REACHED();
	return 0;
}

static size_t unlabeled_field_pack(const PbFieldDescriptor *field, const void *member, uint8_t *out) {
    if (field_is_zeroish(field, member)) {
    	return 0;
    }

    return required_field_pack(field, member, out);
}

static size_t repeated_field_pack(const PbFieldDescriptor *field, size_t count, const void *member, uint8_t *out) {
	void *array = *(void * const *) member;
	unsigned i;
    const uint16_t type = field->flag & PB_TYPE_MASK;
    const uint16_t packed = (field->flag & PB_FLAG_MASK) == PB_FLAG_PACKED;

    unsigned header_len;
	unsigned len_start;
	unsigned min_length;
	unsigned payload_len;
	unsigned length_size_min;
	unsigned actual_length_size;
	uint8_t *payload_at;

    if (count == 0) {
    	return 0;
    }

    if (!packed) {
        size_t rv = 0;
		unsigned siz = sizeof_elt_in_repeated_array(type);

		for (i = 0; i < count; i++) {
			rv += required_field_pack(field, array, out + rv);
			array = (char *)array + siz;
		}
		
        return rv;
    }

    header_len = tag_pack(field->id, out);
	out[0] |= PB_WIRE_TYPE_LENGTH_PREFIXED;
	
    len_start = header_len;
	
    min_length = get_type_min_size(type) * count;
	length_size_min = uint32_size(min_length);
	header_len += length_size_min;
	
    payload_at = out + header_len;

	switch (type) {
    case PB_TYPE_INT32:
	case PB_TYPE_ENUM: {
		const int32_t *arr = (const int32_t *) array;
		for (i = 0; i < count; i++)
			payload_at += int32_pack(arr[i], payload_at);
		break;
	}

	case PB_TYPE_UINT32: {
		const uint32_t *arr = (const uint32_t *) array;
		for (i = 0; i < count; i++)
			payload_at += uint32_pack(arr[i], payload_at);
		break;
	}

    case PB_TYPE_SINT32: {
		const int32_t *arr = (const int32_t *) array;
		for (i = 0; i < count; i++)
			payload_at += sint32_pack(arr[i], payload_at);
		break;
	}
	
    case PB_TYPE_SFIXED32:
	case PB_TYPE_FIXED32:
	case PB_TYPE_FLOAT:
		copy_to_little_endian_32(payload_at, array, count);
		payload_at += count * 4;
		break;

	case PB_TYPE_INT64:
	case PB_TYPE_UINT64: {
		const uint64_t *arr = (const uint64_t *) array;
		for (i = 0; i < count; i++)
			payload_at += uint64_pack(arr[i], payload_at);
		break;
	}

	case PB_TYPE_SINT64: {
		const int64_t *arr = (const int64_t *) array;
		for (i = 0; i < count; i++)
			payload_at += sint64_pack(arr[i], payload_at);
		break;
	}

    case PB_TYPE_SFIXED64:
	case PB_TYPE_FIXED64:
	case PB_TYPE_DOUBLE:
		copy_to_little_endian_64(payload_at, array, count);
		payload_at += count * 8;
		break;

	case PB_TYPE_BOOL: {
		const PbBool *arr = (const PbBool *) array;
		for (i = 0; i < count; i++)
			payload_at += boolean_pack(arr[i], payload_at);
		break;
	}
	
    default:
		PB__ASSERT_NOT_REACHED();
	}

	payload_len = payload_at - (out + header_len);
	
    actual_length_size = uint32_size(payload_len);
	if (length_size_min != actual_length_size) {
		assert(actual_length_size == length_size_min + 1);
		memmove(out + header_len + 1, out + header_len,
			payload_len);
		header_len++;
	}
	
    uint32_pack(payload_len, out + len_start);
	
    return header_len + payload_len;
}

size_t pb_pack(const void *message, const PbStructDescriptor *descriptor, uint8_t *out) {
	unsigned i;
	size_t rv = 0;

	for (i = 0; i < descriptor->n_fields; i++) {
		const PbFieldDescriptor *field = descriptor->fields + i;
		const void *member = ((const char *) message) + field->offset;
		const void *qmember = ((const char *) message) + field->quantifier_offset;
        const uint16_t label = field->flag & PB_LABEL_MASK;

        if (label == PB_LABEL_NONE) {
			rv += unlabeled_field_pack(field, member, out + rv);
		} else {
			rv += repeated_field_pack(field, *(const size_t *) qmember, member, out + rv);
		}
	}

    return rv;
}

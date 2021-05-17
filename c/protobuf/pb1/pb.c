#include "pb.h"

size_t parse_tag_and_wiretype(size_t len,
		       const uint8_t *data,
		       uint32_t *tag_out,
		       ProtobufCWireType *wiretype_out)
{
	unsigned max_rv = len > 5 ? 5 : len;
	uint32_t tag = (data[0] & 0x7f) >> 3;
	unsigned shift = 4;
	unsigned rv;

	if ((data[0] & 0xf8) == 0) {
		return 0;
	}

	*wiretype_out = data[0] & 7;
	if ((data[0] & 0x80) == 0) {
		*tag_out = tag;
		return 1;
	}

    for (rv = 1; rv < max_rv; rv++) {
		if (data[rv] & 0x80) {
			tag |= (data[rv] & 0x7f) << shift;
			shift += 7;
		} else {
			tag |= data[rv] << shift;
			*tag_out = tag;
			return rv + 1;
		}
	}
	
    return 0;
}

size_t scan_length_prefixed_data(size_t len, const uint8_t *data,
			  size_t *prefix_len_out)
{
	unsigned hdr_max = len < 5 ? len : 5;
	unsigned hdr_len;
	size_t val = 0;
	unsigned i;
	unsigned shift = 0;

	for (i = 0; i < hdr_max; i++) {
		val |= ((size_t)data[i] & 0x7f) << shift;
		shift += 7;
		if ((data[i] & 0x80) == 0)
			break;
	}
	if (i == hdr_max) {
		return 0;
	}
	hdr_len = i + 1;
	*prefix_len_out = hdr_len;
	if (val > INT_MAX) {
		return 0;
	}
	if (hdr_len + val > len) {
		return 0;
	}
	return hdr_len + val;
}

bool pb_scanned_member(size_t len, const uint8_t *buf, ScannedMember *sm) {
    uint32_t tag;
    ProtobufCWireType wire_type;
    size_t used;

    if (len < 1) {
        return false;
    }

    used = parse_tag_and_wiretype(len, buf, &tag, &wire_type);

    if (!used) {
        return false;
    }

    buf += used;
    len -= used;

    sm->tag = tag;
    sm->wire_type = wire_type;
    sm->data = buf;
    sm->length_prefix_len = 0;

    if (wire_type == PROTOBUF_C_WIRE_TYPE_VARINT) {
        unsigned max_len = len < 10 ? len : 10;
		unsigned i;

		for (i = 0; i < max_len; i++)
			if ((buf[i] & 0x80) == 0)
				break;

        if (i == max_len) {
            return false;
        }
		
        sm->len = i + 1;
		return true;
    }
		
    if (wire_type == PROTOBUF_C_WIRE_TYPE_LENGTH_PREFIXED) {
		size_t pref_len;

		sm->len = scan_length_prefixed_data(len, buf, &pref_len);
		if (sm->len == 0) {
			return false;
		}
		
        sm->length_prefix_len = pref_len;
		return true;
	}
	
    if (wire_type ==  PROTOBUF_C_WIRE_TYPE_32BIT) {
		if (len < 4) {
			return false;
		}
		
        sm->len = 4;
		return true;
	}

    if (wire_type == PROTOBUF_C_WIRE_TYPE_64BIT) {
        if (len < 8) {
			return false;
		}
		
        sm->len = 8;
		return true;
    }

    return false;
}

char* pb_string(ScannedMember *sm) {
    size_t len = sm->len - sm->length_prefix_len;
    char *str = malloc(len + 1);
    memcpy(str, sm->data + sm->length_prefix_len, len);
    str[len] = 0;
    return str;
}

/* Generated by the protocol buffer compiler.  DO NOT EDIT! */
/* Generated from: test2.proto */

/* Do not generate deprecated warnings for self */
#ifndef PROTOBUF_C__NO_DEPRECATED
#define PROTOBUF_C__NO_DEPRECATED
#endif

#include "test2.pb-c.h"
size_t a__get_packed_size
                     (const A *message)
{
  return protobuf_c_message_get_packed_size ((const void*)message, &a__descriptor);
}
size_t a__pack
                     (const A *message,
                      uint8_t       *out)
{
  return protobuf_c_message_pack ((const void*)message, &a__descriptor, out);
}
A *
       a__unpack
                     (ProtobufCAllocator  *allocator,
                      size_t               len,
                      const uint8_t       *data)
{
  return (A *)
     protobuf_c_message_unpack (&a__descriptor,
                                allocator, len, data);
}
void   a__free_unpacked
                     (A *message,
                      ProtobufCAllocator *allocator)
{
  if(!message)
    return;
  protobuf_c_message_free_unpacked ((void*)message, &a__descriptor, allocator);
}
size_t b__get_packed_size
                     (const B *message)
{
  return protobuf_c_message_get_packed_size ((const void*)message, &b__descriptor);
}
size_t b__pack
                     (const B *message,
                      uint8_t       *out)
{
  return protobuf_c_message_pack ((const void*)message, &b__descriptor, out);
}
B *
       b__unpack
                     (ProtobufCAllocator  *allocator,
                      size_t               len,
                      const uint8_t       *data)
{
  return (B *)
     protobuf_c_message_unpack (&b__descriptor,
                                allocator, len, data);
}
void   b__free_unpacked
                     (B *message,
                      ProtobufCAllocator *allocator)
{
  if(!message)
    return;
  protobuf_c_message_free_unpacked ((void*)message, &b__descriptor, allocator);
}
size_t c__get_packed_size
                     (const C *message)
{
  return protobuf_c_message_get_packed_size ((const void*)message, &c__descriptor);
}
size_t c__pack
                     (const C *message,
                      uint8_t       *out)
{
  return protobuf_c_message_pack ((const void*)message, &c__descriptor, out);
}
C *
       c__unpack
                     (ProtobufCAllocator  *allocator,
                      size_t               len,
                      const uint8_t       *data)
{
  return (C *)
     protobuf_c_message_unpack (&c__descriptor,
                                allocator, len, data);
}
void   c__free_unpacked
                     (C *message,
                      ProtobufCAllocator *allocator)
{
  if(!message)
    return;
  protobuf_c_message_free_unpacked ((void*)message, &c__descriptor, allocator);
}
static const ProtobufCFieldDescriptor a__field_descriptors[] =
{
  {
    1,
    0,   /* quantifier_offset */
    offsetof(A, str),
    NULL,
    PROTOBUF_C_FIELD_FLAG_VTF(0) | PROTOBUF_C_LABEL_VTF(PROTOBUF_C_LABEL_OPTIONAL) | PROTOBUF_C_TYPE_VTF(PROTOBUF_C_TYPE_STRING)             /* flags */
  },
};
const ProtobufCMessageDescriptor a__descriptor =
{
  sizeof(A),
  sizeof(a__field_descriptors) / sizeof(ProtobufCFieldDescriptor),
  a__field_descriptors,
};
static const ProtobufCFieldDescriptor b__field_descriptors[] =
{
  {
    1,
    0,   /* quantifier_offset */
    offsetof(B, i),
    NULL,
    PROTOBUF_C_FIELD_FLAG_VTF(0) | PROTOBUF_C_LABEL_VTF(PROTOBUF_C_LABEL_REQUIRED) | PROTOBUF_C_TYPE_VTF(PROTOBUF_C_TYPE_INT32)             /* flags */
  },
};
const ProtobufCMessageDescriptor b__descriptor =
{
  sizeof(B),
  sizeof(b__field_descriptors) / sizeof(ProtobufCFieldDescriptor),
  b__field_descriptors,
};
static const ProtobufCFieldDescriptor c__field_descriptors[] =
{
  {
    1,
    0,   /* quantifier_offset */
    offsetof(C, a),
    &a__descriptor,
    PROTOBUF_C_FIELD_FLAG_VTF(0) | PROTOBUF_C_LABEL_VTF(PROTOBUF_C_LABEL_REQUIRED) | PROTOBUF_C_TYPE_VTF(PROTOBUF_C_TYPE_MESSAGE)             /* flags */
  },
  {
    2,
    0,   /* quantifier_offset */
    offsetof(C, b),
    &b__descriptor,
    PROTOBUF_C_FIELD_FLAG_VTF(0) | PROTOBUF_C_LABEL_VTF(PROTOBUF_C_LABEL_OPTIONAL) | PROTOBUF_C_TYPE_VTF(PROTOBUF_C_TYPE_MESSAGE)             /* flags */
  },
};
const ProtobufCMessageDescriptor c__descriptor =
{
  sizeof(C),
  sizeof(c__field_descriptors) / sizeof(ProtobufCFieldDescriptor),
  c__field_descriptors,
};

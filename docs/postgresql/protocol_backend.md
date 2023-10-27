### BindComplete (B) 

Byte1('2')
Identifies the message as a Bind-complete indicator.

Int32(4)
Length of message contents in bytes, including self.

### ErrorResponse (B) 

Byte1('E')
Identifies the message as an error.

Int32
Length of message contents in bytes, including self.

The message body consists of one or more identified fields, followed by a zero byte as a terminator. Fields can appear in any order. For each field there is the following:

Byte1
A code identifying the field type; if zero, this is the message terminator and no string follows. The presently defined field types are listed in Section 55.8. Since more field types might be added in future, frontends should silently ignore fields of unrecognized type.

String
The field value.

### CommandComplete (B) 

Byte1('C')
Identifies the message as a command-completed response.

Int32
Length of message contents in bytes, including self.

String
The command tag. This is usually a single word that identifies which SQL command was completed.

For an INSERT command, the tag is INSERT oid rows, where rows is the number of rows inserted. oid used to be the object ID of the inserted row if rows was 1 and the target table had OIDs, but OIDs system columns are not supported anymore; therefore oid is always 0.

For a DELETE command, the tag is DELETE rows where rows is the number of rows deleted.

For an UPDATE command, the tag is UPDATE rows where rows is the number of rows updated.

For a MERGE command, the tag is MERGE rows where rows is the number of rows inserted, updated, or deleted.

For a SELECT or CREATE TABLE AS command, the tag is SELECT rows where rows is the number of rows retrieved.

For a MOVE command, the tag is MOVE rows where rows is the number of rows the cursor's position has been changed by.

For a FETCH command, the tag is FETCH rows where rows is the number of rows that have been retrieved from the cursor.

For a COPY command, the tag is COPY rows where rows is the number of rows copied. (Note: the row count appears only in PostgreSQL 8.2 and later.)

### ReadyForQuery (B) 

Byte1('Z')
Identifies the message type. ReadyForQuery is sent whenever the backend is ready for a new query cycle.

Int32(5)
Length of message contents in bytes, including self.

Byte1
Current backend transaction status indicator. Possible values are 'I' if idle (not in a transaction block); 'T' if in a transaction block; or 'E' if in a failed transaction block (queries will be rejected until block is ended).

### RowDescription (B) 

Byte1('T')
Identifies the message as a row description.

Int32
Length of message contents in bytes, including self.

Int16
Specifies the number of fields in a row (can be zero).

Then, for each field, there is the following:

String
The field name.

Int32
If the field can be identified as a column of a specific table, the object ID of the table; otherwise zero.

Int16
If the field can be identified as a column of a specific table, the attribute number of the column; otherwise zero.

Int32
The object ID of the field's data type.

Int16
The data type size (see pg_type.typlen). Note that negative values denote variable-width types.

Int32
The type modifier (see pg_attribute.atttypmod). The meaning of the modifier is type-specific.

Int16
The format code being used for the field. Currently will be zero (text) or one (binary). In a RowDescription returned from the statement variant of Describe, the format code is not yet known and will always be zero.

### DataRow (B) 

Byte1('D')
Identifies the message as a data row.

Int32
Length of message contents in bytes, including self.

Int16
The number of column values that follow (possibly zero).

Next, the following pair of fields appear for each column:

Int32
The length of the column value, in bytes (this count does not include itself). Can be zero. As a special case, -1 indicates a NULL column value. No value bytes follow in the NULL case.

Byten
The value of the column, in the format indicated by the associated format code. n is the above length.

### EmptyQueryResponse (B) 

Byte1('I')
Identifies the message as a response to an empty query string. (This substitutes for CommandComplete.)

Int32(4)
Length of message contents in bytes, including self.

### ParseComplete (B) 

Byte1('1')
Identifies the message as a Parse-complete indicator.

Int32(4)
Length of message contents in bytes, including self.

### ParameterDescription (B) 

Byte1('t')
Identifies the message as a parameter description.

Int32
Length of message contents in bytes, including self.

Int16
The number of parameters used by the statement (can be zero).

Then, for each parameter, there is the following:

Int32
Specifies the object ID of the parameter data type.

### CloseComplete (B) 

Byte1('3')
Identifies the message as a Close-complete indicator.

Int32(4)
Length of message contents in bytes, including self.
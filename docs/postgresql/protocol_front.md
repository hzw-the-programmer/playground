### Bind (F) 

Byte1('B')
Identifies the message as a Bind command.

Int32
Length of message contents in bytes, including self.

String
The name of the destination portal (an empty string selects the unnamed portal).

String
The name of the source prepared statement (an empty string selects the unnamed prepared statement).

Int16
The number of parameter format codes that follow (denoted C below). This can be zero to indicate that there are no parameters or that the parameters all use the default format (text); or one, in which case the specified format code is applied to all parameters; or it can equal the actual number of parameters.

Int16[C]
The parameter format codes. Each must presently be zero (text) or one (binary).

Int16
The number of parameter values that follow (possibly zero). This must match the number of parameters needed by the query.

Next, the following pair of fields appear for each parameter:

Int32
The length of the parameter value, in bytes (this count does not include itself). Can be zero. As a special case, -1 indicates a NULL parameter value. No value bytes follow in the NULL case.

Byten
The value of the parameter, in the format indicated by the associated format code. n is the above length.

After the last parameter, the following fields appear:

Int16
The number of result-column format codes that follow (denoted R below). This can be zero to indicate that there are no result columns or that the result columns should all use the default format (text); or one, in which case the specified format code is applied to all result columns (if any); or it can equal the actual number of result columns of the query.

Int16[R]
The result-column format codes. Each must presently be zero (text) or one (binary).

### Execute (F)

Byte1('E')
Identifies the message as an Execute command.

Int32
Length of message contents in bytes, including self.

String
The name of the portal to execute (an empty string selects the unnamed portal).

Int32
Maximum number of rows to return, if portal contains a query that returns rows (ignored otherwise). Zero denotes “no limit”.

### Sync (F) 

Byte1('S')
Identifies the message as a Sync command.

Int32(4)
Length of message contents in bytes, including self.

### Parse (F) 

Byte1('P')
Identifies the message as a Parse command.

Int32
Length of message contents in bytes, including self.

String
The name of the destination prepared statement (an empty string selects the unnamed prepared statement).

String
The query string to be parsed.

Int16
The number of parameter data types specified (can be zero). Note that this is not an indication of the number of parameters that might appear in the query string, only the number that the frontend wants to prespecify types for.

Then, for each parameter, there is the following:

Int32
Specifies the object ID of the parameter data type. Placing a zero here is equivalent to leaving the type unspecified.

### Describe (F) 

Byte1('D')
Identifies the message as a Describe command.

Int32
Length of message contents in bytes, including self.

Byte1
'S' to describe a prepared statement; or 'P' to describe a portal.

String
The name of the prepared statement or portal to describe (an empty string selects the unnamed prepared statement or portal).

### Sync (F) 

Byte1('S')
Identifies the message as a Sync command.

Int32(4)
Length of message contents in bytes, including self.

### Close (F) 

Byte1('C')
Identifies the message as a Close command.

Int32
Length of message contents in bytes, including self.

Byte1
'S' to close a prepared statement; or 'P' to close a portal.

String
The name of the prepared statement or portal to close (an empty string selects the unnamed prepared statement or portal).

### Query (F) 

Byte1('Q')
Identifies the message as a simple query.

Int32
Length of message contents in bytes, including self.

String
The query string itself.
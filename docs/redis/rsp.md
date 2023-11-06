https://redis.io/docs/reference/protocol-spec/#simple-strings

Simple strings
+OK\r\n

Simple errors
-Error message\r\n

-ERR unknown command 'asdf'
-WRONGTYPE Operation against a key holding the wrong kind of value

Integers
:[<+|->]<value>\r\n
:0\r\n
:1000\r\n

Bulk strings
$<length>\r\n<data>\r\n
$5\r\nhello\r\n
$0\r\n\r\n
$-1\r\n

Arrays
*<number-of-elements>\r\n<element-1>...<element-n>
*0\r\n
*2\r\n$5\r\nhello\r\n$5\r\nworld\r\n
*3\r\n:1\r\n:2\r\n:3\r\n

*5\r\n
:1\r\n
:2\r\n
:3\r\n
:4\r\n
$5\r\n
hello\r\n

*2\r\n
*3\r\n
:1\r\n
:2\r\n
:3\r\n
*2\r\n
+Hello\r\n
-World\r\n

*-1\r\n

*3\r\n
$5\r\n
hello\r\n
$-1\r\n
$5\r\n
world\r\n

Nulls
_\r\n

Booleans
#<t|f>\r\n

Doubles
,[<+|->]<integral>[.<fractional>][<E|e>[sign]<exponent>]\r\n

,1.23\r\n

:10\r\n
,10\r\n

,inf\r\n
,-inf\r\n
,nan\r\n

Big numbers
([+|-]<number>\r\n

(3492890328409238509324850943850943825024385\r\n

Bulk errors
!<length>\r\n<error>\r\n

!21\r\n
SYNTAX invalid syntax\r\n

Verbatim strings
=<length>\r\n<encoding>:<data>\r\n

=15\r\n
txt:Some string\r\n

Maps
%<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n>

{
    "first": 1,
    "second": 2
}

%2\r\n
+first\r\n
:1\r\n
+second\r\n
:2\r\n

Sets
~<number-of-elements>\r\n<element-1>...<element-n>

Pushes
><number-of-elements>\r\n<element-1>...<element-n>


Client handshake

HELLO <protocol-version> [optional-arguments]

Client: HELLO 4
Server: -NOPROTO sorry, this protocol version is not supported.

Client: HELLO 3
Server: -ERR unknown command 'HELLO'

Client: HELLO 3 AUTH default mypassword
Server: -ERR invalid password
(the connection remains in RESP2 mode)

server: "redis" (or other software name).
version: the server's version.
proto: the highest supported version of the RESP protocol.

id: the connection's identifier (ID).
mode: "standalone", "sentinel" or "cluster".
role: "master" or "replica".
modules: list of loaded modules as an Array of Bulk Strings.

Sending commands to a Redis server
C: *2\r\n
C: $4\r\n
C: LLEN\r\n
C: $6\r\n
C: mylist\r\n

S: :48293\r\n

Inline commands

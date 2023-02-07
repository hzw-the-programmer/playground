https://www.developershome.com/sms/operatingMode.asp#:~:text=They%20are%20called%20SMS%20text,the%20responses%20returned%20after%20execution.

###
Let's say you would like to send the SMS message "It is easy to send text messages." to the mobile phone number +85291234567.

In SMS text mode, this is the command line that you should enter:
AT+CMGS="+85291234567"<CR>It is easy to send text messages.<Ctrl+z>

if the GSM/GPRS modem or mobile phone is operating in SMS PDU mode:
AT+CMGS=42<CR>07915892000000F001000B915892214365F7000021493A283D0795C3F33C88FE06CDCB6E32885EC6D341EDF27C1E3E97E72E<Ctrl+z>

###
Suppose you would like to list all SMS messages from message storage. 

If the GSM/GPRS modem or mobile phone is operating in SMS text mode
AT+CMGL="ALL"

In SMS PDU mode, the numeric value 4 should be assigned to the +CMGL AT command instead:
AT+CMGL=4

###
https://www.gsmfavorites.com/documents/sms/pdutext/#:~:text=Introduction%20to%20SMS%20PDU%20Mode,in%20the%20binary%20bit%20stream.

07
Length of the SMSC information (in this case 7 octets)

91
Type-of-address of the SMSC. (91 means international format of the phone number)

72 38 01 00 10 F5
Service center number(in decimal semi-octets). The length of the phone  number is odd (11), so a trailing F has been added to form proper octets. The  phone number of this service center is "+27831000015". See below.

04
First octet of this SMS-DELIVER message.

0B
Address-Length. Length of the sender number (0B hex = 11 dec)

C8
Type-of-address of the sender number

72 38 88 09 00 F1
Sender number (decimal semi-octets), with a trailing F

00
TP-PID. Protocol identifier.

00
TP-DCS Data coding scheme

99 30 92 51 61 95 80
TP-SCTS. Time stamp (semi-octets)

0A
TP-UDL. User data length, length of message. The TP-DCS field indicated  7-bit data, so the length here is the number of septets (10). If the TP-DCS  field were set to indicate 8-bit data or Unicode, the length would be the number of octets (9).

E8329BFD4697D9EC37
TP-UD. Message "hellohello" , 8-bit octets representing 7-bit data.

All the octets above are hexa-decimal 8-bit octets,  except the Service center number, the sender number and the timestamp; they are  decimal semi-octets. The message part in the end of the  PDU string consists of  hexa-decimal 8-bit octets, but these octets represent 7-bit data (see below). The semi-octets are decimal, and e.g. the sender number is obtained by performing internal swapping within the semi-octets from "72 38 88 09 00 F1" to "27 83 88 90 00 1F". The length of the phone number is odd, so a proper octet sequence cannot be formed by this number. This is the reason why the trailing F has been added. The time stamp, when parsed, equals "99 03 29 15 16 59 08",  where the 6 first characters represent date, the following 6 represents time, and the last two represents time-zone related to GMT.

AT+CMGF=0 //Set PDU mode
AT+CSMS=0 //Check if modem supports SMS commands
AT+CMGS=23 //Send message, 23 octets (excluding the two initial zeros) >0011000B916407281553F80000AA0AE8329BFD4697D9EC37

00
Length of SMSC information. Here the length is 0, which means that the SMSC stored in the phone should be used. Note: This octet is optional. On some  phones this octet should be omitted! (Using the SMSC stored in phone is thus implicit)

11 (00010001)
First octet of the SMS-SUBMIT message.

00
TP-Message-Reference. The "00" value here lets the phone set the message  reference number itself.

0B
Address-Length. Length of phone number (11)

91
Type-of-Address. (91 indicates international format of the phone number).

6407281553F8
The phone number in semi octets (46708251358). The length of the phone  number is odd (11), therefore a trailing F has been added, as if the phone number were "46708251358F". Using the unknown format (i.e. the Type-of-Address 81 instead of 91) would yield the phone number octet sequence 7080523185 (0708251358). Note that this has the length 10 (A), which is even.

00
TP-PID. Protocol identifier

00
TP-DCS. Data coding scheme.This message is coded according to the 7bit default alphabet. Having "02" instead of "00" here, would indicate that the TP-User-Data field of this message should be interpreted as 8bit rather than 7bit (used in e.g. smart messaging, OTA provisioning etc).

AA
TP-Validity-Period. "AA" means 4 days. Note: This octet is optional, see bits 4 and 3 of the first  octet

0A
TP-User-Data-Length. Length of message. The TP-DCS field indicated 7-bit  data, so the length here is the number of septets (10). If the TP-DCS field were  set to 8-bit data or Unicode, the length would be the number of octets.

E8329BFD4697D9EC3 7
TP-User-Data. These octets represent the message "hellohello". How to do the  transformation from 7bit septets into octets is shown here


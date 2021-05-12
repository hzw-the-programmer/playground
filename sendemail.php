<?php
require './vendor/autoload.php';

//Create a new PHPMailer instance
$mail = new PHPMailer;
//Tell PHPMailer to use SMTP
$mail->isSMTP();
//Enable SMTP debugging
// 0 = off (for production use)
// 1 = client messages
// 2 = client and server messages
$mail->SMTPDebug = 2;
//Ask for HTML-friendly debug output
$mail->Debugoutput = 'html';
//Set the hostname of the mail server
$mail->Host = "mail.kaifa.cn";
//Enable TLS encryption, `ssl` also accepted
$mail->SMTPSecure = 'tls';
//Set the SMTP port number - likely to be 25, 465 or 587
$mail->Port = 587;
//Whether to use SMTP authentication
$mail->SMTPAuth = true;
//Username to use for SMTP authentication
$mail->Username = "zhiwenhe@kaifa.cn";
//Password to use for SMTP authentication
$mail->Password = "hzw@123";
//Set who the message is to be sent from
$mail->setFrom('zhiwenhe@kaifa.cn', 'ZhiWen HE');
//Set an alternative reply-to address
$mail->addReplyTo('xingpingliu@kaifa.cn', 'XingPing Liu');
//Set who the message is to be sent to
$mail->addAddress('zhiwenhe@kaifa.cn', 'ZhiWen He');
//Set the subject line
$mail->Subject = 'PHPMailer SMTP test';
//Read an HTML message body from an external file, convert referenced images to embedded,
//convert HTML into a basic plain-text alternative body
$mail->msgHTML(file_get_contents('easyui-layout-01.html'), dirname(__FILE__));
//Replace the plain text body with one created manually
$mail->AltBody = 'This is a plain-text message body';
//Attach an image file
$mail->addAttachment('images/msn.gif');
//send the message, check for errors
if (!$mail->send()) {
    echo "Mailer Error: " . $mail->ErrorInfo;
} else {
    echo "Message sent!";
}
?>

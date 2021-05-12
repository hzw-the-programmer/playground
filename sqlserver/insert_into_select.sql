SET IDENTITY_INSERT FOS_USER ON
INSERT INTO FOS_USER (
id, username, username_canonical, email, email_canonical,
enabled, salt, password, last_login, confirmation_token, password_requested_at, roles
)
SELECT * FROM iot.dbo.FOS_USER

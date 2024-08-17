# PhiLogin
SessionToken By Web or QRcode

# Introduction
A HTTP Server for API to Login Phigros By Taptap

# Use
`./PhiLogin 127.0.0.1:3000`

# API
The {} is Placeholder, Please remove it.
```
get("/login").body("{device_id}")
{"device_code":"nieMOsKTp","expires_in":300,"qrcode_url":"https://accounts.taptap.cn/device?qrcode=1&user_code=hycvj"}
```
device_id: Redmi Note 5, Nova 6(5G), etc

device_code: use at get_token

qrcode_url: Give the url to user, open it on Taptap APP or Web Browser.
```
get("/token/{device_code}").body("{device_id}")
{"code":-1,"error":"authorization_pending","error_description":"oauth2.tapapis.com.AUTHORIZATION_PENDING: InvalidArgument: the end-user authorization is pending","msg":"oauth2.tapapis.com.AUTHORIZATION_PENDING: InvalidArgument: the end-user authorization is pending"}
{"sessionToken":"?"}
```
return code:-1 until user complete qrcode_url

then return sessionToken

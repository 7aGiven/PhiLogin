# PhiLogin
SessionToken By Web or QRcode

# Introduction
A HTTP Server for API to Login Phigros By Taptap

# API
The {} is Placeholder, Please remove it.
```
get("/login").body("{device_id}")
{"device_code":"?","qrcode_url":"?"}
```
device_id: Redmi Note 5, Nova 6(5G), etc

device_code: use at get_token

qrcode_url: Give the url to user, open it on Taptap APP or Web Browser.
```
get("/token/{device_code}").body("{device_id}")
{"code":-1}
{"sessionToken":"?"}
```
return code:-1 until user complete qrcode_url

then return sessionToken

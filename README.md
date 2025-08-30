# Micro Monitoring Application

Application which checks whether a webpage is down and when this is the case, sends out an email.

## Compile

Download Rust and then build the binary:

```
cargo build --release
```

## Configure

You will need to following variables in an .env file:

```
WEBSITE=https://brahmakumaris1.de
EMAIL_RECIPIENTS=<recipient>
EMAIL_HOST=<host>
EMAIL_PORT=465
EMAIL_SSL_ENABLE=true
EMAIL_SSL_PROTOCOLS=TLSv1.2
EMAIL_USERNAME=<username>
EMAIL_PASSWORD=<password>
EMAIL_FROM=<email>
EMAIL_SUBJECT="Website {website} is in trouble"
EMAIL_HTML="Website <a href='{website}'>{website}</a> is in trouble. <br />Please go and check."
```

## Run

Start the micro monitor with this command:

```
.\target\release\micro-monitor.exe
```
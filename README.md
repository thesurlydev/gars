# Google Authenticator for Rust (GARS)

If you use Google Authenticator for multi-factor authentication (MFA) and you'd like to automate or simplify things, then GARS is for you.


## Usage

1. Create an environment variable with your Google Authenticator secret: `export GA_SECRET=YOUR_GA_SECRET_HERE`
2. Run `gars`

`gars` will give you the latest Google Authenticator generated number associated with the given GA secret.


## Why was GARS created?

MFA is great for added security but makes it difficult to automate things.


## What is Google Authenticator?

Google Authenticator is a software-based authenticator by Google that implements two-step verification services using the Time-based One-time Password Algorithm and HMAC-based One-time Password algorithm, for authenticating users of software applications. 

It's also one of several mechanisms that Okta uses for MFA.

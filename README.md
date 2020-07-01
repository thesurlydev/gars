# Google Authenticator for Rust (GARS)

If you use [Google Authenticator](https://github.com/google/google-authenticator) for multi-factor authentication (MFA) and would like to automate authentication, then GARS is for you.

## Usage

Create an environment variable with your Google Authenticator secret:

```bash
export GA_SECRET=YOUR_GA_SECRET_HERE
```

Run `gars` from the command line.

```bash
$ gars
123456
```

`gars` will return the latest Google Authenticator generated number associated with the given GA secret to stdout.

Tested on Ubuntu Linux and Mac OS X.

## Why was GARS created

MFA is great for added security but makes it difficult to automate things.

## What is Google Authenticator

Google Authenticator is a software-based authenticator by Google that implements two-step verification services using the Time-based One-time Password Algorithm and HMAC-based One-time Password algorithm, for authenticating users of software applications.

It's also one of several mechanisms that [Okta](https://www.okta.com/) uses for MFA.

#!/usr/bin/env python3
"""
Asks the user for auth info and contacts API & extracts required cookies to a JSON file for tests
"""

import urllib.parse
import base64
import http.client
import re
import json
import getpass

API_KEY = "JlE5Jldo5Jibnk5O5hTx6XVqsJu4WJ26"
USER_AGENT = "vrc_rs-get_auth.py/0.0.0 (at ljoonal)"

AUTH_TOKEN = getpass.getpass("Already got auth token? (empty if not): ")
SECOND_FACTOR_TOKEN = None
REQUIRED_SECOND_FACTOR_TYPE = None

if AUTH_TOKEN:
    REQUIRED_SECOND_FACTOR_TYPE = input("2FA type ('emailotp'/'totp'/'otp'=recovery): ")
else:
    username = urllib.parse.quote_plus(input("Username: "))
    password = urllib.parse.quote_plus(getpass.getpass("Password: "))
    user_and_pass = username + ":" + password
    BASIC_AUTH = base64.urlsafe_b64encode(user_and_pass.encode("utf-8")).decode("utf-8")

    conn = http.client.HTTPSConnection("api.vrchat.cloud")
    basic_auth_headers = { "User-Agent": USER_AGENT, "Authorization" : "Basic " + BASIC_AUTH }
    #print("Sending request")
    conn.request("GET", "/api/1/auth/user", headers=basic_auth_headers)
    #print("Reading response")
    resp = conn.getresponse()
    print("Response " + str(resp.status) + ": " + resp.reason)
    #print("Headers:")
    headers = resp.getheaders()
    #print(headers)
    #print("Body:")
    data = resp.read()
    #print(data)
    json_data = json.loads(data)

    if json_data["requiresTwoFactorAuth"]:
        if "emailOtp" in json_data["requiresTwoFactorAuth"]:
            REQUIRED_SECOND_FACTOR_TYPE = "emailotp"
        else:
            REQUIRED_SECOND_FACTOR_TYPE = "totp"

    # No handling for multiple set-cookie headers...yet.
    auth_cookie = next((
        x for x in headers if x[0].lower() == "set-cookie"
    ), None)
    AUTH_TOKEN = re.search("auth=(.+?)(;|$)", auth_cookie[1], re.IGNORECASE).group(1)

if REQUIRED_SECOND_FACTOR_TYPE:
    code_request = json.dumps({"code": input("2FA code: ") })

    auth_headers = {
        "Content-Type": "application/json",
        "User-Agent": USER_AGENT,
        "Cookie" : "apiKey=" + API_KEY + "; auth=" + AUTH_TOKEN
    }
    #print("Sending request")
    conn = http.client.HTTPSConnection("api.vrchat.cloud")
    conn.request(
        "POST",
        "/api/1/auth/twofactorauth/" + REQUIRED_SECOND_FACTOR_TYPE + "/verify",
        body=code_request,
        headers=auth_headers
    )
    #print("Reading response")
    second_factor_resp = conn.getresponse()
    print("Response " + str(second_factor_resp.status) + ": " + second_factor_resp.reason)
    #print("Headers:")
    second_factor_resp_headers = second_factor_resp.getheaders()
    #print(second_factor_resp_headers)
    #print("Body:")
    second_factor_resp_data = second_factor_resp.read()
    print(second_factor_resp_data)

    # No handling for multiple set-cookie headers...yet.
    second_factor_cookie = next((
        x for x in second_factor_resp_headers if x[0].lower() == "set-cookie"
    ), None)
    SECOND_FACTOR_TOKEN = re.search(
        "twoFactorAuth=(.+?)(;|$)",
        second_factor_cookie[1],
        re.IGNORECASE
    ).group(1)

with open("user-auth.json", "w", encoding="UTF8") as auth_file:
    auth_file.write(json.dumps({
        "token":  AUTH_TOKEN,
        "second_factor_token": SECOND_FACTOR_TOKEN
    }, indent=2))

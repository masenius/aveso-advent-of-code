#!/usr/bin/env python

import md5

pwd_input = "ojvtpuvg"
pwd = ""
done = False

i = 0
while done is False:
    pwd_test = pwd_input + str(i)
    h = md5.new(pwd_test).hexdigest()
    if h[:5] == '00000':
        pwd = pwd + h[5]
        print h, pwd_test, pwd
        if len(pwd) > 7:
            done = True
    i += 1

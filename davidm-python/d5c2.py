#!/usr/bin/env python

import md5

pwd_input = "ojvtpuvg"
pwd = list("XXXXXXXX")
done = False

print "Decrypting: " + ''.join(pwd)

i=0
while done == False:
    pwd_test = pwd_input + str(i)
    h = md5.new(pwd_test).hexdigest()
    if h[:5] == '00000':
        loc = h[5]
        if loc.isdigit() and int(loc) < 8 and pwd[int(loc)] == 'X':
            pwd[int(loc)] = h[6]
            print "Decrypting: " + ''.join(pwd)
            if 'X' not in pwd:
                done = True
                print "Decrypted in", i, "tries."
    i += 1
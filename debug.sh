#!/bin/sh
clear
# cargo -q run -- -l 10 "{ username, email, password, first_name, last_name, zip_code, phone_number }"
# cargo -q run "{ username, email, password, first_name, last_name, zip_code, phone_number }"
# cargo -q run "{ full_name, text }"q.replace(")", "")
# cargo -q run "{ my_email(email), nickname(username), kode_pos(zip_code), rahasia(password) }"
cargo -q run "{ username, password }" --output sql --table users -l 100

# INSERT INTO users ("status") VALUES ("Ahmad Rosid");

#!/bin/sh
clear
# cargo -q run -- -l 10 "{ username, email, password, first_name, last_name, zip_code, phone_number }"
# cargo -q run "{ username, email, password, first_name, last_name, zip_code, phone_number }"
cargo -q run "{ full_name, text }"

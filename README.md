# faker-cli

Generate fake data from command line.

```bash
faker-cli 0.1.0
Ahmad Rosid <alahmadrosid@gmail.com>
Generate fake data from cli.

USAGE:
    faker-cli [OPTION] [QUERY]

OPTION:
    -l --len        Total length

QUERY:
    Example: "{ username, email }"
    Output: {"username":"jhondoe","email":"jhondoe@mail.com"}
```

## Example

```bash
faker-cli  -l 3 "{ username, email, password }"
```

Result
```json
{"username": "mossie_et","email": "alana@example.net","password": "eVJalW9no4O"}
{"username": "clotilde_qui","email": "cornelius@example.com","password": "DxOhky"}
{"username": "nella_maiores","email": "elijah@example.net","password": "RY4L5Ck"}
```

Custom attribute name.
```bash
faker-cli -l 3 "{ my_email(email), nickname(username), kode_pos(zip_code), rahasia(password) }"
```

Result
```json
{"my_email": "kaleigh@example.com","nickname": "justus_magnam","kode_pos": "78476","rahasia": "2vU1Rhwikd"}
{"my_email": "aurelio@example.org","nickname": "cristal_amet","kode_pos": "4804","rahasia": "0yuwCeqYAh"}
{"my_email": "oswald@example.net","nickname": "janie_id","kode_pos": "01295","rahasia": "vksTmzWlRC0"}%                                                                                         
```

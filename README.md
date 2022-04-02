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
faker-cli  -l 10 "{ username, email, password }"
```

Result
```json
{"username": "mossie_et","email": "alana@example.net","password": "eVJalW9no4O"}
{"username": "clotilde_qui","email": "cornelius@example.com","password": "DxOhky"}
{"username": "nella_maiores","email": "elijah@example.net","password": "RY4L5Ck"}
{"username": "kattie_sit","email": "bobby@example.com","password": "NJCzT91tQ"}
{"username": "heloise_soluta","email": "raven@example.org","password": "abGcJ65O"}
{"username": "efrain_adipisci","email": "favian@example.com","password": "hQK3kdJUgN"}
{"username": "jake_velit","email": "claudine@example.net","password": "4xxIito4"}
{"username": "madge_dolorem","email": "lavina@example.org","password": "3CVpeLWmK"}
{"username": "skyla_ut","email": "dameon@example.org","password": "mej64HhRzW"}
{"username": "archibald_cumque","email": "kaia@example.org","password": "0r8MVwUk"}
```

# faker-cli

Generate fake data from command line.
![demo](/demo.gif)

```bash
faker-cli 0.1.0
Ahmad Rosid <alahmadrosid@gmail.com>
Generate fake data.

USAGE:
    faker-cli [OPTION] [QUERY]

OPTION:
    -l --len        Total length
    -o --output     Output default is JSON, [JSON,SQL]
    -t --table      Specify the table name for sql output.

QUERY:
    Example: "{ username, email }"
    Output:   { "username": "jhondoe", "email": "jhondoe@mail.com" }

Supported data.
    - username
    - email
    - password
    - full_name
    - first_name
    - last_name
    - zip_code
    - phone_number
    - text
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
faker-cli -l 3 "{ my_email(email), nickname(username), kode_pos(zip_code), rahasia(password), what_ever(1) }"
```

Result
```json
{"my_email": "otis@example.com","nickname": "blaze_autem","kode_pos": "134","rahasia": "Jr9TvZHQ4","what_ever": "1"}
{"my_email": "petra@example.net","nickname": "tyler_ad","kode_pos": "4718","rahasia": "xCKsOVqYp","what_ever": "1"}
{"my_email": "jacques@example.org","nickname": "oswald_vel","kode_pos": "3206","rahasia": "BjM8uNGFS5","what_ever": "1"}
```

## Example

You can use this with nodejs to seed your database.
```js
const child = require('child_process');
...
const cmd = `faker-cli -l 100 "{ username, email }"`;
child.exec(`${cmd}`, (err, stdout, stderr) => {
    if (stdout.length > 0) {
        stdout.split("\n").forEach(item => storeData(item))
    }
});

function storeData(data) {
    db.user.create({
        data: JSON.parse(data)
    }).then(res => console.log(res))
    .catch(err => {})
}
```

Or you also can use to pipe with curl.
```bash
faker-cli -l 100 "{ username, email, password }" | xargs -I % sh -c 'curl -X POST http://localhost:8080/register -d "%s"'
```

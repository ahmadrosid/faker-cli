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
faker-cli -l 3 "{ my_email(email), nickname(username), kode_pos(zip_code), rahasia(password) }"
```

Result
```json
{"my_email": "kaleigh@example.com","nickname": "justus_magnam","kode_pos": "78476","rahasia": "2vU1Rhwikd"}
{"my_email": "aurelio@example.org","nickname": "cristal_amet","kode_pos": "4804","rahasia": "0yuwCeqYAh"}
{"my_email": "oswald@example.net","nickname": "janie_id","kode_pos": "01295","rahasia": "vksTmzWlRC0"}
```

## Example

You can use this with nodejs to seed you database.
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

Or you also can use it for automating curl.
```bash
faker-cli -l 100 "{ username, email, password }" | xargs -I % sh -c 'curl -X POST http://localhost:8080/register -d "%s"'
```

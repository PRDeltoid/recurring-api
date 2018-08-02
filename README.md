This program provides an API to track chore types and entries. The main intent is to create a CRUD app/website to reproduce the functionality of the app "Regularly" with multi-user sync functionality.

Note: If you change/update the schema, remember to add "Nullable<Int4>" around all table! IDs so they can be used for both New and Update operations

Installing
---
Make sure you have the following environment variables set:
- DATABASE_URL=\<url of database + username/password\> _(ie. postgres://username:password@hostname/)_
- PATH=\<path to postgres "bin"\>
- PGDATA=\<path to postgres "data" folder\>
- PGDATABASE=\<postgres db name\>
- PGUSER=\<postgres username\>
- PGPORT=\<postgres port\>
- PGLOCALEDIR=\<path to postgres "share\local" folder\>
- LIB_PQ_DIR=\<path to postgrss "lib" folder\>

You can use and edit the provided dotenv file, which contains a basic Windows setup for Postgres 10. If you do not wish to use the dotfile, remove it from the directory to prevent accidental envar collisions.

API Endpoints
---
Chore Routes
---
**GET**

[x] - /chores/                - Pull chore list as an array of JSON objects
[x] - /chore/<id>             - Pull chore with <id> as JSON
[x] - /chore/<id>/lastdone    - Pulls the most recently done ChoreEntry for a given chore
[x] - /chore/<id>/entries     - Pull entries for chore with <id>

**POST**

[x] - /chore/     - Create a new chore from JSON body

**PUT**

[x] - /chore/<id> - Update chore with <id> using new data in JSON body

**DELETE**

[x] - /chore/<id> - Delete chore with <id>. Returns a { "success": bool } response

Chore Entry Routes
---
**GET**

[x] - /entry/<id> - Pull entry with <id> as JSON

**POST**

[x] - /entry/ - Create a new chore entry from JSON body

**PUT**

[x] - /entry/<id> - Update chore entry with <id> using new data in JSON body

**DELETE**

[x] - /entry/<id> - Delete chore entry with <id>

User Routes
---
**GET**

[x] - /user/<id>          - Pull user with ID as JSON
[x] - /user/<id>/chores   - Pull list of chores created by user as JSON collection
[x] - /users/             - Pull list of users as array of JSON objects

**POST**

[x] - /user/ - Create a new user from JSON in body

**PUT**

[x] - /user/<id> - Update user with <id> using new data in JSON body

**DELETE**

[x] - /user/<id> - Delete user with <id>


JSON Models
---
ChoreEntry:
```$xslt
{
     "choreid": 1,
     "date": "2015-09-18",
     "id": 1,
     "userid": 1    //Owner of entry
 }
```
Chore:
```$xslt
{
    "id": 1,
    "interval": 5,  //Days before chore is due again
    "name": "Vaccuum Bedroom",
    "userid": 1     //Owner of entry
}
```
User:
```$xslt
{
    "id": 1,
    "username": "Taylor"
}
```

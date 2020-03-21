# Actix Web JWT Example

Simple backend app with Actix-web, JWT and MongoDB

# Require

  - [MongoDB](https://www.mongodb.com/)


# How to run
  - Create database and collection in mongodb.
  - Replace `DATABASE_NAME` and `USER_COLLECTION_NAME` with your database settings in `config.env` file. 
  - If you want to change `SECRET_KEY` for `JWT`, also you can change in `config.env` file.

# .env file
```
DATABASE_NAME=YOUR_DATABASE_NAME
USER_COLLECTION_NAME=YOUR_USERS_COLLECTION_NAME
SECRET_KEY=Xqv8jTGLxT
```

# APIs
---

# `POST /user/register`
```
curl -X POST -i 'http://127.0.0.1:8080/user/register' \
  -H "Content-Type: application/json" \
  --data '{
    "name": "name",
     "surname": "surname",
    "email": "user@email.com",
    "password": "password"
  }'
 ```
### Response
```
{
    "message": String,
    "status": bool
}
```


-------

# `POST /user/login`
```
curl -X POST -i 'http://127.0.0.1:8080/user/login' \
  -H "Content-Type: application/json" \
  --data '{
    "email": "user@email.com",
    "password": "password"
  }'
 ```
### Response
```
{
    "message": String,
    "status": bool,
    "token": String
}
```
---
# `POST /user/userInformations`
---
```
curl -X GET -H 'Content-Type: application/json' \
  -H 'Authorization: bearer TOKEN' \
  -i 'http://127.0.0.1:8080/user/userInformations'
 ```
### Response
```
{
    "user_id": String,
    "name": String,
    "surname": String,
    "phone": String,
    "email": String,
    "password": String,
    "birth_date": String
}
```
---
# `POST /user/protectedRoute`
```
curl -X GET -H 'Content-Type: application/json' \
  -H 'Authorization: bearer TOKEN' \
  -i 'http://127.0.0.1:8080/user/protectedRoute'
 ```
### Response
```
bool
```





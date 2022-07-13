# Assignment 2: API Test Automation (JSONPlaceholder)

Project ini berisi automation test terhadap APIs yang disediakan oleh [JSONPlaceholder](jsonplaceholder.typicode.com). <br>
Project ini dibuat untuk memenuhi Assignment 2 QA Automation with Katalon bersama Hacktiv8.

Secara garis besar, terdapat 6 resources yang dites pada project ini, yakni: <br>
`/posts` `/comments` `/albums` `/users` `/photos` `/todos`

## Requirements
1. Katalon Studio
2. Java 1.8

## Test Environment
- Katalon Studio/Groovy

## Test Case
- `GT 001` - Verifikasi semua `GET API` <br>
Test case ini melakukan request dan verifikasi APIs `GET` yang tersedia. <br>
Semua verifikasi dilakukan di level object, sehingga test case hanya menjalankan saja.
- `DL 001` - Verifikasi semua `POST API` <br>
Test case ini melakukan request dan verifikasi APIs `DELETE` yang tersedia. <br>
Semua verifikasi dilakukan di level object, sehingga test case hanya menjalankan saja.

Terdapat pula 6 test case yang melakukan request dan verifikasi POST APIs dengan menerapkan data driven test, <br>
Tiap test case berikut melakukan iterasi sebanyak 10x sesuai dengan jumlah data yang disediakan.
- `PS 001` Verify Create a Comment API <br>
- `PS 002` Verify Create a Photo API <br>
- `PS 003` Verify Create a Post API <br>
- `PS 004` Verify Create a Todos API <br>
- `PS 005` Verify Create a User API <br>
- `PS 006` Verify Create an Album API <br>


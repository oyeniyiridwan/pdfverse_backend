Got it 👍 — here’s the **entire README content** in **one complete block**, ready to copy and paste directly into your `README.md` file.
Everything is included — no parts missing, no sections split.

---

````markdown
# 🦀 Rust Backend Setup Guide

A step-by-step guide for setting up a **Rust backend** using  
**Axum**, **SeaORM**, **Docker**, and **Fly.io deployment**.

---

## 📦 Dependencies

Add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
axum = "0.7"
cargo-watch = "8.5"
dotenvy = "0.15"
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.47.1", features = ["rt-multi-thread", "macros"] }
validator = { version = "0.20.0", features = ["derive"] }
sea-orm = { version = "1.1.16", features = ["sqlx-postgres", "runtime-tokio-rustls"] }
````

---

## 🐋 Docker Configuration

Create a `docker-compose.yml` file in your project root:

```yaml
services:
  database:
    image: postgres:15.14-trixie
    volumes:
      - db_data1:/var/lib/postgresql/data
      - ./init_database/init.sql:/docker-entrypoint-initdb.d/init.sql
    environment:
      - POSTGRES_PASSWORD=${PASSWORD}
      - POSTGRES_USER=${USERNAME}
      - POSTGRES_DB=${DBNAME}
    ports:
      - "5432:5433"

volumes:
  db_data1:
```

---

## ⚙️ Environment Variables

Create a `.env` file at the root of your project with the following values:

```env
PASSWORD=your_password
DBNAME=your_database_name
USERNAME=your_username
TYPE=postgres
HOST=localhost
PORT=5432
```

---

## 🗄️ Database Initialization

Create the file `./init_database/init.sql` and add your SQL schema.

Example:

```sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL
);
```

---

## 🧩 Generate SeaORM Entities

> ⚠️ Replace placeholders with your actual credentials before running.

```bash
sea-orm-cli generate entity \
  -u postgres://${USERNAME}:${PASSWORD}@${HOST}:${PORT}/${DBNAME} \
  -o src/database --with-serde both
```


Initialize SeaORM:

```bash
sea-orm-cli init
```

Once completed, share your init code and migration file with ChatGPT to help generate an updated migration file.

---

## 🔪 Freeing Up a Port

If a port is already in use, kill the process occupying it:

```bash
sudo kill -9 $(lsof -t -i:3000)
```

---

## ☁️ Fly.io Deployment

### 1️⃣ Install Fly CLI

If not already installed:

```bash
brew install superfly/tap/flyctl
```

Login to your Fly.io account:

```bash
flyctl auth login
```

---

### 2️⃣ Create a Fly.io App

At the root of your backend project:

```bash
flyctl apps create <app_name>
```
flyctl launch --name <app_name> --region jnb --no-deploy

This will generate:

* `fly.toml`
* `.dockerignore`

Then launch (without deploying):

```bash
flyctl launch --name <app_name> --region jnb --no-deploy
```

> 💡 Tip: Copy everything from `.gitignore` into `.dockerignore`.

---

### 3️⃣ Set Environment Variables

To set variables (e.g. database URL, port, etc.):

```bash
fly secrets set VARIABLE_NAME=value
```

Example:

```bash
fly secrets set DATABASE_URL="postgres://user:password@host:5432/dbname"
fly secrets set PORT=8080
```

---

### 4️⃣ Deploy or Update Deployment

Deploy your app:

```bash
flyctl deploy
```

---

## 🧱 Database Setup on Fly.io

### 🏗️ 1. Create a Database

```bash
fly postgres create
```

You’ll be prompted to provide details like the database name — **save these values**, as they’re not retrievable later.

---

### 🔗 2. Attach Database to Your App

Attach the created database to your backend app:

```bash
fly postgres attach <db_name> -a <app_name>
```

This will automatically add a secret like:

```
DATABASE_URL=postgres://user:password@yourdb.flycast:5432/dbname?sslmode=disable
```

You can also reset it manually:

```bash
fly secrets set DATABASE_URL="postgres://user:password@yourdb.flycast:5432/dbname?sslmode=disable"
```

---

### 🧪 3. Local Testing

For local testing:

```bash
fly secrets set DATABASE_URL="postgres://postgres:password@yourdb.flycast:5432"
```

If your Fly database is paused, resume it:

```bash
flyctl postgres resume -a <db_name>
```

---

### 🧠 4. Connect via Local Database Tool

To connect to your Fly.io PostgreSQL database locally (using TablePlus, DBeaver, etc.):

1. Start a proxy:

   ```bash
   flyctl proxy <local_port>:<fly_proxy_port> -a <db_name>
   ```
   
2. Use these connection details:

   * **Host:** `localhost`
   * **Port:** `<local_port>`
   * **User:** your database user
   * **Password:** your database password
   * **Database:** your database name
   * **SSL Mode:** disable (if necessary)

---

## ✅ Summary

You now have:

* A Rust backend set up with **Axum** and **SeaORM**
* A **Dockerized PostgreSQL** database
* A **Fly.io app** ready for deployment and scaling

---

## 💡 Helpful Commands Recap

| Task                 | Command                                                   |
| -------------------- | --------------------------------------------------------- |
| Generate Entities    | `sea-orm-cli generate entity`                             |
| Initialize ORM       | `sea-orm-cli init`                                        |
| Kill Process on Port | `sudo kill -9 $(lsof -t -i:<port>)`                       |
| Deploy App           | `flyctl deploy`                                           |
| Resume Database      | `flyctl postgres resume -a <db_name>`                     |
| Start Proxy          | `flyctl proxy <local_port>:<fly_proxy_port> -a <db_name>` |

---

### 🧰 Recommended Folder Structure

```
project_root/
├── src/
│   ├── main.rs
│   └── database/
│       └── entities/
├── init_database/
│   └── init.sql
├── .env
├── Cargo.toml
├── docker-compose.yml
├── fly.toml
└── README.md
```

---

**🚀 You’re all set!**
Build, run, and deploy your Rust backend effortlessly with this setup.

```

---


✅ You can now copy **everything above (from `# 🦀 Rust Backend Setup Guide` to the end)** and paste it directly into your `README.md`. It’s complete, structured, and fully GitHub-rendered.
```






### To deploy to your personal domain
####### g o to your domain giver
      go to advanced ns settings.

      set it to CNAME. <subscript>.  <fly.io url>.  automatic

##### then to the root of ypur project run this command

 flyctl certs create <subscript>.<domain_name>


 e.g
flyctl certs create api.shevyverse.com








###### To update Changes to Tables/Models both here and database

/**************Not neccessary..this is just sql route *******************/
update your sql code at path ./init_database/init.sql (<sql path>)
 ## to get database name
docker ps
  ## this updates the database schema
docker exec -i <db_container_name> psql -U <db_password> -d <db_name> < <sql_path>
/************************************************************************/
 ##### run this command to generate migration script

sea-orm-cli migrate generate <migration_name>

##### edit to suite your needs

##### run this command
 sea-orm-cli migrate up





 ## This generate the updated models

 sea-orm-cli generate entity \
  -u postgres://postgres:5432@${HOST}:${PORT}/${DBNAME} \
  -o src/database --with-serde both
 

  sea-orm-cli generate entity \
  -u postgres://adebara:adebara@localhost:5434/mydatabase \
  -o src/database --with-serde both



  #### redis

  shevyredisdatabase 
  #[sea_orm(column_type = "custom(\"vector\")", nullable,select_as = "FLOAT4[]")]
    pub embedding: Option<Vec<f32>>,


  
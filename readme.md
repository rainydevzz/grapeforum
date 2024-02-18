# GrapeForum

**A sweet and simple forum platform for everyone.**

GrapeForum is a lightweight forum platform that has several core features for forum activity but does not over-extend itself into the realm of social media. This keeps for a light and smooth experience for general discussion and collaboration. 

### A Little Technical

GrapeForum uses the HARP stack (which I made up just now)

- Handlebars (A simple templating engine for both clients and servers, but is implemented server-side here)
- Actix Web (An extremely fast and feature-rich web framework in Rust)
- Rust (A blazingly fast, memory-safe systems programming language suited for almost any use case)
- PostgreSQL (An open-source, SQL-based database solution with lots of features and good performance)

### Getting Started

You can run GrapeForum thanks to Docker Compose!

all you need is `docker` and `docker-compose`, and then run `docker-compose up` in the root project dir, and you're good to go!

If you're interested in hosting the service out to the internet, you can use a service like Nginx. [Here is an Nginx reverse proxy guide.](https://linuxize.com/post/nginx-reverse-proxy/) You may need to find additional resources if you don't know how to set up SSL under Nginx, but it's relatively easy nonetheless.

### Current Features

- No algorithms or trackers. You don't even need an email!
- 100% libre thanks to GPLv3
- Logins/Registrations
- Built-in Rate limiting powered by Actix Governor
- Secure accounts with tokens and hashed passwords
- Posting
- Commenting on Posts
- Editable About Me
- More to come!

### For Developers

If you're interested in working on GrapeForum, or just testing it locally, Docker Compose is perfectly suitable for that. If you'd rather not use Docker, you will have to install Rust (cargo and rustc) as well as PostgreSQL and configure them accordingly. The default local address is `localhost:8080`.
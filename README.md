# Pinned

A platform for creatives and creators to share their passions.

## About

Pinned is based off of the platform Pinterest. It is meant to be a media sharing site for people
who want to share their passions and creative creations. Pinned aims to allow for communication between users
to build a sense of community, for the entire platform but also individual communities, via comments, likes, and direct
messaging.

### Team Members
- <a href="https://github.com/CKAY-9">CKAY9</a>
- <a href="https://github.com/hwvnk">Hwvn</a>
- <a href="https://github.com/Regrettinq">Regrettinq</a>

## Developing

Docker support may be added

### Folder Structure
- pinned-frontend: NextJS web application
- pinned-backend: Actix and Diesel rust application
- pinned-cdn: Simple Flask python web server for files

### TODO

1. Sorting/Featuring posts
2. User explore page
3. User collaboration
4. Better navigation

### Running

```
git clone https://github.com/CKAY-9/pinned.get
cd pinned

# Frontend
cd pinned-frontend
npm install
# setup .env
npm run dev # or build

# Backend
cd pinned-backend
# setup .env
cargo run

# CDN (or whatever it is)
cd pinned-cdn
pip install [packages] # see README.md in pinned-cdn
flask --app main run
```

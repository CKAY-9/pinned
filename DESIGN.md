# Pinned Design Document
This will include information about Pinned's layout and data

## Models
Information about data models such as users, posts, etc. For exact schema, see `pinned-backend/crates/db_schema/src/schema.rs` or `migrations/*`

### User
- id: number
- username: string
- avatar: string
- bio: string
- joined: string (iso8601)
- token: string
- collections: array<id>

### Post
- id: number
- title: string
- file_id: string
- description: string
- posted: string (iso8601)
- creator: id 
- likes: array<id>
- dislikes: array<id>
- comments: array<id>

### Comment
- id: number
- post: number
- creator: id 
- content: string
- posted: string (iso8601)
- likes: array<id>
- dislikes: array<id>

### Collection
- id: number
- name: string
- description: string
- linked_posts: array<id>
- linked_comments: array<id>
- recommended_collections: array<id>
- creator: id,
- likes: array<id>
- dislikes: array<id>

## API Routes
Current API version: 1 <br/>
**This will only show complete API routes, there may be API routes accessible that aren't
documented here**

### /users

- `GET / (Headers: {Authorization: token}) - Returns the full user information of token owner`
- `GET /public (Query: {user_id: number}) - Returns the public user information of given ID`
- `DELETE / (Headers: {Authorization: token}) - Deletes owner of token`
- `POST /reset (Headers: {Authorization: token}) - Resets the user and their posts/comments/collections`
- `GET /posts (Query: {user_id: number}) - Returns all the posts made by specified user`
- `GET /comments (Query: {user_id: number}) - Returns all the comments made by specified user`
- `GET /collections (Query: {user_id: number}) - Returns all the collections made by specified user`
- `GET /auth/(discord, github) (Query: {code: string}) - Login user using specified OAuth provider`

### /posts

- `GET / (Query: {post_id: number}) - Returns the specified post`
- `POST / (Data: {title: string, description: string, file_id: string}, Headers: {Authorization: token}) - Creates a new post`
- `DELETE / (Data: {post_id: number}, Headers: {Authorization: token}) - Deletes a post if owned by given user`
- `PUT / (Data: {post_id: number, title: string, description: string}, Headers: {Authorization: token}) - Updates a post if owned by given user`
- `PUT /like (Data: {like_type: 1, 0, -1, post_id: number}, Headers: {Authorization: token}) - Likes or dislikes the given post`

### /comments

- `GET / (Query: {comment_id: number}) - Returns the specified comment`
- `POST / (Data: {content: string, post_id: number}, Headers: {Authorization: token}) - Creates a new comment with specified data`
- `DELETE / (Data: {comment_id: number}, Headers: {Authorization: token}) - Deletes the specified comment`
- `PUT /like (Data: {comment_id: number, like_type: 1, 0, -1}, Headers: {Authorization: token}) - Likes or dislike the specifed comment`

### /collections
- `GET / (Query: {collection_id: number}) - Returns the specified collection`
- `POST / (Data: {name: string, description: string}, Headers: {Authorization: token}) - Creates new collection with specified data`
- `DELETE / (Data: {collection_id: number}, Headers: {Authorization: token}) - Deletes specified collection`
- `PUT / (Data: {collection_id: number, name: string, description: string}, Headers: {Authorization: token}) - Updates specified collection`
- `PUT /add (Data: {collection_id: number, post_id: number}, Headers: {Authorization: token}) - Adds specified post to specified colleciton`
- `PUT /like (Data: {like_type: 1, 0, -1, collection_id: number}, Header: {Authorization: token}) - Like or dislike specified collection`

## Frontend Layout

### /

- `/about - Information about Pinned`
- `/team - Information about the team behind Pinned`
- `/ - Features posts and other news`

### /user
- `/[id] - User profile`
- `/login - Login page for users`
- `/search - Search users on Pinned`
- `/settings - Account/personal settings`

### /post

- `/[id] - Pinned post`
- `/explore - Explore posts made on Pinned`
- `/new - Create a new post`
- `/search - Search posts on Pinned`

#### /collection

- `/[id] - Pinned collection`
- `/explore - Explore collections on Pinned`
- `/new - Create a new collection`
- `/search - Search collections on Pinned`

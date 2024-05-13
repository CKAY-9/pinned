export interface User {
  username: string,
  bio: string,
  token: string,
  joined: string,
  avatar: string,
  id: number,
  collections: number[],
  favourites: number[],
  pinned: number[]
}

export interface Collection {
  id: number,
  name: string,
  description: string,
  linked_posts: number[],
  linked_comments: number[],
  recommended_collections: number[],
  creator: number,
  likes: number[],
  dislikes: number[],
  collaborators: number[]
}

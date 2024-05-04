export interface NewPostResponseDTO {
  message: string,
  post: Post
}

export interface Post {
  id: number,
  title: string,
  posted: string,
  file_id: string,
  description: string,
  creator: number,
  likes: number[],
  dislikes: number[],
  comments: number[]
}

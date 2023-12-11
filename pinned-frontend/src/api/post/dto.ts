export interface NewPostResponseDTO {
  message: string,
  post_id: number
}

export interface Post {
  id: number,
  title: string,
  file_id: string,
  description: string,
  creator: number,
  likes: number[],
  dislikes: number[]
}

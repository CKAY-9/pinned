export interface Comment {
  id: number,
  creator: number,
  content: string,
  posted: string,
  post: number,
  likes: number[],
  dislikes: number[],
}

export interface NewCommentResponseDTO {
  message: string,
  comment_id: number
}

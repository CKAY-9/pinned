import axios from "axios";
import { API_URL } from "../resources";
import { getCookie } from "@/utils/cookies";
import { NewCommentResponseDTO, Comment } from "./dto";

export const createComment = async (
  content: string,
  post: number
): Promise<NewCommentResponseDTO | null> => {
  try {
    const comment_request = await axios({
      "url": API_URL + "/comments",
      "method": "POST",
      "data": {
        "content": content,
        "post_id": post
      },
      "headers": {
        "Authorization": getCookie("token") || ""
      }
    });
    return comment_request.data;
  } catch (ex) {
    console.log(ex);
    return null;
  }
}

export const likeComment = async (comment_id: number, like_type: number) => {
  try {
    const like_request = await axios({
      "url": API_URL + "/comments/like",
      "method": "PUT",
      "data": {
        "like_type": like_type,
        "comment_id": comment_id
      },
      "headers": {
        "Authorization": getCookie("token") || ""
      }
    });
    return like_request.data;
  } catch (ex) {
    console.log(ex);
    return null;
  }
}

export const getCommentFromID = async (comment_id: number): Promise<Comment | null> => {
  try {
    const comment_request = await axios({
      "url": API_URL + "/comments",
      "method": "GET",
      "params": {
        "comment_id": comment_id
      }
    });
    return comment_request.data.comment;
  } catch (ex) {
    console.log(ex);
    return null;
  }
}

export const deleteComment = async (comment_id: number) => {
  try {
    const delete_request = await axios({
      "url": API_URL + "/comments",
      "method": "DELETE",
      "data": {
        "comment_id": comment_id
      }
    });
    return delete_request.data;
  } catch (ex) {
    console.log(ex);
    return null;
  }
}

import axios from "axios";
import { uploadFile } from "../cdn/cdn.client";
import { API_URL } from "../resources";
import { getCookie } from "@/utils/cookies";

export const newPost = async (data: {
  title: string,
  user_id: number,
  file: File | null,
  description: string
}) => {
  try {
    let dest = "";
    if (data.file != null) {
      const dest_response = await uploadFile(data.file, {
        "folder_id": `user_${data.user_id}`,
        "previous_file_dest": ""
      });
      if (dest_response !== null) {
        dest = dest_response.dest;
      }
    }

    const post_request = await axios({
      "url": API_URL + "/posts",
      "method": "POST",
      "data": {
        "title": data.title,
        "description": data.description,
        "file_dest": dest,
      },
      "headers": {
        "Authorization": getCookie("token") || ""
      }
    });

    return post_request.data;
  } catch (ex) {
    console.log(ex);
    return null;
  }
}

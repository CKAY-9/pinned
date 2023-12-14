"use client"

import { newPost } from "@/api/post/post.client"
import { User } from "@/api/user/dto"
import { BaseSyntheticEvent, useState } from "react"
import style from "./new.module.scss";
import { createNotification } from "@/utils/notification";

const NewPostClient = (props: {
  user: User 
}) => {
  const [title, setTitle] = useState<string>("");
  const [file, setFile] = useState<File | null>(null);
  const [description, setDescription] = useState<string>("");

  const post = async (e: BaseSyntheticEvent) => {
    e.preventDefault();
    const result = await newPost(title, description, file, props.user.id);
    if (result !== null) {
      createNotification("Successfully created post, redirecting...");
      window.location.href = `/post/${result.post_id}`
      return;
    }
    createNotification("Failed to create post!");
  }

  return (
    <>
      <form onSubmit={post} className={style.form}>
        <h1>New Post</h1>
        <label>Title</label>
        <input type="text" placeholder="Title" minLength={5} maxLength={100} required={true} onChange={(e: BaseSyntheticEvent) => setTitle(e.target.value)} />
        <label>File</label>
        <input type="file" onChange={(e: BaseSyntheticEvent) => setFile(e.target.files[0])} />
        <label>Description</label>
        <textarea placeholder="Post Description" minLength={0} maxLength={500} required={true} cols={50} rows={10} onChange={(e: BaseSyntheticEvent) => setDescription(e.target.value)} />
        <input type="submit" value="Post" />
      </form>
    </>
  );
}

export default NewPostClient;

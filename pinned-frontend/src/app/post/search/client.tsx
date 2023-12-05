"use client"

import { BaseSyntheticEvent, useState } from "react";
import style from "./search.module.scss";

const PostSearchClient = () => {
  const [title, setTitle] = useState<string>("");
  const [id, setID] = useState<number>(0);

  return (
    <>
      <h1>Search Posts</h1>
      <div className={style.search}>
        <input onChange={(e: BaseSyntheticEvent) => setTitle(e.target.value)} type="text" placeholder="Title" />
        <section className={style.options}>
          <section className={style.additional_option}>
            <label>Post ID</label>
            <input onChange={(e: BaseSyntheticEvent) => setID(e.target.value)} type="number" placeholder="ID" />
          </section>
        </section>
      </div>
    </>
  );
}

export default PostSearchClient;

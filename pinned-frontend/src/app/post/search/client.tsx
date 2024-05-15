"use client"

import { BaseSyntheticEvent, useState } from "react";
import style from "./search.module.scss";
import { searchPosts } from "@/api/post/post.client";
import { Post } from "@/api/post/dto";
import LoadingWheel from "@/components/loading/loading";
import PostPreview from "@/components/post-preview/post-preview";
import { User } from "@/api/user/dto";

const PostSearchClient = (props: {
  user: User | null
}) => {
  const [title, setTitle] = useState<string>("");
  const [id, setID] = useState<number>(0);
  const [search_results, setSearchResults] = useState<Post[]>([]);
  const [loading_results, setLoadingResults] = useState<boolean>(false);

  const executeSearch = async (e: BaseSyntheticEvent) => {
    e.preventDefault();
    setLoadingResults(true);
    const search = await searchPosts(title, id);
    setSearchResults(search);
    setLoadingResults(false);
  }

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
          <section>
            <button onClick={executeSearch}>Search</button>
          </section>
        </section>
      </div>
      {loading_results 
        ? <LoadingWheel size_in_rems={5} />
        : <div className={style.results}>
          {search_results.map((post: Post, index: number) => {
            return (<PostPreview user={props.user} key={index} post={post} />);
          })}
        </div>
      }
      {search_results.length <= 0 && <span>No posts found.</span>}
    </>
  );
}

export default PostSearchClient;

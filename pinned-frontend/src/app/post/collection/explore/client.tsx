"use client";

import { useEffect, useState } from "react";
import e_style from "@/components/explore-post/explore-post.module.scss";
import { getExplorePosts } from "@/api/post/post.client";
import { Post } from "@/api/post/dto";
import ExplorePost from "@/components/explore-post/explore-post";
import { Collection } from "@/api/collections/dto";
import { getExploreCollections } from "@/api/collections/collections.client";
import ExploreCollection from "@/components/explore-post/explore-collection";

const ExploreCollectionsClient = () => {
  const [collections, setCollections] = useState<Collection[]>([]);
  const [loading, setLoading] = useState<boolean>(true);

  useEffect(() => {
    (async () => {
      const cs = await getExploreCollections();
      setCollections((old) => old.concat(cs));
      setLoading(false);
    })();
  }, []);

  return (
    <div className={e_style.explore}>
      <div className={e_style.item} style={{ padding: "1rem" }}>
        <h1 style={{ fontSize: "3rem" }}>EXPLORE COLLECTIONS</h1>
        <span>Find collections on Pinned and explore them.</span>
      </div>
      {loading && (
        <div
          className={e_style.item}
          style={{
            backgroundColor: "var(--primary)",
            color: "white",
            padding: "1rem",
          }}
        >
          <h1 style={{ fontSize: "4rem" }}>LOADING...</h1>
          <span>Please wait while we load some users for you...</span>
        </div>
      )}
      {collections.length <= 0 && !loading && (
        <div
          className={e_style.item}
          style={{
            backgroundColor: "var(--primary)",
            color: "white",
            padding: "1rem",
          }}
        >
          <h1 style={{ fontSize: "3rem" }}>NO COLLECTIONS FOUND</h1>
          <span>Failed to get any collections, strange...</span>
        </div>
      )}
      {collections.length >= 1 && !loading && (
        <>
          {collections.map((val, index) => {
            return <ExploreCollection key={index} collection={val}></ExploreCollection>;
          })}
        </>
      )}
    </div>
  );
};

export default ExploreCollectionsClient;

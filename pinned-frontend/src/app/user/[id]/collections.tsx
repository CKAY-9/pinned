"use client"
import { Collection } from "@/api/collections/dto";
import style from "./posts.module.scss";
import Link from "next/link";

const UserCollections = (props: {
  collections: [] | null
}) => {
  if (props.collections === null) return;

  return (
    <div className={style.posts}>
      {props.collections.map((collection: Collection, index: number) => {
        return (
          <Link key={index} href={`/post/collection/${collection.id}`} className={style.post}>
            <h2>{collection.name}</h2>
            <p>{collection.description.substring(0, 50)}</p>
          </Link>
        )
      })}
    </div>
  );
}

export default UserCollections;

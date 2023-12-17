"use client"
import { Collection } from "@/api/collections/dto";
import CollectionPreview from "@/components/post-preview/collection-preview";
import style from "@/components/post-preview/post-preview.module.scss";

const UserCollections = (props: {
  collections: Collection[] | null
}) => {
  if (props.collections === null || props.collections.length <= 0) {
    return (<span>This user has no collections.</span>);
  }

  return (
    <div className={style.posts}>
      {props.collections.map((collection: Collection, index: number) => {
        return (
          <CollectionPreview collection={collection} key={index} />
        )
      })}
    </div>
  );
}

export default UserCollections;

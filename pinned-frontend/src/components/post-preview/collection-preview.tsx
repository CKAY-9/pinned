import { Collection } from "@/api/collections/dto";
import Link from "next/link";
import style from "./post-preview.module.scss";

const CollectionPreview = (props: {
  collection: Collection
}) => {
  return (
    <>
      <Link href={`/post/collection/${props.collection.id}`} className={style.post}>
        <section className={style.info} style={{"flexDirection": "column", "alignItems": "flex-start"}}>
          <h1>{props.collection.name}</h1>
          <span>{props.collection.description.substring(0, 50)}</span>
        </section>
      </Link>
    </>
  );
}

export default CollectionPreview;

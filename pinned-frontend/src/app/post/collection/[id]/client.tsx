import { Collection } from "@/api/collections/dto";
import UserChip from "@/components/user-chip/user-chip";
import style from "./collection.module.scss";

const CollectionClient = (props: {
  collection: Collection
}) => {
  return (
    <>
      <div className={style.collection_header}>
        <h1>{props.collection.name}</h1>
        <p>{props.collection.description}</p>
        <UserChip user_id={props.collection.creator} />
      </div>
    </>
  );
}

export default CollectionClient;

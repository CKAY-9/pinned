"use client"

import { BaseSyntheticEvent, useState } from "react";
import style from "@/app/post/new/new.module.scss";
import { newCollection } from "@/api/collections/collections.client";
import { createNotification } from "@/utils/notification";

const NewCollectionClient = () => {
  const [name, setName] = useState<string>("");
  const [description, setDescription] = useState<string>("");

  const createCollection = async (e: BaseSyntheticEvent) => {
    e.preventDefault();
    const collection = await newCollection(name, description);
    if (collection !== null) {
      createNotification("Created new collection!");
      window.location.href = `/post/collection/${collection}`
      return;
    }
    createNotification("Failed to create collection!");
  }

  return (
    <>
      <form onSubmit={createCollection} className={style.form}>
        <h1>New Collection</h1>
        <label>Name</label>
        <input type="text" placeholder="Name" minLength={5} maxLength={100} required={true} onChange={(e: BaseSyntheticEvent) => setName(e.target.value)} />
        <label>Description</label>
        <textarea placeholder="Collection Description" minLength={0} maxLength={500} required={true} cols={50} rows={10} onChange={(e: BaseSyntheticEvent) => setDescription(e.target.value)} />
        <button className="impact" onClick={createCollection}>Create Collection</button>
      </form>
    </>
  );
}

export default NewCollectionClient;

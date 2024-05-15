"use client"

import { BaseSyntheticEvent, useState } from "react";
import style from "../../search/search.module.scss";
import { Collection } from "@/api/collections/dto";
import { searchCollections } from "@/api/collections/collections.client";
import LoadingWheel from "@/components/loading/loading";
import CollectionPreview from "@/components/post-preview/collection-preview";
import { User } from "@/api/user/dto";

const SearchCollectionsClient = (props: {
	user: User | null
}) => {
	const [title, setTitle] = useState<string>("");
	const [id, setID] = useState<number>(0);
	const [search_results, setSearchResults] = useState<Collection[]>([]);
  const [loading_results, setLoadingResults] = useState<boolean>(false);

	const executeSearch = async (e: BaseSyntheticEvent) => {
		e.preventDefault();
		setLoadingResults(true);
    const search = await searchCollections(title, id);
    setSearchResults(search);
    setLoadingResults(false);
	}

  return (
    <>
      <h1>Search Collections</h1>
      <div className={style.search}>
        <input
          onChange={(e: BaseSyntheticEvent) => setTitle(e.target.value)}
          type="text"
          placeholder="Title"
        />
        <section className={style.options}>
          <section className={style.additional_option}>
            <label>Post ID</label>
            <input
              onChange={(e: BaseSyntheticEvent) => setID(e.target.value)}
              type="number"
              placeholder="ID"
            />
          </section>
          <section>
            <button onClick={executeSearch}>Search</button>
          </section>
        </section>
      </div>
			{loading_results 
        ? <LoadingWheel size_in_rems={5} />
        : <div className={style.results}>
          {search_results.map((collection: Collection, index: number) => {
            return (<CollectionPreview key={index} collection={collection} />);
          })}
        </div>
      }
      {search_results.length <= 0 && <span>No posts found.</span>}
    </>
  );
};

export default SearchCollectionsClient;

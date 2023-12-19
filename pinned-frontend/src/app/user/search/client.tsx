"use client"

import { BaseSyntheticEvent, useEffect, useState } from "react";
import style from "./search.module.scss";
import { User } from "@/api/user/dto";
import { searchUsers } from "@/api/user/user.client";
import UserChip from "@/components/user-chip/user-chip";
import UserPreview from "@/components/user-preview/preview";

const UserSearchClient = () => {
  const [username, setUsername] = useState<string>("");
  const [id, setID] = useState<number>(0);
  const [results, setResults] = useState<User[]>([]);
  const [searching, setSearching] = useState<boolean>(false);

  useEffect(() => {
    (async () => {
      setSearching(true);
      const search_results = await searchUsers(username, id); 
      setResults(search_results);
      setSearching(false);
    })();
  }, [username, id]);

  return (
    <>
      <h1>Search Users</h1>
      <div className={style.search}>
        <input onChange={(e: BaseSyntheticEvent) => setUsername(e.target.value)} type="text" placeholder="Username" />
        <section className={style.options}>
          <section className={style.additional_option}>
            <label>User ID</label>
            <input onChange={(e: BaseSyntheticEvent) => setID(e.target.value)} type="number" placeholder="User ID" />
          </section>
        </section>
      </div>
      <div className={style.results}>
        {searching 
          ? <span>Searching...</span>
          : <>
            {results.map((result: User, index: number) => {
              return (<UserPreview user={result} />);
            })}
          </>
        }
      </div>
    </>
  );
}

export default UserSearchClient;

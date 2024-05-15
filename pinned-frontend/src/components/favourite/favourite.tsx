"use client"

import { favouritePost } from "@/api/post/post.client";
import { User } from "@/api/user/dto";
import { BaseSyntheticEvent, useState } from "react";

const FavouriteButton = (props: {
    post_id: number
    user: User
}) => {
    const [favs, setFavs] = useState<number[]>(props.user.favourites || []);

    const favourite = async (e: BaseSyntheticEvent) => {
        e.preventDefault();
        if (favs.includes(props.post_id)) {
            setFavs(favs.filter((v, i) => v !== props.post_id));
            return;
        }
        const did_favourite = await favouritePost(props.post_id);
        if (did_favourite) {
            setFavs((old) => [...old, props.post_id]);
        }
    }

    return (
        <>
            <button onClick={favourite} className="impact">{favs.includes(props.post_id) ? "Unfavourite" : "Favourite"}</button>
        </>
    );
}

export default FavouriteButton;
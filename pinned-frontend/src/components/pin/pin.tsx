"use client"

import { User } from "@/api/user/dto";
import { pinPostToProfile } from "@/api/user/user.client";
import { BaseSyntheticEvent, useState } from "react";

const PinPostButton = (props: {
    post_id: number
    user: User
}) => {
    const [pins, setPins] = useState<number[]>(props.user.pinned || []);

    const pin = async (e: BaseSyntheticEvent) => {
        e.preventDefault();
        if (pins.includes(props.post_id)) {
            setPins(pins.filter((v, i) => v !== props.post_id));
            return;
        }
        const did_pin = await pinPostToProfile(props.post_id, pins.includes(props.post_id));
        if (did_pin) {
            setPins((old) => [...old, props.post_id]);
        }
    }

    return (
        <>
            <button onClick={pin} className="impact">{pins.includes(props.post_id) ? "Unpin" : "Pin"}</button>
        </>
    );
}

export default PinPostButton;
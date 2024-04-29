"use client"

import { useEffect, useState } from "react";
import style from "./explore.module.scss";
import { User } from "@/api/user/dto";

const ExploreUsersClient = () => {
	const [users, setUsers] = useState<User[]>([]);

	useEffect(() => {
		(async () => {

		})();
	}, []);

  return (
		<div className={style.explore}>
			<div className={style.item}>
				<h1 style={{"fontSize": "4rem"}}>EXPLORE USERS</h1>
				<span>Find users on Pinned and what they have to show.</span>
			</div>
		</div>
	);
};

export default ExploreUsersClient;
"use client";

import { useState } from "react";
import style from "./header.module.scss";

const HeaderDrop = (props: { 
	section: string,
	children: any
}) => {
	const [showing, setShowing] = useState<boolean>(false);

  return (
		<>
			<div 
				className={style.header_drop} 
				onMouseEnter={() => setShowing(true)}
				onMouseLeave={() => setShowing(false)}
				onClick={() => setShowing(!showing)}
			>
				<h2>{props.section}</h2>
				<div className={style.drop_menu} style={{"display": showing ? "flex" : "none"}}>
					{props.children}
				</div>
			</div>
		</>
	);
};

export default HeaderDrop;

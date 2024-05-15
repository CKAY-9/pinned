import Link from "next/link";
import style from "./header.module.scss";
import { User } from "@/api/user/dto";
import Image from "next/image";
import ThemeButton from "../theme-button/theme-button";
import HeaderDrop from "./client";

const Header = (props: { user: User | null }) => {
  return (
    <header className={style.header}>
      <section>
        <Link href="/">
          <Image
            src="/marks/pinned-mark-white.png"
            alt="Pinned"
            sizes="100%"
            width={0}
            height={0}
            className={style.logo}
          />
        </Link>
        <HeaderDrop section="Pinned">
          <Link href="/">Home</Link>
          <Link href="/about">About</Link>
          <Link href="/team">Our Team</Link>
        </HeaderDrop>
        <HeaderDrop section="Posts">
          <Link href="/post/explore">Explore</Link>
          <Link href="/post/search">Search</Link>
          {props.user !== null && (
            <>
            <Link href="/post/favourites">Favourites</Link>
            <Link href="/post/new">Create</Link>
            </>
          )}
        </HeaderDrop>
        <HeaderDrop section="Collections">
            <Link href="/post/collection/explore">Explore</Link>
            <Link href="/post/collection/search">Search</Link>
            <Link href="/post/collection/new">Create</Link>
        </HeaderDrop>
        <HeaderDrop section="Users">
          <Link href="/user/explore">Explore</Link>
          <Link href="/user/search">Search</Link>
          {props.user !== null ? (
            <Link href={`/user/${props.user.id}`}>My Account</Link>
          ) : (
            <Link href="/user/login">Login</Link>
          )}
        </HeaderDrop>
      </section>
      <section>
        {props.user !== null && (
          <Link href="/post/new">
            <Image
              src="/icons/add.svg"
              alt="New"
              sizes="100%"
              width={0}
              height={0}
              className={style.add}
            />
          </Link>
        )}
        <ThemeButton />
        {props.user === null ? (
          <Link href="/user/login">Login</Link>
        ) : (
          <Link href={`/user/${props.user.id}`} className={style.user}>
            <Image
              src={props.user.avatar}
              alt="Profile Picture"
              sizes="100%"
              width={0}
              height={0}
              className={style.pfp}
            />
            <span>{props.user.username}</span>
          </Link>
        )}
      </section>
    </header>
  );
};

export default Header;

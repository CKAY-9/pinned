import Link from "next/link";
import style from "./footer.module.scss";
import { User } from "@/api/user/dto";
import Image from "next/image";

const Footer = (props: {
  user: User | null
}) => {
  return (
    <footer className={style.footer}>
      <section>
        <h1>Pinned</h1>
        <span>Made with ❤️ by <Link href="/team">the team</Link></span>
        <div className={style.links}>
          <Link href="https://github.com/CKAY-9/pinned">
            <Image src="/marks/github-mark-white.svg" alt="GitHub" sizes="100%" width={0} height={0} />
          </Link>
        </div>
      </section>
      <section>
        <strong>General</strong>
        <Link href="/">Home</Link> 
        <Link href="/about">About</Link> 
        <Link href="/team">Our Team</Link> 
      </section>
      <section>
        <strong>Posts</strong>
        <Link href="/post/explore">Explore</Link> 
        <Link href="/post/search">Search</Link> 
        {props.user !== null && <Link href="/post/new">New Post</Link>}
        <Link href="/post/collection/explore">Collections</Link> 
      </section>
      <section>
        <strong>Users</strong>
        <Link href="/user/search">Search</Link> 
        {props.user === null
          ? <Link href="/user/login">Login</Link>
          : <Link href={`/user/${props.user.id}`}>My Profile</Link>
        }
      </section>
    </footer>
  );
}

export default Footer;

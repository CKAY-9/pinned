import Link from "next/link";
import style from "./header.module.scss";
import { User } from "@/api/user/dto";
import Image from "next/image";

const Header = (props: {
  user: User | null
}) => {
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
        {props.user !== null &&
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
        }
      </section>
      <section>
        {props.user === null
          ? <Link href="/user/login">Login</Link>
          : <Link href={`/user/${props.user.id}`} className={style.user}>
            <Image src={props.user.avatar} alt="Profile Picture" sizes="100%" width={0} height={0} className={style.pfp} />
            <span>{props.user.username}</span>
          </Link>
        }
      </section>
    </header>
  );
}

export default Header;

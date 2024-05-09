import { getUserFromToken } from "@/api/user/user";
import Footer from "@/components/footer/footer";
import Header from "@/components/header/header";
import { Metadata } from "next";
import style from "./team.module.scss";
import Image from "next/image";

export const generateMetadata = (): Metadata => {
  return {
    "title": "Our Team // Pinned",
    "description": "Meet the team behind Pinned and how the teams works."
  }
}

const TeamMember = (props: {
  avatar: string,
  description: string,
  name: string
}) => {
  return (
    <div className={style.member}>
      <Image 
        src={props.avatar}
        alt="PFP"
        sizes="100%"
        width={0}
        height={0}
      />
      <h2>{props.name}</h2>
      <span>{props.description}</span>
    </div>
  );
}

const TeamPage = async () => {
  const user = await getUserFromToken();

  return (
    <>
      <Header user={user} />
      <main className="container">
        <h1 style={{"textAlign": "center"}}>About The Team</h1>
        <div className={style.team_members}>
          <TeamMember name="CKAY9" avatar="https://avatars.githubusercontent.com/u/53030585?v=4" description="Lead Programmer" />
          <TeamMember name="Hwvn" avatar="https://avatars.githubusercontent.com/u/116260123?v=4" description="UI/UX Designer" />
          <TeamMember name="Regrettinq" avatar="https://avatars.githubusercontent.com/u/160052254?v=4" description="Team Coordinator" />
        </div>
      </main>
      <Footer user={user} />
    </>
  ) 
}

export default TeamPage;

import { getUserFromToken } from "@/api/user/user";
import Footer from "@/components/footer/footer";
import Header from "@/components/header/header";
import { Metadata } from "next";

export const generateMetadata = (): Metadata => {
  return {
    "title": "Our Team // Pinned",
    "description": "Meet the team behind Pinned and how the teams works."
  }
}

const TeamPage = async () => {
  const user = await getUserFromToken();

  return (
    <>
      <Header user={user} />
      <main className="container">
        <h1>About The Team</h1>
        <span>Coming soon...</span>
      </main>
      <Footer user={user} />
    </>
  ) 
}

export default TeamPage;

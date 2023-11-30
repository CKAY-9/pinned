import { getUserFromToken } from "@/api/user/user";
import Footer from "@/components/footer/footer";
import Header from "@/components/header/header";
import { Metadata } from "next";

export const generateMetadata = (): Metadata => {
  return {
    "title": "About Pinned",
    "description": "Learn more about Pinned and how it operates."
  }
}

const AboutPage = async () => {
  const user = await getUserFromToken();

  return (
    <>
      <Header user={user} />
      <main className="container">
        <h1>About Pinned</h1>
        <span>Coming soon...</span>
      </main>
      <Footer user={user} />
    </>
  ) 
}

export default AboutPage;

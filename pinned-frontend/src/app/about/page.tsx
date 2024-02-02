import { getUserFromToken } from "@/api/user/user";
import Footer from "@/components/footer/footer";
import Header from "@/components/header/header";
import { Metadata } from "next";
import Link from "next/link";

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
      <main className="container" style={{"gap": "3rem"}}>
        <section>
          <h1>About Pinned</h1>
          <p>
            Pinned is a social platform meant for creators, creatives, and anyone who wants to share things that find interesting.
            Pinned is dedicated to making sure that posts shared on the platform are found via searching, algorithmic sorting, and general
            post finding. Users can upload images through posts, create collections, post comments, and vote on posts and collections.
          </p>
        </section>
        <section>
          <h2>Behind the Scenes</h2>
          <p>
            Pinned is comprised of three main services or components:
          </p>
          <ul>
            <li>pinned-frontend: The primary frontend or client-side interface for Pinned. Written in TypeScript using NextJS</li>
            <li>pinned-backend: The primary API and server-side service for Pinned. Written in Rust using Actix and Diesel.</li>
            <li>pinned-cdn: The primary CDN (content delivery network) for images and other static files for Pinned. Written in Python using Flask.</li>
          </ul>
          <p>
            The official repository for Pinned can be found on <Link href="https://github.com/CKAY-9/pinned">GitHub</Link>, along with licenses and more specfics on
            technical details of Pinned.
          </p>
        </section>
      </main>
      <Footer user={user} />
    </>
  ) 
}

export default AboutPage;

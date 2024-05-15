import { getUserFromToken } from "@/api/user/user";
import Footer from "@/components/footer/footer";
import Header from "@/components/header/header";
import { Metadata } from "next";
import FavouritesClient from "./client";
import { redirect } from "next/navigation";

export const generateMetadata = (): Metadata => {
    return {
        title: "My Favourite Posts // Pinned",
        description: "View your favourited posts on Pinned."
    }
}

const FavouritesPage = async () => {
    const user = await getUserFromToken();
    
    if (user === null) {
        redirect("/user/login");
    }

    return (
        <>
            <Header user={user}></Header>
            <main className="container">
                <h1>Favourited Posts</h1>
                <span>View all the posts that you thought were a little more special than most.</span>
                <FavouritesClient user={user} />
            </main>
            <Footer user={user}></Footer>
        </>
    );
}

export default FavouritesPage;
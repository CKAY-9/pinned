import { getUserFromToken } from "@/api/user/user";
import Footer from "@/components/footer/footer";
import Header from "@/components/header/header";
import { Metadata } from "next";

export const generateMetadata = (): Metadata => {
    return {
        title: "My Favourite Posts // Pinned",
        description: "View your favourited posts on Pinned."
    }
}

const FavouritesPage = async () => {
    const user = await getUserFromToken();
    
    return (
        <>
            <Header user={user}></Header>
            <main className="container">
                
            </main>
            <Footer user={user}></Footer>
        </>
    );
}

export default FavouritesPage;
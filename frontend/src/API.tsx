import { type ScoredUrls } from "./models/ScoredUrl";
import axios from "axios";

export async function Search(query: string): Promise<ScoredUrls | undefined> {
    let response = await axios.get(`${import.meta.env.VITE_BACKEND_URL}/search?s=${query}`)
    return response.data as ScoredUrls;
}
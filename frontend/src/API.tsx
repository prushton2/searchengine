import { type ScoredUrls } from "./models/ScoredUrl";
import axios from "axios";

export async function Search(query: string): Promise<ScoredUrls | undefined> {
    let response = await axios.get(`http://localhost:3333/search?s=${query}`)
    return response.data as ScoredUrls;
}
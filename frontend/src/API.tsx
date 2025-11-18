import { type ScoredUrls } from "./models/ScoredUrl";
import axios from "axios";

export async function Search(query: string): Promise<ScoredUrls | undefined> {
    let response = await axios.get(`/search?s=${query}&p=1`)
    return response.data as ScoredUrls;
}
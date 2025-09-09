import { type ScoredUrls } from "./models/ScoredUrl";
import axios from "axios";

export async function Search(query: string): Promise<ScoredUrls | undefined> {
    let response = await axios.get(`http://localhost:3333/search?s=${query}`)
    // console.log(response.data);
    // let object = response.data as ScoredUrls
    // console.log(object);
    // console.log(typeof object);
    return response.data as ScoredUrls;
    // return undefined
}
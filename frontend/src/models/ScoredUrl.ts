export interface ScoredUrls {
    urls: Map<string, number>
    metadata: Map<string, sitemetadata>
}

export interface sitemetadata {
    title: string
}
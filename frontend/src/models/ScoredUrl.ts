export interface ScoredUrls {
    words: Map<string, number>
    metadata: Map<string, sitemetadata>
}

export interface sitemetadata {
    title: string
    description: string
}
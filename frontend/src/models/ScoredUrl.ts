export interface ScoredUrls {
    words: Map<string, number>
    metadata: Map<string, sitemetadata>
    elapsedtime: number
}

export interface sitemetadata {
    title: string
    description: string
}
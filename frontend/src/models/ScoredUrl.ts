export interface ScoredUrls {
    url: string[]
    metadata: Map<string, sitemetadata>
    elapsedtime: number
}

export interface sitemetadata {
    title: string
    description: string
}
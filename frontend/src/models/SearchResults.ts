export interface SearchResult {
  id: string;
  title: string;
  description: string;
  url: string;
  domain: string;
  timestamp?: string;
}

export interface SearchResultsProps {
  isLoading: boolean;
  results: SearchResult[];
  searchTime?: number;
  totalResults?: number;
}
import { useState } from "react";
import { SearchBar } from "@/components/SearchBar";
import { SearchResults } from "@/components/SearchResults";
import { useToast } from "@/components/ui/use-toast";
import { Search } from "lucide-react";
import { Search as APISearch } from "../API.tsx"
import { type SearchResult as SearchResultType } from "@/models/SearchResults";
import { sitemetadata } from "@/models/ScoredUrl";

const Index = () => {
  const [searchResults, setSearchResults] = useState<any[]>([]);
  const [totalResults, setTotalResults] = useState<number>(0);
  const [isLoading, setIsLoading] = useState(false);
  const [hasSearched, setHasSearched] = useState(false);
  const [searchTime, setSearchTime] = useState<number>();
  const { toast } = useToast();

  const handleSearch = async (query: string) => {
    setIsLoading(true);
    setHasSearched(true);
    
    try {
      const searchResult = await APISearch(query);

      let results: SearchResultType[] = []
      
      searchResult.url.forEach((e, i) => {
        let metadata: sitemetadata = searchResult.metadata[e];
      
        results.push({
          id: `${i}`,
          title: metadata.title,
          description: metadata.description,
          url: e,
          domain: e
        })
      })

      setSearchResults(results);
      setTotalResults(searchResult.totalResults);
      setSearchTime(searchResult.elapsedtime / 1000);
      
      toast({
        title: "Search completed",
        description: `Found ${searchResult.totalResults} results for "${query}"`,
      });
    } catch (error) {
      toast({
        title: "Search failed",
        description: "Something went wrong. Please try again.",
        variant: "destructive",
      });
    } finally {
      setIsLoading(false);
    }
  };


  return (
    <div className="min-h-screen bg-background">
      {/* Hero Section */}
      <div className={`transition-all duration-500 ${hasSearched ? 'py-8' : 'py-20'}`}>
        <div className="container mx-auto px-4">
          {/* Logo/Brand */}
          <div className={`text-center mb-8 transition-all duration-500 ${hasSearched ? 'mb-6' : 'mb-10'}`}>
            <div className="flex items-center justify-center gap-3 mb-4">
              <Search className="h-10 w-10 text-primary" />
              <h1 className={`font-bold text-primary transition-all duration-500 ${
                hasSearched ? 'text-2xl' : 'text-4xl'
              }`}>
                Search
              </h1>
            </div>
            {!hasSearched && (
              <p className="text-base text-muted-foreground max-w-xl mx-auto">
                Privacy focused search that wont keep track of your searches
              </p>
            )}
          </div>

          {/* Search Bar */}
          <SearchBar 
            onSearch={handleSearch} 
            isLoading={isLoading}
            className={`transition-all duration-500 ${hasSearched ? 'max-w-2xl' : 'max-w-xl'}`}
          />
        </div>
      </div>

      {/* Search Results Section */}
      {hasSearched && (
        <div className="container mx-auto px-4 pb-12">
          <div className="max-w-3xl mx-auto">
            {/* Results */}
            <SearchResults
              isLoading={isLoading}
              results={searchResults}
              searchTime={searchTime}
              totalResults={totalResults}
            />
          </div>
        </div>
      )}
    </div>
  );
};

export default Index;
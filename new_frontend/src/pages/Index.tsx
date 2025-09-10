import { useState } from "react";
import { SearchBar } from "@/components/SearchBar";
import { SearchResults } from "@/components/SearchResults";
import { useToast } from "@/components/ui/use-toast";
import { Search } from "lucide-react";

// Mock search results for demonstration
const generateMockResults = (query: string) => [
  {
    id: "1",
    title: `Understanding ${query}: A comprehensive guide`,
    description: `Learn everything you need to know about ${query}. This comprehensive guide covers all the essential aspects, best practices, and practical applications you'll need to master this topic.`,
    url: `https://example.com/guide-to-${query.toLowerCase().replace(/\s+/g, '-')}`,
    domain: "example.com",
    timestamp: "2 hours ago"
  },
  {
    id: "2",
    title: `${query} - Latest News and Updates`,
    description: `Stay up to date with the latest developments in ${query}. Get breaking news, expert analysis, and insights from industry leaders covering recent trends and innovations.`,
    url: `https://news.example.com/${query.toLowerCase().replace(/\s+/g, '-')}-updates`,
    domain: "news.example.com",
    timestamp: "1 day ago"
  },
  {
    id: "3",
    title: `How to get started with ${query}`,
    description: `A beginner-friendly tutorial that walks you through the basics of ${query}. Perfect for newcomers looking to understand fundamental concepts and practical implementation strategies.`,
    url: `https://tutorial.example.com/getting-started-${query.toLowerCase().replace(/\s+/g, '-')}`,
    domain: "tutorial.example.com",
    timestamp: "3 days ago"
  },
  {
    id: "4",
    title: `${query}: Best Practices and Tips`,
    description: `Discover proven strategies and expert tips for working with ${query}. This article covers common pitfalls to avoid and optimization techniques used by professionals.`,
    url: `https://blog.example.com/${query.toLowerCase().replace(/\s+/g, '-')}-best-practices`,
    domain: "blog.example.com",
    timestamp: "1 week ago"
  },
  {
    id: "5",
    title: `Advanced ${query} Techniques`,
    description: `Take your ${query} skills to the next level with these advanced techniques and methodologies. Explore cutting-edge approaches used by experts in the field.`,
    url: `https://advanced.example.com/${query.toLowerCase().replace(/\s+/g, '-')}-techniques`,
    domain: "advanced.example.com",
    timestamp: "2 weeks ago"
  }
];

const Index = () => {
  const [searchResults, setSearchResults] = useState<any[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [hasSearched, setHasSearched] = useState(false);
  const [searchTime, setSearchTime] = useState<number>();
  const { toast } = useToast();

  const handleSearch = async (query: string) => {
    setIsLoading(true);
    setHasSearched(true);
    
    try {
      // Simulate search delay
      const startTime = Date.now();
      await new Promise(resolve => setTimeout(resolve, 800));
      const endTime = Date.now();
      
      const results = generateMockResults(query);
      setSearchResults(results);
      setSearchTime((endTime - startTime) / 1000);
      
      toast({
        title: "Search completed",
        description: `Found ${results.length} results for "${query}"`,
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
                DuckSearch
              </h1>
            </div>
            {!hasSearched && (
              <p className="text-base text-muted-foreground max-w-xl mx-auto">
                Privacy-focused search that doesn't track you. Find what you need without compromising your privacy.
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
              results={searchResults}
              searchTime={searchTime}
              totalResults={searchResults.length > 0 ? 1240000 : undefined}
            />
          </div>
        </div>
      )}
    </div>
  );
};

export default Index;
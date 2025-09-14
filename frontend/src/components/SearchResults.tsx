import { ExternalLink, Clock, Globe, Loader2 } from "lucide-react";
import { Card } from "@/components/ui/card";
import { SearchResultsProps } from "@/models/SearchResults";

export const SearchResults = ({ results, isLoading, searchTime, totalResults }: SearchResultsProps) => {
  if (isLoading) {
    return (
      <div className="text-center py-16">
        <Loader2 className="mx-auto h-16 w-16 text-primary mb-4 animate-spin" />
        <h3 className="text-xl font-medium text-foreground mb-2">Searching...</h3>
        <p className="text-muted-foreground">Finding the best results for you</p>
      </div>
    );
  }

  if (results.length === 0) {
    return (
      <div className="text-center py-16">
        <Globe className="mx-auto h-16 w-16 text-muted-foreground mb-4" />
        <h3 className="text-xl font-medium text-foreground mb-2">No results found</h3>
        <p className="text-muted-foreground">Try different keywords or check your spelling</p>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Search Stats */}
      <div className="text-sm text-muted-foreground border-b border-border pb-4">
        {totalResults && (
          <span>About {totalResults.toLocaleString()} results</span>
        )}
        {searchTime && (
          <span className="ml-2">({searchTime.toFixed(2)} seconds)</span>
        )}
      </div>

      {/* Results */}
      <div className="space-y-6">
        {results.map((result) => (
          <Card
            key={result.id}
            className="p-5 bg-card hover:bg-accent/30 transition-all duration-200 cursor-pointer group border border-border hover:border-primary/30"
          >
            <a
              href={result.url}
              target="_blank"
              rel="noopener noreferrer"
              className="block"
            >
              <div className="flex items-start justify-between gap-4">
                <div className="flex-1 min-w-0">
                  {/* URL and Domain */}
                  <div className="flex items-center gap-2 mb-2">
                    <Globe className="h-4 w-4 text-muted-foreground flex-shrink-0" />
                    <span className="text-sm text-primary hover:underline truncate">
                      {result.domain}
                    </span>
                    {result.timestamp && (
                      <div className="flex items-center gap-1 text-xs text-muted-foreground">
                        <Clock className="h-3 w-3" />
                        <span>{result.timestamp}</span>
                      </div>
                    )}
                  </div>

                  {/* Title */}
                  <h2 className="text-xl font-medium text-primary hover:underline mb-2 line-clamp-2">
                    {result.title}
                  </h2>

                  {/* Description */}
                  <p className="text-sm text-muted-foreground line-clamp-3 leading-relaxed">
                    {result.description}
                  </p>
                </div>

                <ExternalLink className="h-4 w-4 text-muted-foreground opacity-0 group-hover:opacity-100 transition-opacity flex-shrink-0" />
              </div>
            </a>
          </Card>
        ))}
      </div>
    </div>
  );
};
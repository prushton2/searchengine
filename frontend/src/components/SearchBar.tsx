import { useState } from "react";
import { Search, Mic, Camera } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { cn } from "@/lib/utils";

interface SearchBarProps {
  onSearch: (query: string) => void;
  isLoading?: boolean;
  className?: string;
}

export const SearchBar = ({ onSearch, isLoading, className }: SearchBarProps) => {
  const [query, setQuery] = useState("");

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (query.trim()) {
      onSearch(query.trim());
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter") {
      handleSubmit(e);
    }
  };

  return (
    <div className={cn("w-full max-w-2xl mx-auto", className)}>
      <form onSubmit={handleSubmit} className="relative">
        <div className="relative flex items-center bg-card rounded-lg border border-border hover:border-primary/50 transition-all duration-300 focus-within:border-primary focus-within:ring-2 focus-within:ring-primary/20">
          <Search className="absolute left-4 h-5 w-5 text-muted-foreground" />
          <Input
            type="text"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="Search the web..."
            className="flex-1 border-0 bg-transparent pl-12 pr-20 py-4 text-base placeholder:text-muted-foreground focus-visible:ring-0 focus-visible:ring-offset-0"
            disabled={isLoading}
          />
          <div className="absolute right-2 flex items-center gap-1">
            <Button
              type="button"
              variant="ghost"
              size="sm"
              className="h-8 w-8 p-0 hover:bg-accent rounded-full"
            >
              <Mic className="h-4 w-4" />
            </Button>
            <Button
              type="button"
              variant="ghost"
              size="sm"
              className="h-8 w-8 p-0 hover:bg-accent rounded-full"
            >
              <Camera className="h-4 w-4" />
            </Button>
          </div>
        </div>
      </form>
      
      <div className="flex justify-center mt-6">
        <Button
          type="submit"
          onClick={handleSubmit}
          disabled={isLoading || !query.trim()}
          className="bg-primary hover:bg-primary/90 text-primary-foreground px-8 py-2 transition-all duration-200"
        >
          {isLoading ? "Searching..." : "Search"}
        </Button>
      </div>
    </div>
  );
};
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Clock, Image, Video, FileText, MapPin, MoreHorizontal } from "lucide-react";

interface SearchFiltersProps {
  activeFilter: string;
  onFilterChange: (filter: string) => void;
}

const filters = [
  { id: "all", label: "All", icon: null },
  { id: "images", label: "Images", icon: Image },
  { id: "videos", label: "Videos", icon: Video },
  { id: "news", label: "News", icon: FileText },
  { id: "maps", label: "Maps", icon: MapPin },
  { id: "recent", label: "Recent", icon: Clock },
];

export const SearchFilters = ({ activeFilter, onFilterChange }: SearchFiltersProps) => {
  return (
    <div className="border-b border-border pb-4 mb-6">
      <div className="flex items-center gap-2 overflow-x-auto scrollbar-hide">
        {filters.map((filter) => {
          const Icon = filter.icon;
          const isActive = activeFilter === filter.id;
          
          return (
            <Button
              key={filter.id}
              variant={isActive ? "default" : "ghost"}
              size="sm"
              onClick={() => onFilterChange(filter.id)}
              className={`flex items-center gap-2 whitespace-nowrap transition-all duration-200 ${
                isActive 
                  ? "bg-primary text-primary-foreground shadow-sm" 
                  : "hover:bg-accent"
              }`}
            >
              {Icon && <Icon className="h-4 w-4" />}
              {filter.label}
              {isActive && (
                <Badge variant="secondary" className="ml-1 text-xs bg-primary-foreground/20">
                  Active
                </Badge>
              )}
            </Button>
          );
        })}
        
        <Button
          variant="ghost"
          size="sm"
          className="flex items-center gap-2 whitespace-nowrap hover:bg-accent"
        >
          <MoreHorizontal className="h-4 w-4" />
          More
        </Button>
      </div>
    </div>
  );
};
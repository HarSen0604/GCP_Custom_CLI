"use client"

import { Button } from "@/components/ui/button"
import { Menu, Share2, Download, Trash2, MoreVertical, Terminal } from "lucide-react"
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from "@/components/ui/dropdown-menu"
import { ModeToggle } from "@/components/mode-toggle"

interface ChatHeaderProps {
  title: string
  onMenuClick: () => void
  onClearChat: () => void
}

export function ChatHeader({ title, onMenuClick, onClearChat }: ChatHeaderProps) {
  return (
    <header className="sticky top-0 z-10 flex items-center justify-between border-b bg-background p-4">
      <div className="flex items-center gap-3">
        <Button variant="ghost" size="icon" onClick={onMenuClick} className="lg:hidden">
          <Menu className="h-5 w-5" />
        </Button>
        <h1 className="text-lg font-semibold truncate max-w-[200px] sm:max-w-md flex items-center gap-2">
          <Terminal className="h-5 w-5" />
          {title}
        </h1>
      </div>

      <div className="flex items-center gap-2">
        <ModeToggle />

        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button variant="ghost" size="icon">
              <MoreVertical className="h-5 w-5" />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            <DropdownMenuItem>
              <Share2 className="mr-2 h-4 w-4" />
              <span>Share command</span>
            </DropdownMenuItem>
            <DropdownMenuItem>
              <Download className="mr-2 h-4 w-4" />
              <span>Export history</span>
            </DropdownMenuItem>
            <DropdownMenuItem onClick={onClearChat}>
              <Trash2 className="mr-2 h-4 w-4" />
              <span>Clear conversation</span>
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
    </header>
  )
}


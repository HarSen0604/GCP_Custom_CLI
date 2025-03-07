"use client"

import { useState } from "react"
import { Button } from "@/components/ui/button"
import { ScrollArea } from "@/components/ui/scroll-area"
import { cn } from "@/lib/utils"
import type { Conversation } from "@/types"
import { PlusCircle, Trash2, X, Settings, Moon, Sun, Monitor, Terminal } from "lucide-react"
import { useTheme } from "next-themes"
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from "@/components/ui/dropdown-menu"

interface SidebarProps {
  conversations: Conversation[]
  currentConversationId: string
  onSelectConversation: (id: string) => void
  onNewConversation: () => void
  onDeleteConversation: (id: string) => void
  isOpen: boolean
  onOpenChange: (open: boolean) => void
}

export function Sidebar({
  conversations,
  currentConversationId,
  onSelectConversation,
  onNewConversation,
  onDeleteConversation,
  isOpen,
  onOpenChange,
}: SidebarProps) {
  const { setTheme } = useTheme()
  const [hoveredConversation, setHoveredConversation] = useState<string | null>(null)

  return (
    <>
      {/* Overlay for mobile */}
      {isOpen && (
        <div
          className="fixed inset-0 z-40 bg-background/80 backdrop-blur-sm lg:hidden"
          onClick={() => onOpenChange(false)}
        />
      )}

      {/* Sidebar */}
      <div
        className={cn(
          "fixed inset-y-0 left-0 z-50 flex w-80 flex-col border-r bg-card transition-transform duration-300 ease-in-out lg:z-0",
          isOpen ? "translate-x-0" : "-translate-x-full lg:translate-x-0 lg:w-0",
        )}
      >
        <div className="flex items-center justify-between p-4 border-b">
          <h2 className="text-lg font-semibold flex items-center gap-2">
            <Terminal className="h-5 w-5" />
            GCloud Generator
          </h2>
          <Button variant="ghost" size="icon" onClick={() => onOpenChange(false)} className="lg:hidden">
            <X className="h-5 w-5" />
          </Button>
        </div>

        <div className="p-4">
          <Button className="w-full justify-start gap-2" onClick={onNewConversation}>
            <PlusCircle className="h-5 w-5" />
            New command
          </Button>
        </div>

        <ScrollArea className="flex-1 px-4">
          <div className="space-y-2 py-2">
            {conversations.map((conversation) => (
              <div
                key={conversation.id}
                className="relative group"
                onMouseEnter={() => setHoveredConversation(conversation.id)}
                onMouseLeave={() => setHoveredConversation(null)}
              >
                <Button
                  variant={conversation.id === currentConversationId ? "secondary" : "ghost"}
                  className="w-full justify-start gap-2 text-left overflow-hidden"
                  onClick={() => onSelectConversation(conversation.id)}
                >
                  <Terminal className="h-5 w-5 shrink-0" />
                  <span className="truncate">{conversation.title}</span>
                </Button>

                {(hoveredConversation === conversation.id || conversation.id === currentConversationId) && (
                  <Button
                    variant="ghost"
                    size="icon"
                    className="absolute right-2 top-1/2 -translate-y-1/2 opacity-70 hover:opacity-100"
                    onClick={(e) => {
                      e.stopPropagation()
                      onDeleteConversation(conversation.id)
                    }}
                  >
                    <Trash2 className="h-4 w-4" />
                  </Button>
                )}
              </div>
            ))}
          </div>
        </ScrollArea>

        <div className="p-4 border-t">
          <div className="flex items-center justify-between">
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <Button variant="outline" size="icon">
                  <Settings className="h-5 w-5" />
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="start">
                <DropdownMenuItem onClick={() => setTheme("light")}>
                  <Sun className="mr-2 h-4 w-4" />
                  <span>Light</span>
                </DropdownMenuItem>
                <DropdownMenuItem onClick={() => setTheme("dark")}>
                  <Moon className="mr-2 h-4 w-4" />
                  <span>Dark</span>
                </DropdownMenuItem>
                <DropdownMenuItem onClick={() => setTheme("system")}>
                  <Monitor className="mr-2 h-4 w-4" />
                  <span>System</span>
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>

            <div className="text-sm text-muted-foreground">
              <span>v1.0.0</span>
            </div>
          </div>
        </div>
      </div>
    </>
  )
}


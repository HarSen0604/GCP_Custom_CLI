"use client"

import { Button } from "@/components/ui/button"
import { Menu, PlusCircle, Terminal } from "lucide-react"

interface MobileNavProps {
  isOpen: boolean
  onOpenChange: (open: boolean) => void
  onNewChat: () => void
}

export function MobileNav({ isOpen, onOpenChange, onNewChat }: MobileNavProps) {
  return (
    <div className="fixed top-0 left-0 right-0 z-30 flex items-center justify-between p-4 border-b bg-background lg:hidden">
      <Button variant="ghost" size="icon" onClick={() => onOpenChange(!isOpen)}>
        <Menu className="h-5 w-5" />
      </Button>

      <h1 className="text-lg font-semibold flex items-center gap-2">
        <Terminal className="h-5 w-5" />
        GCloud Generator
      </h1>

      <Button variant="ghost" size="icon" onClick={onNewChat}>
        <PlusCircle className="h-5 w-5" />
      </Button>
    </div>
  )
}

